/*

Implements the functions of the calculator.

*/

use super::CalcResult;
use super::expression::Expression;
use super::environment::Environment;

#[derive(Clone, Copy, Debug)]
pub enum Function {
    Sqrt,
    Pow,
    If
}

impl Function {
    pub fn eval(&self, args: &[Expression], env: &Environment) -> CalcResult {
        match *self {
            Function::Sqrt => {
                if args.len() != 1 {
                    Err("'sqrt' requires one argument".into())
                } else {
                    let x = try!(args[0].eval(env));
                    Ok(x.sqrt())
                }
            }
            Function::Pow => {
                if args.len() != 2 {
                    Err("'pow' requires two arguments".into())
                } else {
                    let base = try!(args[0].eval(env));
                    let exponent = try!(args[1].eval(env));
                    Ok(base.powf(exponent))
                }
            }
            Function::If => {
                if args.len() != 3 {
                    Err("'if' requires three arguments".into())
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
            "sqrt" => Ok(Function::Sqrt),
            "pow"  => Ok(Function::Pow),
            "if"   => Ok(Function::If),
            _      => Err(format!("Unknown function '{}'", s).into())
        }
    }
}
