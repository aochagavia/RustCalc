/*

Implements a polish notation calculator.

See the module files for more information about how things are
implemented.

*/

use std::str;
use self::tokenize::tokenize;
use self::translate::translate;
pub use self::number::Number;

mod tokenize;
mod translate;
mod expression;
mod number;

// A shortcut for the result type that is used everywhere
pub type CalcResult<T = f64> = Result<T, str::MaybeOwned<'static>>;

// This trait is implemented by all structs that can be evaluated
pub trait Evaluate {
    fn eval(&self) -> CalcResult;
}

// Evaluates a string
pub fn eval(s: &str) -> CalcResult {
    let tokens = try!(tokenize(s.trim()));
    let expr = try!(translate(tokens.as_slice()));
    expr.eval()
}