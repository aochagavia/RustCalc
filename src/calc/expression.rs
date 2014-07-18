/*

Implements an expression struct which can be evaluated.

*/

use super::CalcResult;
use super::environment::Environment;
use super::function::Function;
use super::operator::Operator;
use super::constant::Constant;

#[deriving(Show)]
pub enum ExprType {
    Operator(Operator),
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
            Operator(op) => {
                op.eval(self.args.as_slice(), env)
            }
            Function(ref name) => {
                // Check if the function has been defined by the user
                match env.get_fn(name.as_slice()) {
                    Some(f) => return f.eval(self.args.as_slice(), env),
                    None    => ()
                }

                // Otherwise, treat it as a predefined function
                Function::from_str(name.as_slice())
                    .and_then(|f| f.eval(self.args.as_slice(), env))
            }
            Number(x) => {
                Ok(x)
            }
            Variable(ref name) => {
                // Check if the variable has been defined by the user
                match env.get_var(name.as_slice()) {
                    Some(v) => return Ok(v),
                    None    => ()
                }

                // Otherwise, treat it as a constant
                Constant::from_str(name.as_slice())
                    .and_then(|c| c.eval())
            }
        }
    }
}
