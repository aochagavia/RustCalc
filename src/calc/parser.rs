/*

Implements functions to translate a vector of tokens into
an Abstract Syntax Tree that will be used later when the
expression is evaluated.

*/

use std::str::{Slice, Owned};
use super::CalcResult;
use super::scanner;
use super::scanner::{Token, Literal, LPar, RPar, Name, Keyword, Set, Def};
use super::expression;
use super::expression::{Expression, ExprType, Function, Number, Variable};
use super::statement::{Statement, StmtType, Assign, FuncDef};

pub enum AST {
    Expression(Expression),
    Statement(Statement)
}

pub fn parse(token_slice: &[Token]) -> CalcResult<AST> {
    // Check that we have an opening parenthesis
    let mut tokens = token_slice.iter();
    match tokens.next() {
        Some(&LPar) => parse_line(&mut tokens),
        _           => Err(Slice("Parentheses not present or wrongly formatted"))
    }
}

fn parse_line<'a, T: Iterator<&'a Token>>(tokens: &mut T) -> CalcResult<AST> {
    // Depending on the first token, we parse an expression or a statement
    match tokens.next() {
        Some(&scanner::Operator(op)) => {
            parse_expression(tokens, expression::Operator(op))
                .map(|e| Expression(e))
        }
        Some(&Name(ref func_name)) => {
            parse_expression(tokens, Function(func_name.clone()))
                .map(|e| Expression(e))
        }
        Some(&Keyword(k)) => {
            match k {
                Set => parse_statement(tokens, Assign)
                           .map(|s| Statement(s)),
                Def => parse_statement(tokens, FuncDef)
                           .map(|s| Statement(s))
            }
        }
        _ => Err(Slice("Invalid first token"))
    }
}

fn parse_whole_expression<'a, T: Iterator<&'a Token>>(tokens: &mut T)
        -> CalcResult<Expression>
{
    match tokens.next() {
        Some(&scanner::Operator(op)) => {
            parse_expression(tokens, expression::Operator(op))
        }
        Some(&Name(ref func_name)) => {
            parse_expression(tokens, Function(func_name.clone()))
        }
        _ => Err(Slice("Invalid first token"))
    }
}

fn parse_expression<'a, T: Iterator<&'a Token>>(tokens: &mut T, top_expr: ExprType)
        -> CalcResult<Expression>
{
    // Here we will save the arguments of the expression
    let mut args: Vec<Expression> = vec![];

    // Go through the restant tokens and translate the arguments into structs
    loop {
        let token = tokens.next();
        if token.is_none() { return Err(Slice("Unable to find last parentheses of expression")) }
        match *token.unwrap() {
            // Here begins a sub expression
            LPar => {
                // Call this function recursively to get the AST of the sub expression
                let sub_expr = try!(parse_whole_expression(tokens));
                args.push(sub_expr);
            }
            // Here ends an expression
            RPar => {
                // We make a new Expression based on the Expression type and the arguments
                return Ok(Expression{ expr_type: top_expr, args: args });
            }
            // Operator
            scanner::Operator(op) => {
                return Err(Owned(format!("Operator '{}' in wrong position", op)));
            }
            // A number to be used as argument for an Expression
            Literal(x) => {
                let number = Expression::from_type(Number(x));
                args.push(number);
            }
            // The name of a constant or variable
            Name(ref name) => {
                let var = Expression::from_type(Variable(name.clone()));
                args.push(var);
            }
            Keyword(k) => {
                return Err(Owned(format!("Keyword '{}' in wrong position", k)));
            }
        }
    }
}

fn parse_statement<'a, T: Iterator<&'a Token>>(tokens: &mut T, top_stmt: StmtType)
        -> CalcResult<Statement>
{
    match top_stmt {
        FuncDef => fail!("Function definition is not yet implemented"), // FIXME
        Assign  => {
            parse_assign(tokens)
        }
    }
}

fn parse_assign<'a, T: Iterator<&'a Token>>(tokens: &mut T) -> CalcResult<Statement> {
    // The first token will be the name of the variable
    let name = match tokens.next() {
        Some(&Name(ref n)) => n.clone(),
        Some(ref t)        => return Err(Owned(format!("Unexpected {} expecting Name", t))),
        None               => return Err(Slice("Unexpected end of token-stream"))
    };
    
    // The second token will be a number or a sub-expression
    let rhs = match tokens.next() {
        Some(&LPar)       => try!(parse_whole_expression(tokens)),
        Some(&Literal(x)) => Expression::from_type(Number(x)),
        Some(ref t)       => return Err(Owned(format!("Unexpected {} expecting LPar or Literal", t))),
        None              => return Err(Slice("Unexpected end of token-stream"))
    };
    
    Ok(Statement { stmt_type: Assign, name: name, rhs: rhs })
}
