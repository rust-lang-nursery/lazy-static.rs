/*!
A macro for declaring lazily evaluated statics.

Using this macro, it is possible to have `static`s that require code to be
executed at runtime in order to be initialized.
This includes anything requiring heap allocations, like vectors or hash maps,
as well as anything that requires function calls to be computed.

# Syntax

```ignore
lazy_static! {
    [pub] static ref NAME_1: TYPE_1 = EXPR_1;
    [pub] static ref NAME_2: TYPE_2 = EXPR_2;
    ...
    [pub] static ref NAME_N: TYPE_N = EXPR_N;
}
```

Metadata (such as doc comments) is allowed on each ref.

# Semantic

For a given `static ref NAME: TYPE = EXPR;`, the macro generates a unique type that
implements `Deref<TYPE>` and stores it in a static with name `NAME`. (Metadata ends up
attaching to this type.)

On first deref, `EXPR` gets evaluated and stored internally, such that all further derefs
can return a reference to the same object.

Like regular `static mut`s, this macro only works for types that fulfill the `Sync`
trait.

# Example

Using the macro:

```
# #![cfg_attr(feature="nightly", feature(const_fn, core_intrinsics))]
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
```

# Implementation details

The `Deref` implementation uses a hidden static variable that is guarded by a atomic check on each access. On stable Rust, the macro may need to allocate each static on the heap.

*/

#![cfg_attr(feature="nightly", feature(const_fn, core_intrinsics))]
#![crate_type = "dylib"]

pub mod lazy;

#[cfg(feature="nightly")]
#[macro_export]
macro_rules! lazy_create {
    ($T:ty) => {
        static LAZY: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy::new();
    }
}

#[cfg(not(feature="nightly"))]
#[macro_export]
macro_rules! lazy_create {
    ($T:ty) => {
        use std::sync::ONCE_INIT;
        static mut LAZY: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy(0 as *const $T, ONCE_INIT);
    }
}

#[macro_export]
macro_rules! lazy_static {
    ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PRIV, $(#[$attr])* static ref $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PUB, $(#[$attr])* static ref $N : $T = $e; $($t)*);
    };
    ($VIS:ident, $(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(MAKE TY, $VIS, $(#[$attr])*, $N);
        impl ::std::ops::Deref for $N {
            type Target = $T;
            fn deref<'a>(&'a self) -> &'a $T {
                unsafe {
                    #[inline(always)]
                    fn __static_ref_initialize() -> $T { $e }

                    unsafe fn __stability() -> &'static $T {
                        lazy_create!($T);
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
        }
        lazy_static!($($t)*);
    };
    (MAKE TY, PUB, $(#[$attr:meta])*, $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        pub struct $N {__private_field: ()}
        #[doc(hidden)]
        pub static $N: $N = $N {__private_field: ()};
    };
    (MAKE TY, PRIV, $(#[$attr:meta])*, $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        struct $N {__private_field: ()}
        #[doc(hidden)]
        static $N: $N = $N {__private_field: ()};
    };
    () => ()
}
