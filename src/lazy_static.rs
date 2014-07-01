/*!
A macro for declaring lazily evaluated statics.

Using this macro, it is possible to have `static`s that require code to be
executed at runtime in order to be initialized.
This includes anything requiring heap allocations, like vectors or hash maps,
as well as anything that requires function calls to be computed.

# Syntax

```rust
lazy_static! {
    static ref NAME_1: TYPE_1 = EXPR_1;
    static ref NAME_2: TYPE_2 = EXPR_2;
    ...
    static ref NAME_N: TYPE_N = EXPR_N;
}
```

# Semantic

For a given `static ref NAME: TYPE = EXPR;`, the macro generates a
unique type that implements `Deref<TYPE>` and stores it in a static with name `NAME`.

On first deref, `EXPR` gets evaluated and stored internally, such that all further derefs
can return a reference to the same object.

Like regular `static mut`s, this macro only works for types that fulfill the `Share`
trait.

# Example

Using the macro:

```rust
#![feature(phase)]

#[phase(plugin)]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<uint, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0u, "foo");
        m.insert(1u, "bar");
        m.insert(2u, "baz");
        m
    };
    static ref COUNT: uint = HASHMAP.len();
    static ref NUMBER: uint = times_two(21);
}

fn times_two(n: uint) -> uint { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0));
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
```

# Implementation details

The `Deref` implementation uses a hidden `static mut` that is guarded by a atomic check
using the `sync::Once` abstraction. All lazily evaluated values are currently
put in a heap allocated box, due to the Rust language currently not providing any way to
define uninitialized `static mut` values.

*/

#![crate_id = "lazy_static"]
#![crate_type = "dylib"]
#![license = "MIT"]

#![feature(macro_rules)]

#[macro_export]
macro_rules! lazy_static {
    ($(static ref $N:ident : $T:ty = $e:expr;)*) => {
        $(
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct $N {__unit__: ()}
            static $N: $N = $N {__unit__: ()};
            impl Deref<$T> for $N {
                fn deref<'a>(&'a self) -> &'a $T {
                    use std::sync::{Once, ONCE_INIT};
                    use std::mem::transmute;

                    #[inline(always)]
                    fn require_share<T: Share>(_: &T) { }

                    unsafe {
                        static mut s: *const $T = 0 as *const $T;
                        static mut ONCE: Once = ONCE_INIT;
                        ONCE.doit(|| {
                            s = transmute::<Box<$T>, *const $T>(box() ($e));
                        });
                        let static_ref = &*s;
                        require_share(static_ref);
                        static_ref
                    }
                }
            }

        )*
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    lazy_static! {
        static ref NUMBER: uint = times_two(3);
        static ref VEC: [Box<uint>, ..3] = [box 1, box 2, box 3];
        static ref OWNED_STRING: String = "hello".to_string();
        static ref HASHMAP: HashMap<uint, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0u, "abc");
            m.insert(1, "def");
            m.insert(2, "ghi");
            m
        };
        static ref UNUSED: () = ();
    }

    fn times_two(n: uint) -> uint {
        n * 2
    }

    #[test]
    fn test_basic() {
        assert_eq!(OWNED_STRING.as_slice(), "hello");
        assert_eq!(*NUMBER, 6);
        assert!(HASHMAP.find(&1).is_some());
        assert!(HASHMAP.find(&3).is_none());
        assert_eq!(VEC.as_slice(), &[box 1, box 2, box 3]);
    }

    #[test]
    fn test_repeat() {
        assert_eq!(*NUMBER, 6);
        assert_eq!(*NUMBER, 6);
        assert_eq!(*NUMBER, 6);
    }
}
