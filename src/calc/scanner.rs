/*

Implements a function to transform an input string into a vector
of tokens.

*/

use std::str::Owned;
use super::CalcResult;
use super::operator::Operator;
use super::buffer::Buffer;

#[deriving(Show)]
pub enum Token {
    Literal(f64),               // A number
    LPar,                       // A left parenthesis
    RPar,                       // A right parenthesis
    Operator(Operator),         // An operator
    Name(String),               // A name
    Keyword(Keyword),            // A keyword
}

#[deriving(Show)]
pub enum Keyword {
    Set,
    Def
}

pub fn scan(s: &str) -> CalcResult<Vec<Token>> {
    let mut buf = Buffer::new(s.chars());
    let mut tokens = Vec::new();

    while !buf.is_empty() {
        // Skip whitespace
        if buf.peek().unwrap().is_whitespace() {
            buf.pop();
            continue;
        }

        // -----------
        // Parentheses
        // -----------
        let token = match buf.peek().unwrap() {
            '(' => Some(LPar),
            ')' => Some(RPar),
            _   => None
        };
        if token.is_some() {
            tokens.push(token.unwrap());
            buf.pop();
            continue;
        }

        // ---------
        // Operators
        // ---------

        // We know that there is at least one word
        let word = String::from_chars(buf.take_until(|&c| c.is_whitespace() || c == ')' || c == '(').as_slice());

        // Operators are always separated by whitespace from the restant tokens
        match Operator::from_str(word.as_slice()) {
            Some(op_type) => {
                tokens.push(Operator(op_type));
                continue;
            }
            _   => { }
        };

        // -----------------
        // Literals and names
        // -----------------

        // We know that a word has at least one character, so we can safely unwrap it
        let c = word.as_slice().chars().next().unwrap();

        // A literal token
        if c.is_digit() || c == '-' {
            match from_str::<f64>(word.as_slice()) {
                Some(x) => {
                    tokens.push(Literal(x));
                    continue;
                }
                None => { return Err(Owned(format!("Invalid number '{}'", word))); }
            }
        }

        // A name token
        if c.is_alphabetic() {
            // It can be a keyword or a name
            match word.as_slice() {
                "set" => tokens.push(Keyword(Set)),
                "def" => tokens.push(Keyword(Def)),
                _     => tokens.push(Name(word))
            }

            continue;
        }

        // This point is only reached when no token has been matched
        return Err(Owned(format!("Unrecognized token '{}'", word)));
    }

    Ok(tokens)
}
