/*

Implements an expression struct which can be evaluated.

*/

use super::CalcResult;
use super::environment::Environment;
use super::operator;
use super::constant::Constant;
use super::function;

#[derive(Debug)]
pub enum ExprType {
    Operator(operator::Operator),
    Function(String),
    Number(f64),
    Variable(String)
}

pub struct Expression {
    pub expr_type: ExprType,
    pub args: Vec<Expression>
}

impl Expression {
    pub fn from_type(ty: ExprType) -> Expression {
        Expression { expr_type: ty, args: vec![] }
    }

    pub fn eval(&self, env: &Environment) -> CalcResult {
        match self.expr_type {
            ExprType::Operator(op) => {
                op.eval(&self.args, env)
            }
            ExprType::Function(ref name) => {
                // Check if the function has been defined by the user
                match env.get_fn(name) {
                    Some(f) => return f.eval(&self.args, env),
                    None    => ()
                }

                // Otherwise, treat it as a predefined function
                function::Function::from_str(name)
                    .and_then(|f| f.eval(&self.args, env))
            }
            ExprType::Number(x) => {
                Ok(x)
            }
            ExprType::Variable(ref name) => {
                // Check if the variable has been defined by the user
                match env.get_var(name) {
                    Some(v) => return Ok(v),
                    None    => ()
                }

                // Otherwise, treat it as a constant
                Constant::from_str(name)
                    .and_then(|c| c.eval())
            }
        }
    }
}
