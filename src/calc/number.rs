/*

Implements a struct containing a number which can be evaluated.

*/

use super::{CalcResult, Evaluate};

pub struct Number(pub f64);

impl Evaluate for Number {
    fn eval(&self) -> CalcResult {
        let &Number(x) = self;
        Ok(x)
    }
}