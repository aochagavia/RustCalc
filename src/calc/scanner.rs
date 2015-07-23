/*

Implements a function to transform an input string into a vector
of tokens.

*/

use std::char;

use super::CalcResult;
use super::operator::Operator;
use super::buffer::Buffer;

#[derive(Clone, Debug)]
pub enum Token {
    Literal(f64),               // A number
    LPar,                       // A left parenthesis
    RPar,                       // A right parenthesis
    Operator(Operator),         // An operator
    Name(String),               // A name
    TKeyword(Keyword),          // A keyword
}

#[derive(Clone, Copy, Debug)]
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
            '(' => Some(Token::LPar),
            ')' => Some(Token::RPar),
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
        let word: String = buf.take_until(|&c| c.is_whitespace() || c == ')' || c == '(').into_iter().collect();

        // Operators are always separated by whitespace from the restant tokens
        match Operator::from_str(&word) {
            Some(op_type) => {
                tokens.push(Token::Operator(op_type));
                continue;
            }
            _   => { }
        };

        // -----------------
        // Literals and names
        // -----------------

        // We know that a word has at least one character, so we can safely unwrap it
        let c = word.chars().next().unwrap();

        // A literal token
        if char::is_digit(c, 10) || c == '-' {
            match word.parse::<f64>() {
                Ok(x) => {
                    tokens.push(Token::Literal(x));
                    continue;
                }
                _ => { return Err(format!("Invalid number '{}'", word).into()); }
            }
        }

        // A name token
        if c.is_alphabetic() {
            // It can be a keyword or a name
            match &word[..] {
                "set" => tokens.push(Token::TKeyword(Keyword::Set)),
                "def" => tokens.push(Token::TKeyword(Keyword::Def)),
                _     => tokens.push(Token::Name(word))
            }

            continue;
        }

        // This point is only reached when no token has been matched
        return Err(format!("Unrecognized token '{}'", word).into());
    }

    Ok(tokens)
}
