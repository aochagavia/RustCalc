/*

Implements the functions of the calculator.

*/

use std::num::Float;
use std::str::{Slice, Owned};
use super::{Evaluate, CalcResult};

#[deriving(Show)]
pub enum FunctionType {
    Sqrt,
    Pow2
}

pub fn eval(f_type: FunctionType, args: &Vec<Box<Evaluate>>) -> CalcResult {
    match f_type {
        Sqrt => {
            if args.len() != 1 {
                Err(Slice("'sqrt' requires one argument"))
            } else {
                let x = try!(args.get(0).eval());
                Ok(x.sqrt())
            }
        }
        Pow2 => {
            if args.len() != 1 {
                Err(Slice("'pow2' requires one argument"))
            } else {
                let base = try!(args.get(0).eval());
                Ok(base * base)
            }
        }
    }
}

pub fn from_str(s: &str) -> CalcResult<FunctionType> {
    match s {
        "sqrt" => Ok(Sqrt),
        "pow2"  => Ok(Pow2),
        _      => Err(Owned(format!("Unknown function '{}'", s)))
    }
}