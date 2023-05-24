//use example_tveir::tst::intellisense_or_nei;

use example::slib::{
    Input,
    test_func,
};

fn main() {
    let input = Input {
        value: String::from("Hello, world!"),
    };
    test_func(input);
    // So this still lacks intellisense for dependencies from components outside 
    // the current workspace. Which makes sense given the layout, but causes obvious problem. I wonder
    // now if the solution is to open it at the area where the two are combined. Let's see.
    intellisense_or_nei("octagon".to_string());
    intellisense_or_nei(122);
}