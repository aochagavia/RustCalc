// We need this feature for CalcResult
#![feature(default_type_params)]

#[cfg(not(test))]
use std::io;

mod calc;

#[cfg(not(test))]
fn main() {
    let mut reader = io::stdin();
    let mut env = calc::Environment::new();

    for line in reader.lines().map(|l| l.unwrap_or_else(|_| String::new())) {
        match calc::run(line.as_slice(), &mut env) {
            Err(msg) => println!("Error: {}", msg),
            Ok(result) => println!("Result: {}", result)
        }
    }
}

// General tests
#[cfg(test)]
mod tests {
    use super::calc::{eval, run, Environment};

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

    #[test]
    fn check_subexpr() {
        assert!(eval("(+ (+ 2 3) 5)").unwrap() == 10.);
        assert!(eval("(* (+ 2 3) 5)").unwrap() == 25.)
        assert!(eval("(- (+ 2 3) 5)").unwrap() == 0.)
    }

    #[test]
    fn check_negative() {
        assert!(eval("(+ -5 -5)").unwrap() == -10.);
        assert!(eval("(+ -5 8)").unwrap() == 3.);
    }

    #[test]
    fn check_constants() {
        assert!(eval("(* pi 2)").unwrap() > 6.);
        assert!(eval("(* e 2)").unwrap() > 4.);
        assert!(eval("(* 2 e)").unwrap() > 4.);
    }

    #[test]
    fn check_var() {
        let mut env = Environment::new();

        // Define a variable and use it
        run("(set myVar 700)", &mut env).unwrap();
        assert!(run("(* myVar 2)", &mut env).unwrap() == 1400.0);
    }

    #[test]
    fn check_func() {
        let mut env = Environment::new();

        run("(def (myFunc arg1 arg2 arg3) (+ arg1 arg2 arg3))", &mut env).unwrap();
        assert!(run("(myFunc 1 2 3)", &mut env).unwrap() == 6.0);
    }
}
