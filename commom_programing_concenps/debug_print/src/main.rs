
use nightly_crimes::nightly_crimes;

nightly_crimes! {
    #![feature(print_internals)]
    use std::io::_print;
}

fn main() {
    print!("Hello, print!");
    _print("Hello, print");
}
