#![cfg(lazy_static_rw_spin_impl)]

#[macro_use]
extern crate lazy_static;

use std::thread;

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
fn test_threads() {
    let mut threads = Vec::new();

    for _ in 0 .. 8 {
        threads.push(thread::spawn(|| assert_eq!(*NUMBER, 3628800)))
    }

    for thread in threads {
        thread.join().unwrap()
    }
}
