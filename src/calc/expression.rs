/*

Implements an expression struct which can be evaluated.

*/

use super::{CalcResult, Evaluate};
use super::function;
use super::function::FunctionType;
use super::operator;
use super::operator::OperatorType;

// Shortcut to combine two Results and return a new one
// If both results contain an Ok value, the given function will be applied
// Otherwise, the first error found will be returned
pub fn combine(opt1: CalcResult, opt2: CalcResult, func: |f64, f64| -> f64) -> CalcResult {
    let (v1, v2) = (try!(opt1), try!(opt2));
    Ok(func(v1, v2))
}

#[deriving(Show)]
pub enum ExprType {
    Operator(OperatorType),
    Function(FunctionType)
}

pub struct Expression {
    pub expr_type: ExprType,
    pub args: Vec<Box<Evaluate>>
}

impl Evaluate for Expression {
    fn eval(&self) -> CalcResult {
        match self.expr_type {
            Operator(op_type) => {
                operator::eval(op_type, &self.args)
            }
            Function(f_type) => {
                function::eval(f_type, &self.args)
            }
        }
    }
}