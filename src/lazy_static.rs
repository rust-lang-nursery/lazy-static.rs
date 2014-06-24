/*!
Macro to declare lazily evaluated statics, which allows runtime initialization.

The macro works by defining a custom type with a `Deref` impl, which delegates the dereferencing
to a hidden `static mut` that gets lazily initialized on first access.

# Examples

To load the extension and use it:

```rust,ignore

}
```
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
                        static mut s: *$T = 0 as *$T;
                        static mut ONCE: Once = ONCE_INIT;
                        ONCE.doit(|| {
                            s = transmute::<Box<$T>, *$T>(box() ($e));
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
