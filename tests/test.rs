#![allow(unstable)]

#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBER: u32 = times_two(3);
    static ref ARRAY_BOXES: [Box<u32>; 3] = [Box::new(1), Box::new(2), Box::new(3)];
    static ref STRING: String = "hello".to_string();
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "abc");
        m.insert(1, "def");
        m.insert(2, "ghi");
        m
    };
    // This *should* triggger warn(dead_code) by design.
    static ref UNUSED: () = ();
}

fn times_two(n: u32) -> u32 {
    n * 2
}

#[test]
fn test_basic() {
    assert_eq!(&STRING[..], "hello");
    assert_eq!(*NUMBER, 6);
    assert!(HASHMAP.get(&1).is_some());
    assert!(HASHMAP.get(&3).is_none());
    assert_eq!(&ARRAY_BOXES[..], &[Box::new(1), Box::new(2), Box::new(3)][..]);
}

#[test]
fn test_repeat() {
    assert_eq!(*NUMBER, 6);
    assert_eq!(*NUMBER, 6);
    assert_eq!(*NUMBER, 6);
}

mod visibility {
    lazy_static! {
        pub static ref FOO: Box<u32> = Box::new(0);
    }
}

#[test]
fn test_visibility() {
    assert_eq!(*visibility::FOO, Box::new(0));
}

// This should not cause a warning about a missing Copy implementation
lazy_static! {
    pub static ref VAR: i32 = { 0 };
}
