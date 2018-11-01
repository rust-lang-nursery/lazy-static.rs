/*
This library is a shim around `lazy_static` that disambiguates it with the `lazy_static`
that's shipped with the Rust toolchain. We re-export the entire public API of `lazy_static`
under a different crate name so that can be imported in the compile tests.

This currently appears to use the right local build of `lazy_static`.
*/

extern crate lazy_static;

pub use self::lazy_static::*;
