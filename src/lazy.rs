use std::sync::Once;

#[cfg(not(feature="nightly"))]
use std::mem::transmute;
#[cfg(feature="nightly")]
use std::cell::UnsafeCell;
#[cfg(feature="nightly")]
use std::sync::ONCE_INIT;

#[cfg(feature="nightly")]
pub struct Lazy<T: Sync>(UnsafeCell<Option<T>>, Once);

#[cfg(not(feature="nightly"))]
pub struct Lazy<T: Sync>(pub *const T, pub Once);

#[cfg(feature="nightly")]
impl<T: Sync> Lazy<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Lazy(UnsafeCell::new(None), ONCE_INIT)
    }

    #[inline(always)]
    pub fn get<F>(&'static self, f: F) -> &T
        where F: FnOnce() -> T
    {
        unsafe {
            self.1.call_once(|| {
                *self.0.get() = Some(f());
            });

            match *self.0.get() {
                Some(ref x) => x,
                None => ::std::intrinsics::unreachable(),
            }
        }
    }
}

#[cfg(not(feature="nightly"))]
impl<T: Sync> Lazy<T> {
    #[inline(always)]
    pub fn get<F>(&'static mut self, f: F) -> &T
        where F: FnOnce() -> T
    {
        unsafe {
            let r = &mut self.0;
            self.1.call_once(|| {
                *r = transmute(Box::new(f()));
            });

            &*self.0
        }
    }
}

unsafe impl<T: Sync> Sync for Lazy<T> {}
