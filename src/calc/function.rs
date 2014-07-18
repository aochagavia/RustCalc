/*

Implements the functions of the calculator.

*/

use std::num::Float;
use std::str::{Slice, Owned};
use super::CalcResult;
use super::expression::Expression;
use super::environment::Environment;

#[deriving(Show)]
pub enum Function {
    Sqrt,
    Pow,
    If
}

impl Function {
    pub fn eval(&self, args: &[Expression], env: &Environment) -> CalcResult {
        match *self {
            Sqrt => {
                if args.len() != 1 {
                    Err(Slice("'sqrt' requires one argument"))
                } else {
                    let x = try!(args[0].eval(env));
                    Ok(x.sqrt())
                }
            }
            Pow => {
                if args.len() != 2 {
                    Err(Slice("'pow' requires two arguments"))
                } else {
                    let base = try!(args[0].eval(env));
                    let exponent = try!(args[1].eval(env));
                    Ok(base.powf(exponent))
                }
            }
            If => {
                if args.len() != 3 {
                    Err(Slice("'if' requires three arguments"))
                } else {
                    let condition = try!(args[0].eval(env));

                    // 0 means false, other means true
                    if condition == 0. {
                        Ok(try!(args[2].eval(env)))
                    } else {
                        Ok(try!(args[1].eval(env)))
                    }
                }
            }
        }
    }

    pub fn from_str(s: &str) -> CalcResult<Function> {
        match s {
            "sqrt" => Ok(Sqrt),
            "pow"  => Ok(Pow),
            "if"   => Ok(If),
            _      => Err(Owned(format!("Unknown function '{}'", s)))
        }
    }
}
