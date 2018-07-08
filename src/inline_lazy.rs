// Copyright 2016 lazy-static.rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate core;
extern crate std;

use self::std::prelude::v1::*;
use self::std::sync::Once;
pub use self::std::sync::ONCE_INIT;

pub struct Lazy<T: Sync>(Option<T>, Once);

impl<T: Sync> Lazy<T> {
    pub const INIT: Self = Lazy(None, ONCE_INIT);

    #[inline(always)]
    pub fn get<F>(&'static mut self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        {
            let r = &mut self.0;
            self.1.call_once(|| {
                *r = Some(f());
            });
        }
        unsafe {
            match self.0 {
                Some(ref x) => x,
                None => unreachable_unchecked(),
            }
        }
    }
}

unsafe impl<T: Sync> Sync for Lazy<T> {}

#[macro_export]
#[doc(hidden)]
macro_rules! __lazy_static_create {
    ($NAME:ident, $T:ty) => {
        static mut $NAME: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy::INIT;
    };
}

#[cfg(lazy_static_core_unreachable_unchecked)]
use core::hint::unreachable_unchecked;

#[cfg(not(lazy_static_core_unreachable_unchecked))]
/// Polyfill for core::hint::unreachable_unchecked. Included to support Rust prior to 1.27. See
/// [issue #102](https://github.com/rust-lang-nursery/lazy-static.rs/issues/102#issuecomment-400959779)
/// for details.
unsafe fn unreachable_unchecked() -> ! {
    enum Void {}
    match std::mem::uninitialized::<Void>() {}
}
