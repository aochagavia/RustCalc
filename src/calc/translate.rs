/*

Implements functions to translate a vector of tokens into
an Abstract Syntax Tree that will be used later when the
expression is evaluated.

*/

use std::str;
use std::str::{Slice, Owned};
use super::{CalcResult, Evaluate, Number};
use super::tokenize::{Token, Literal, LPar, RPar, Operator, Name};
use super::expression;
use super::expression::{Expression, Function};
use super::function;

pub fn translate(tokens: &[Token]) -> CalcResult<Box<Evaluate>> {
    // Check that the parentheses are good
    match (tokens.iter().next(), tokens.iter().rev().next()) {
        (Some(&LPar), Some(&RPar)) => {}
        _                        => return Err(Slice("Parentheses not present or wrongly formatted"))
    }

    // There must be an operator or a function name at the top of the expression
    // We check here that it is present
    let top_expr = match tokens[1] {
        Operator(op_type) => expression::Operator(op_type),
        Name(ref func_name)   => Function(try!(function::from_str(func_name.as_slice()))),//return Err(Slice("Functions are not yet supported")),
        _                 => return Err(Slice("Operator or function name not present at the beginning of the expression"))
    };
    
    // Here we will save the arguments of the expression
    let mut args: Vec<Box<Evaluate>> = Vec::new();
    
    let top = tokens.len();
    let mut i = 2u;
    
    // Go through the tokens and translate the arguments into structs
    while i < top {
        match tokens[i] {
            // Here begins a sub expression
            LPar => {
                // The sub expression can be used as argument of the current expression
                // We must find the closing parentheses
                let limit = try!(find_rpar(tokens, i, top)) + 1;
                
                // Call this function recursively to get the AST of the sub expression
                let sub_expr = try!(translate(tokens.slice(i, limit)));
                
                args.push(sub_expr);
                
                // Skip the tokens corresponding to this expression
                i = limit;
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
                i += 1;
            }
            // The name of a constant
            Name(_) => {
                return Err(Slice("Constants are not yet implemented"));
            }
        }
    }
    
    Err(Slice("Unable to find last parentheses of expression"))
}

// Returns the parentheses that closes an expression or an error if it can't be found
fn find_rpar(tokens: &[Token], begin: uint, end: uint) -> Result<uint, str::MaybeOwned> {
    let mut i = begin;
    let mut p_count = 0;
    while i <= end {
        match tokens[i] {
            LPar => p_count += 1,
            RPar => p_count -= 1,
            _    => { }
        }
        if p_count == 0 {
            return Ok(i);
        }
        i += 1;
    }

    Err(Slice("Parentheses not present or wrongly formatted "))
}