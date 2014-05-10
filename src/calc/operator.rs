/*

Implements the operators of the calculator.

*/

use std::str::Slice;
use super::{Evaluate, CalcResult};
use super::expression::combine;

#[deriving(Show)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div
}

pub fn eval(op_type: OperatorType, args: &Vec<Box<Evaluate>>) -> CalcResult {
    match op_type {
        Add => {
            args.iter().fold(Ok(0.0), |acc, x| {
                combine(acc, x.eval(), |v1, v2| v1 + v2)
            })
        }
        Sub => {
            if args.len() == 0 {
                return Err(Slice("Substraction requires at least one argument"));
            }
            let first_arg = args.get(0).eval();
            args.slice_from(1).iter().fold(first_arg, |acc, x| {
                combine(acc, x.eval(), |v1, v2| v1 - v2)
            })
        }
        Mul => {
            args.iter().fold(Ok(1.0), |acc, x| {
                combine(acc, x.eval(), |v1, v2| v1 * v2)
            })
        }
        Div => {
            if args.len() == 0 {
                return Err(Slice("Division requires at least one argument"));
            }
            let first_arg = args.get(0).eval();
            if args.slice_from(1).iter().any(|x| x.eval() == Ok(0.0)) {
                return Err(Slice("Cannot divide by 0"));
            }
            args.slice_from(1).iter().fold(first_arg, |acc, x| {
                combine(acc, x.eval(), |v1, v2| v1 / v2)
            })
        }
    }
}