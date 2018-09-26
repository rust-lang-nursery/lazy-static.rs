// Copyright 2016 lazy-static.rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate std;

use self::std::prelude::v1::*;
use self::std::ptr;
use self::std::sync::Once;
pub use self::std::sync::ONCE_INIT;

pub struct Lazy<T: Sync>(*const T, Once);

impl<T: Sync> Lazy<T> {
    pub const INIT: Self = Lazy(0 as *const T, ONCE_INIT);

    #[inline(always)]
    pub fn get<F>(&'static self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.1.call_once(|| {
            let r = Box::into_raw(Box::new(f()));
            
            // This is safe because the `Once` guarantees we
            // have exclusive access to the value field
            // The value hasn't been previously initialized
            unsafe {
                ptr::write(&self.0 as *const _ as *mut _, r);
            }
        });

        // `self.0` is guaranteed to have a value by this point
        // The `Once` will catch and propegate panics
        unsafe { &*self.0 }
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
