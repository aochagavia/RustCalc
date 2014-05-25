/*

Implements functions to translate a vector of tokens into
an Abstract Syntax Tree that will be used later when the
expression is evaluated.

*/

use std::str::{Slice, Owned};
use super::{CalcResult, Evaluate, Number};
use super::constant::Constant;
use super::tokenize::{Token, Literal, LPar, RPar, Operator, Name};
use super::expression;
use super::expression::{Expression, Function};
use super::function;

pub fn translate(token_slice: &[Token]) -> CalcResult<Box<Evaluate>> {
    // Check that we have an opening parenthesis
    let mut tokens = token_slice.iter();
    match tokens.next() {
        Some(&LPar) => translate_iter(&mut tokens),
        _           => Err(Slice("Parentheses not present or wrongly formatted"))
    }
}

pub fn translate_iter<'a, T: Iterator<&'a Token>>(tokens: &mut T) -> CalcResult<Box<Evaluate>> {
    // There must be an operator or a function name at the top of the expression
    // We check here that it is present
    let top_expr = match tokens.next() {
        Some(&Operator(op_type))   => expression::Operator(op_type),
        Some(&Name(ref func_name)) => Function(try!(function::from_str(func_name.as_slice()))),
        _ => return Err(Slice("Operator or function name not present at the beginning of the expression"))
    };
    
    // Here we will save the arguments of the expression
    let mut args: Vec<Box<Evaluate>> = Vec::new();
    
    // Go through the restant tokens and translate the arguments into structs
    loop {
        let token = tokens.next();
        if token.is_none() { return Err(Slice("Unable to find last parentheses of expression")) }
        match *token.unwrap() {
            // Here begins a sub expression
            LPar => {
                // Call this function recursively to get the AST of the sub expression
                let sub_expr = try!(translate_iter(tokens));
                args.push(sub_expr);
            }
            // Here ends an expression
            RPar => {
                // We make a new Expression based on the Expression type and the arguments
                return Ok(box Expression{ expr_type: top_expr, args: args } as Box<Evaluate>);
            }
            // Operator
            Operator(op) => {
                return Err(Owned(format!("Operator '{}' in wrong position", op)));
            }
            // A number to be used as argument for an Expression
            Literal(x) => {
                args.push(box Number(x) as Box<Evaluate>);
            }
            // The name of a constant
            Name(ref c_name) => {
                let constant = box try!(Constant::from_str(c_name.as_slice()));
                args.push(constant as Box<Evaluate>);
            }
        }
    }
}