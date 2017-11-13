#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref FOO: str = panic!();
}
//^ ERROR `str` does not have a constant size known at compile-time

fn main() {
}
