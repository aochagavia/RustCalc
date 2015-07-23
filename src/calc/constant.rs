/*

Implements the constants of the calculator.

*/

use std::f64;

use super::CalcResult;

#[derive(Debug)]
pub enum Constant {
    Pi,
    E
}

impl Constant {
    pub fn from_str(s: &str) -> CalcResult<Constant> {
        match s {
            "pi" => Ok(Constant::Pi),
            "e"  => Ok(Constant::E),
            _    => Err(format!("Undefined constant '{}'", s).into())
        }
    }

    pub fn eval(&self) -> CalcResult {
        match *self {
            Constant::Pi => Ok(f64::consts::PI),
            Constant::E  => Ok(f64::consts::E)
        }
    }
}
