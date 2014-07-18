/*

Implements the operators of the calculator.

*/

use std::str::Slice;
use super::CalcResult;
use super::expression::Expression;
use super::environment::Environment;
use super::util::combine;

#[deriving(Show)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    NotEq
}

impl Operator {
    pub fn eval(&self, args: &[Expression], env: &Environment) -> CalcResult {
        match *self {
            Add => {
                args.iter().fold(Ok(0.0), |acc, x| {
                    combine(acc, x.eval(env), |v1, v2| v1 + v2)
                })
            }
            Sub => {
                if args.len() == 0 {
                    return Err(Slice("Substraction requires at least one argument"));
                }
                let first_arg = args[0].eval(env);
                args.slice_from(1).iter().fold(first_arg, |acc, x| {
                    combine(acc, x.eval(env), |v1, v2| v1 - v2)
                })
            }
            Mul => {
                args.iter().fold(Ok(1.0), |acc, x| {
                    combine(acc, x.eval(env), |v1, v2| v1 * v2)
                })
            }
            Div => {
                if args.len() != 2 {
                    return Err(Slice("Division requires two arguments"));
                }
                let first_arg = args[0].eval(env);
                if args.slice_from(1).iter().any(|x| x.eval(env) == Ok(0.0)) {
                    return Err(Slice("Cannot divide by 0"));
                }
                args.slice_from(1).iter().fold(first_arg, |acc, x| {
                    combine(acc, x.eval(env), |v1, v2| v1 / v2)
                })
            }
            Eq => {
                if args.len() < 2 {
                    return Err(Slice("== requires at least two arguments"));
                }

                let mut equal = false;
                let first = try!(args[0].eval(env));
                for x in args.slice_from(1).iter() {
                    equal = equal && try!(x.eval(env)) == first;
                }

                if equal { Ok(1.) } else { Ok(0.) }
            }
            Lt => {
                if args.len() != 2 {
                    return Err(Slice("< requires two arguments"));
                }
                let (arg1, arg2) = (try!(args[0].eval(env)), try!(args[1].eval(env)));
                Ok(bool_to_f64(arg1 < arg2))
            }
            LtEq => {
                if args.len() != 2 {
                    return Err(Slice("<= requires two arguments"));
                }
                Gt.eval(args, env).map(negate_f64)
            }
            Gt => {
                if args.len() != 2 {
                    return Err(Slice("> requires two arguments"));
                }
                let (arg1, arg2) = (try!(args[0].eval(env)), try!(args[1].eval(env)));
                Ok(bool_to_f64(arg1 > arg2))
            }
            GtEq => {
                if args.len() != 2 {
                    return Err(Slice(">= requires two arguments"));
                }
                Lt.eval(args, env).map(negate_f64)
            }
            NotEq => {
                if args.len() != 2 {
                    return Err(Slice("!= requires two arguments"));
                }
                Eq.eval(args, env).map(negate_f64)
            }
        }
    }

    pub fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+"  => Some(Add),
            "-"  => Some(Sub),
            "*"  => Some(Mul),
            "/"  => Some(Div),
            "==" => Some(Eq),
            "<"  => Some(Lt),
            "<=" => Some(LtEq),
            ">"  => Some(Gt),
            ">=" => Some(GtEq),
            "!=" => Some(NotEq),
            _    => None
        }
    }
}

// We use 0 as false and any other value as true
fn bool_to_f64(b: bool) -> f64 {
    if b { 1. } else { 0. }
}

// Negates a f64 which is used as a boolean
fn negate_f64(n: f64) -> f64 {
    if n == 0. { 1. } else { 0. }
}
