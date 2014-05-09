/*

Implements an expression struct which can be evaluated.

*/

use std::str::{Slice};
use super::{CalcResult, Evaluate};

// Shortcut to combine two Results and return a new one
// If both results contain an Ok value, the given function will be applied
// Otherwise, the first error found will be returned
fn combine<'a>(opt1: CalcResult, opt2: CalcResult, func: |f64, f64| -> f64) -> CalcResult {
    let (v1, v2) = (try!(opt1), try!(opt2));
    Ok(func(v1, v2))
}

#[deriving(Show)]
pub enum ExprType {
    Add,
    Sub,
    Mul,
    Div,
    Func(&'static str)
}

pub struct Expression {
    pub expr_type: ExprType,
    pub args: Vec<Box<Evaluate>>
}

impl Evaluate for Expression {
    fn eval(&self) -> CalcResult {
        match self.expr_type {
            Add => {
                self.args.iter().fold(Ok(0.0), |acc, x| {
                    combine(acc, x.eval(), |v1, v2| v1 + v2)
                })
            }
            Sub => {
                if self.args.len() == 0 {
                    return Err(Slice("Substraction requires at least one argument"));
                }
                let first_arg = self.args.get(0).eval();
                self.args.slice_from(1).iter().fold(first_arg, |acc, x| {
                    combine(acc, x.eval(), |v1, v2| v1 - v2)
                })
            }
            Mul => {
                self.args.iter().fold(Ok(1.0), |acc, x| {
                    combine(acc, x.eval(), |v1, v2| v1 * v2)
                })
            }
            Div => {
                if self.args.len() == 0 {
                    return Err(Slice("Division requires at least one argument"));
                }
                let first_arg = self.args.get(0).eval();
                if self.args.slice_from(1).iter().any(|x| x.eval() == Ok(0.0)) {
                    return Err(Slice("Cannot divide by 0"));
                }
                self.args.slice_from(1).iter().fold(first_arg, |acc, x| {
                    combine(acc, x.eval(), |v1, v2| v1 / v2)
                })
            }
            Func(_) => {
                Err(Slice("Functions are not yet supported"))
            }
        }
    }
}