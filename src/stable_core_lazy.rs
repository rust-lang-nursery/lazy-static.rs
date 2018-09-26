// Copyright 2016 lazy-static.rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_export]
#[doc(hidden)]
macro_rules! __lazy_static_create {
    ($NAME:ident, $T:ty) => {
        static mut $NAME: $NAME = $NAME {
            align: [],
            _bytes: [0; size_of::<$T>()],
            status: AtomicUsize::new(UNINIT),
        };

        #[cfg(feature = "stable_no_std")]
        use core::{
            cell::UnsafeCell,
            mem::size_of,
            sync::atomic::{AtomicUsize, Ordering::*},
        };

        #[cfg(not(feature = "stable_no_std"))]
        use std::{
            cell::UnsafeCell,
            mem::size_of,
            sync::atomic::{AtomicUsize, Ordering::*},
        };

        const UNINIT: usize = 0;
        const INITING: usize = 1;
        const INITED: usize = 2;

        #[allow(non_camel_case_types)]
        #[repr(C)]
        struct $NAME {
            status: AtomicUsize,
            align: [$T; 0],
            _bytes: [u8; size_of::<$T>()],
        }

        impl $NAME {
            #[inline(always)]
            fn get<F>(&'static mut self, init: F) -> &$T
            where
                F: FnOnce() -> $T,
            {
                let status =
                    self.status.compare_and_swap(UNINIT, INITING, Acquire);

                match status {
                    UNINIT => {
                        unsafe { self.align.as_mut_ptr().write(init()) }
                        self.status.store(INITED, Release);
                    },

                    INITING => while self.status.load(Acquire) == INITING {},

                    INITED | _ => (),
                }

                unsafe { &*self.align.as_ptr() }
            }
        }

        unsafe impl Sync for $NAME where $T: Sync {}
    };
}
