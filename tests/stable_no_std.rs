#![cfg(feature = "stable_no_std")]
#![no_std]

#[macro_use]
extern crate lazy_static;

lazy_static! {
    /// Documentation + attribute!
    #[doc(hidden)]
    pub static ref NUMBER: u64 = long_computation(10);
}

fn long_computation(n: u64) -> u64 {
    let mut acc = 1;

    for n in 2 .. n + 1 {
        acc *= n;
    }

    acc
}

#[test]
fn test_basic() {
    assert_eq!(*NUMBER, 3628800);
}
