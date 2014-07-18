/*

Implements a polish notation calculator.

See the module files for more information about how things are
implemented.

*/

use std::str;
pub use self::environment::Environment;
use self::parser::{Expression, Statement};

mod scanner;
mod parser;
mod environment;
mod expression;
mod statement;
mod constant;
mod operator;
mod function;
mod buffer;
mod util;

// A shortcut for the result type that is used everywhere
pub type CalcResult<T = f64> = Result<T, str::MaybeOwned<'static>>;

// Evaluates a string
pub fn eval(s: &str) -> CalcResult {
    let tokens = try!(scanner::scan(s.trim()));
    let ast = try!(parser::parse(tokens.as_slice()));

    let mut env = Environment::new();
    match ast {
        Expression(e) => e.eval(&env),
        Statement(s)  => s.exec(&mut env).map(|_| 0.)
    }
}

// Runs the code contained in a string, using the given environment
pub fn run(s: &str, env: &mut Environment) -> CalcResult {
    let tokens = try!(scanner::scan(s.trim()));
    let ast = try!(parser::parse(tokens.as_slice()));

    match ast {
        Expression(e) => e.eval(env),
        Statement(s)  => s.exec(env).map(|_| 0.)
    }
}
