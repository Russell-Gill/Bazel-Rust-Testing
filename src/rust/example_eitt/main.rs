extern crate example_tveir;

use example::slib::test_func;

use crate::example_eitt::slib::{
    Input,
    test_func,
};

use crate::example_tveir::tst;

fn main() {
    let input = Input {
        value: String::from("Hello, world!"),
    };
    test_func(input);
    tst::intellisense_or_nei("octagon".to_string());
    tst::intellisense_or_nei(122);
}