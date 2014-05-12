#![feature(default_type_parameters)]

use calc::eval;

mod calc;

// Test evaluation of expressions

// Invalid expressions
#[test]
fn check_invalid() {
    // Vector of invalid expressions
    
    let invalid = ["", "()", "(+)", "(2)", "2",
        "(+ + 2)", ")+ 2)", "@", "(+ 2 (+2 3)"];
    assert!(invalid.iter().map(|s|, eval(s)).all(|x| x.is_err()));
}