/*

Implements the constants of the calculator.

*/

use std::str::Owned;
use super::CalcResult;

#[deriving(Show)]
pub enum ConstantType {
    Pi,
    E
}

pub struct Constant(pub ConstantType);

impl Constant {
    pub fn from_str(s: &str) -> CalcResult<Constant> {
        match s {
            "pi" => Ok(Constant(Pi)),
            "e"  => Ok(Constant(E)),
            _    => Err(Owned(format!("Undefined constant '{}'", s)))
        }
    }

    pub fn eval(&self) -> CalcResult {
        let &Constant(c) = self;
        match c {
            Pi => Ok(3.14159265359),
            E  => Ok(2.71828182846)
        }
    }
}
