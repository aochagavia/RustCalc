/*

Implements a function to transform an input string into a vector
of tokens.

*/

use std::str::Owned;
use super::CalcResult;
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
        
        // -----------
        // Parentheses
        // -----------
        let token = match slice.chars().next().unwrap() {
            '(' => Some(LPar),
            ')' => Some(RPar),
            _   => None
        };
        if token.is_some() {
            tokens.push(token.unwrap());
            i += 1;
            continue;
        }
        
        // -----------------
        // Literals and names
        // -----------------
        
        // We know that there is at least one word, so we can safely unwrap it
        let word = slice.words().next().unwrap();
        
        // Discard parentheses at the end if present
        let word = word.slice(0, word.find(|c: char| c == ')' || c == '(').unwrap_or(word.len()));
        
        // We know that a word has at least one character, so we can safely unwrap it
        let c = word.chars().next().expect(format!("Could not unwrap first char of word '{}'. Parsing on index {} of slice {}.", word, i, slice));
        
        // A literal token
        if c.is_digit() || c == '-' {        
            match from_str::<f64>(word) {
                Some(x) => {
                    tokens.push(Literal(x));
                    i += word.len();
                    continue;
                }
                None => { return Err(Owned(format!("Invalid number '{}'", word))); }
            }
        }
        
        // A name token
        if c.is_alphabetic() {
            tokens.push(Name(word.to_owned()));
            i += word.len();
            continue;
        }
        
        // ---------
        // Operators
        // ---------
        
        // Operators are always separated by whitespace from the restant tokens
        let token = match word {
            "+" => Some(Operator(Add)),
            "-" => Some(Operator(Sub)),
            "*" => Some(Operator(Mul)),
            "/" => Some(Operator(Div)),
            _   => None
        };
        if token.is_some() {
            tokens.push(token.unwrap());
            i += 1;
            continue;
        }

        // This point is only reached when no token has been matched
        return Err(Owned(format!("Unrecognized token '{}'", word)));
    }
    
    Ok(tokens)
}