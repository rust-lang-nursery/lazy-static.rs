// Copyright 2016 lazy-static.rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate core;
extern crate std;

use self::std::prelude::v1::*;
use self::std::cell::Cell;
use self::std::hint::unreachable_unchecked;
use self::std::ptr::null;
use self::std::sync::atomic::AtomicPtr;
use self::std::sync::atomic::Ordering::SeqCst;
use self::std::sync::Once;
#[allow(deprecated)]
pub use self::std::sync::ONCE_INIT;

macro_rules! uninitialized {
    ( $x:expr ) => {{
        debug_assert!(
            false,
            concat!(
                "attempted to ",
                $x,
                " an uninitialized lazy static. This is a bug"
            )
        );
        unreachable_unchecked()
    }};
}

trait Resettable {
    fn reset(&self);
}

struct Node(*const Node, &'static dyn Resettable);

// FIXME: Replace Option<T> with MaybeUninit<T> (stable since 1.36.0)
pub struct Lazy<T: Sync>(Cell<Option<T>>, Cell<Once>);

static INITED: AtomicPtr<Node> = AtomicPtr::new(null::<Node>() as *mut Node);

/// Puts all initialized `lazy_static` variables back in their
/// uninitialized states. The caller is responsible for ensuring that
/// there are no outstanding references to those variables. Similarly,
/// the caller must not try to initialize a `lazy_static` variable
/// concurrently with a call to `reset`, as this presents a data race.
pub unsafe fn reset() {
    let mut node_ptr = INITED.swap(null::<Node>() as *mut Node, SeqCst) as *const Node;
    while node_ptr != null() {
        let node_box = Box::from_raw(node_ptr as *mut Node);
        node_ptr = (*node_box).0;
        (*node_box).1.reset();
    }
}

#[inline(always)]
unsafe fn lysis<T>(cell: &Cell<T>) -> &T {
    &*cell.as_ptr()
}

impl<T: Sync> Lazy<T> {
    #[allow(deprecated)]
    pub const INIT: Self = Lazy(Cell::new(None), Cell::new(ONCE_INIT));

    #[inline(always)]
    pub fn get<F>(&'static self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        unsafe { lysis(&self.1) }.call_once(|| {
            self.0.set(Some(f()));
            let node_box = Box::new(Node(INITED.load(SeqCst), self));
            let node_ptr = Box::into_raw(node_box);
            loop {
                let prev_ptr = INITED.compare_and_swap(
                    unsafe { (*node_ptr).0 } as *mut Node,
                    node_ptr,
                    SeqCst,
                ) as *const Node;
                if prev_ptr == unsafe { (*node_ptr).0 } {
                    break;
                }
                unsafe {
                    (*node_ptr).0 = prev_ptr;
                }
            }
        });

        // `self.0` is guaranteed to be `Some` by this point
        // The `Once` will catch and propagate panics
        match *unsafe { lysis(&self.0) } {
            Some(ref x) => x,
            None => unsafe { uninitialized!("dereference") },
        }
    }
}

impl<T: Sync> Resettable for Lazy<T> {
    fn reset(&self) {
        match *unsafe { lysis(&self.0) } {
            Some(_) => {
                self.0.set(None);
                self.1.set(ONCE_INIT);
            }
            None => unsafe { uninitialized!("reset") },
        }
    }
}

unsafe impl<T: Sync> Sync for Lazy<T> {}

#[macro_export]
#[doc(hidden)]
macro_rules! __lazy_static_create {
    ($NAME:ident, $T:ty) => {
        static $NAME: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy::INIT;
    };
}
