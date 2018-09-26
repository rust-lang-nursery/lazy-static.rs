#![cfg(lazy_static_rw_spin_impl)]

#[macro_use]
extern crate lazy_static;

use std::thread;

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
fn test_threads() {
    let mut threads = Vec::new();

    for _ in 0 .. 8 {
        threads.push(thread::spawn(|| assert_eq!(*NUMBER, 2432902008176640000)))
    }

    for thread in threads {
        thread.join().unwrap()
    }
}
