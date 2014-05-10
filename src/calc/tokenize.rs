/*

Implements a function to transform an input string into a vector
of tokens.

*/

use std::str::Owned;
use super::CalcResult;
use super::expression::{ExprType, Function, Operator};
use super::operator::{OperatorType, Add, Sub, Mul, Div}; 

#[deriving(Show)]
pub enum Token {
    Literal(f64),               // A number
    LPar,                       // A left parenthesis
    RPar,                       // A right parenthesis
    Operator(OperatorType),     // An Expression
    Name(~str)                  // A name
}

pub fn tokenize(s: &str) -> CalcResult<Vec<Token>> {
    let mut tokens = Vec::new();
    
    let mut i = 0;
    let len = s.len();

    while i < len {
        let slice = s.slice_from(i);
        
        // Skip whitespace
        if slice.chars().next().unwrap().is_whitespace() {
            i += 1;
            continue;
        }
        
        // One-char tokens
        let token = match slice.chars().next().unwrap() {
            '(' => Some(LPar),
            ')' => Some(RPar),
            '+' => Some(Operator(Add)),
            '-' => Some(Operator(Sub)),
            '*' => Some(Operator(Mul)),
            '/' => Some(Operator(Div)),
            _   => None
        };
        if token.is_some() {
            tokens.push(token.unwrap());
            i += 1;
            continue;
        }
        
        // Multi-char tokens
        // We know that there is at least one word, so we can safely unwrap it
        let word = slice.words().next().unwrap();
        
        // Discard parentheses if present
        let word = word.slice(0, word.find(|c: char| c == ')' || c == '(').unwrap_or(word.len()));
        
        // We know that a word has at least one character, so we can safely unwrap it
        let c = word.chars().next().unwrap();
        
        // A name token
        if c.is_alphabetic() {
            tokens.push(Name(word.to_owned()));
            i += word.len();
            continue;
        }
        
        // A literal token
        if c.is_digit() {        
            match from_str::<f64>(word) {
                Some(x) => {
                    tokens.push(Literal(x));
                    i += word.len();
                    continue;
                }
                None => { return Err(Owned(format!("Invalid number '{}'", word))); }
            }
        }

        return Err(Owned(format!("Unrecognized token '{}'", word)));
    }
    
    Ok(tokens)
}