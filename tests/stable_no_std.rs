#![cfg(feature = "stable_no_std")]
#![no_std]

#[macro_use]
extern crate lazy_static;

lazy_static! {
    /// Documentation + attribute!
    #[doc(hidden)]
    pub static ref NUMBER: u128 = long_computation(20);
}

fn long_computation(n: u128) -> u128 {
    let mut acc = 1;

    for n in 2 ..= n {
        acc *= n;
    }

    acc
}

#[test]
fn test_basic() {
    assert_eq!(*NUMBER, 2432902008176640000);
}
