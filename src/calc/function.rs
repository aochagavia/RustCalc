/*

Implements the functions of the calculator.

*/

use std::num::Float;
use std::str::{Slice, Owned};
use super::{Evaluate, CalcResult};

#[deriving(Show)]
pub enum FunctionType {
    Sqrt,
    Pow,
    If
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
        Pow => {
            if args.len() != 2 {
                Err(Slice("'pow' requires two arguments"))
            } else {
                let base = try!(args.get(0).eval());
                let exponent = try!(args.get(1).eval());
                Ok(base.powf(exponent))
            }
        }
        If => {
            if args.len() != 3 {
                Err(Slice("'if' requires three arguments"))
            } else {
                let condition = try!(args.get(0).eval());
                
                // 0 means false, other means true
                if condition == 0. {
                    Ok(try!(args.get(2).eval()))
                } else {
                    Ok(try!(args.get(1).eval()))
                }
            }
        }
    }
}

pub fn from_str(s: &str) -> CalcResult<FunctionType> {
    match s {
        "sqrt" => Ok(Sqrt),
        "pow"  => Ok(Pow),
        "if"   => Ok(If),
        _      => Err(Owned(format!("Unknown function '{}'", s)))
    }
}