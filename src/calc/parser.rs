/*

Implements functions to translate a vector of tokens into
an Abstract Syntax Tree that will be used later when the
expression is evaluated.

*/

use super::CalcResult;
use super::scanner::{Keyword, Token};
use super::expression::{self, Expression, ExprType};
use super::statement::{self, Statement, StmtType};

pub enum AST {
    Expression(expression::Expression),
    Statement(statement::Statement)
}

pub fn parse(token_slice: &[Token]) -> CalcResult<AST> {
    // Check that we have an opening parenthesis
    let mut tokens = token_slice.iter();
    match tokens.next() {
        Some(&Token::LPar) => parse_line(&mut tokens),
        _ => Err("Parentheses not present or wrongly formatted".into())
    }
}

fn parse_line<'a, 'b, T: Iterator<Item=&'a Token>>(tokens: &'b mut T) -> CalcResult<AST> {
    // Depending on the first token, we parse an expression or a statement
    match tokens.next() {
        Some(&Token::Operator(op)) => {
            parse_expression(tokens, ExprType::Operator(op))
                .map(|e| AST::Expression(e))
        }
        Some(&Token::Name(ref func_name)) => {
            parse_expression(tokens, ExprType::Function(func_name.clone()))
                .map(|e| AST::Expression(e))
        }
        Some(&Token::TKeyword(k)) => {
            match k {
                Keyword::Set => parse_statement(tokens, StmtType::Assign)
                           .map(|s| AST::Statement(s)),
                Keyword::Def => parse_statement(tokens, StmtType::FuncDef)
                           .map(|s| AST::Statement(s))
            }
        }
        _ => Err("Invalid first token".into())
    }
}

fn parse_whole_expression<'a, 'b, T: Iterator<Item=&'a Token>>(tokens: &'b mut T)
        -> CalcResult<Expression>
{
    match tokens.next() {
        Some(&Token::Operator(op)) => {
            parse_expression(tokens, ExprType::Operator(op))
        }
        Some(&Token::Name(ref func_name)) => {
            parse_expression(tokens, ExprType::Function(func_name.clone()))
        }
        _ => Err("Invalid first token".into())
    }
}

fn parse_expression<'a, 'b, T: Iterator<Item=&'a Token>>(tokens: &'b mut T, top_expr: ExprType)
        -> CalcResult<Expression>
{
    // Here we will save the arguments of the expression
    let mut args: Vec<Expression> = vec![];

    // Go through the restant tokens and translate the arguments into structs
    loop {
        let token = tokens.next();
        if token.is_none() { return Err("Unable to find last parentheses of expression".into()) }
        match *token.unwrap() {
            // Here begins a sub expression
            Token::LPar => {
                // Call this function recursively to get the AST of the sub expression
                let sub_expr = try!(parse_whole_expression(tokens));
                args.push(sub_expr);
            }
            // Here ends an expression
            Token::RPar => {
                // We make a new Expression based on the Expression type and the arguments
                return Ok(expression::Expression{ expr_type: top_expr, args: args });
            }
            // Operator
            Token::Operator(op) => {
                return Err(format!("Operator '{:?}' in wrong position", op).into());
            }
            // A number to be used as argument for an Expression
            Token::Literal(x) => {
                let number = Expression::from_type(ExprType::Number(x));
                args.push(number);
            }
            // The name of a constant or variable
            Token::Name(ref name) => {
                let var = Expression::from_type(ExprType::Variable(name.clone()));
                args.push(var);
            }
            Token::TKeyword(k) => {
                return Err(format!("Keyword '{:?}' in wrong position", k).into());
            }
        }
    }
}

fn parse_statement<'a, 'b, T: Iterator<Item=&'a Token>>(tokens: &'b mut T, top_stmt: StmtType)
        -> CalcResult<Statement>
{
    match top_stmt {
        StmtType::FuncDef => panic!("Function definition is not yet implemented"), // FIXME
        StmtType::Assign  => {
            parse_assign(tokens)
        }
    }
}

fn parse_assign<'a, 'b, T: Iterator<Item=&'a Token>>(tokens: &'b mut T) -> CalcResult<Statement> {
    // The first token will be the name of the variable
    let name = match tokens.next() {
        Some(&Token::Name(ref n)) => n.clone(),
        Some(ref t)        => return Err(format!("Unexpected {:?} expecting Name", t).into()),
        None               => return Err("Unexpected end of token-stream".into())
    };
    
    // The second token will be a number or a sub-expression
    let rhs = match tokens.next() {
        Some(&Token::LPar)       => try!(parse_whole_expression(tokens)),
        Some(&Token::Literal(x)) => Expression::from_type(ExprType::Number(x)),
        Some(ref t)       => return Err(format!("Unexpected {:?} expecting LPar or Literal", t).into()),
        None              => return Err("Unexpected end of token-stream".into())
    };
    
    Ok(Statement { stmt_type: StmtType::Assign, name: name, rhs: rhs })
}
