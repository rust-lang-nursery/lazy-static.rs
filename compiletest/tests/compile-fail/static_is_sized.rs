// error-pattern:the size for values of type `str` cannot be known at compilation time
#[macro_use]
extern crate lazy_static_compiletest as lazy_static;

lazy_static! {
    pub static ref FOO: str = panic!();
}


fn main() {
}
