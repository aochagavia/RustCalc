// We need this feature for CalcResult
#![feature(default_type_params)]

#[cfg(not(test))]
use std::io;
use calc::eval;

mod calc;

#[cfg(not(test))]
fn main() {
    let mut reader = io::stdin();
    
    for line in reader.lines().map(|l| l.unwrap()) {
        match eval(line) {
            Err(msg) => println!("Error: {}", msg),
            Ok(result) => println!("Result: {}", result)
        }
    }
}

// Tests

#[test]
fn check_invalid() {
    // Slice of invalid expressions
    let invalid = ["", "()", "(-)", "(2)", "2",
        "(+ + 2)", ")+ 2)", "@", "(+ 2 (+ 2 3)",
        "(+ aaaa 12)"];
        
    assert!(invalid.iter().map(|&s| eval(s)).all(|x| x.is_err()));
}

#[test]
fn check_valid() {
    assert!(eval("(+ 2 5)").unwrap() == 7.);
    assert!(eval("(- 2 5)").unwrap() == -3.);
    assert!(eval("(/ 8 4)").unwrap() == 2.);
    assert!(eval("(* 8 4)").unwrap() == 32.);
    assert!(eval("(pow 2 8)").unwrap() == 256.);
    assert!(eval("(sqrt 16)").unwrap() == 4.);
}