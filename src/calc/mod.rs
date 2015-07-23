/*

Implements a polish notation calculator.

See the module files for more information about how things are
implemented.

*/

use std::borrow::Cow;

pub use self::environment::Environment;
use self::parser::AST;

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
pub type CalcResult<T = f64> = Result<T, Cow<'static, str>>;

// Evaluates a string
pub fn eval(s: &str) -> CalcResult {
    let tokens = try!(scanner::scan(s.trim()));
    let ast = try!(parser::parse(&tokens));

    let mut env = Environment::new();
    match ast {
        AST::Expression(e) => e.eval(&env),
        AST::Statement(s)  => s.exec(&mut env).map(|_| 0.)
    }
}

// Runs the code contained in a string, using the given environment
pub fn run(s: &str, env: &mut Environment) -> CalcResult {
    let tokens = try!(scanner::scan(s.trim()));
    let ast = try!(parser::parse(&tokens));

    match ast {
        AST::Expression(e) => e.eval(env),
        AST::Statement(s)  => s.exec(env).map(|_| 0.)
    }
}
