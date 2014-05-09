// We need this feature for CalcResult
#![feature(default_type_params)]

use std::io;
use calc::eval;

mod calc;

fn main() {
    let mut reader = io::stdin();
    
    for line in reader.lines().map(|l| l.unwrap()) {
        match eval(line) {
            Err(msg) => println!("Error: {}", msg),
            Ok(result) => println!("Result: {}", result)
        }
    }
}