use std::fmt;

use crate::scanner::{LiteralValue, Token};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    // ERRORS BEFORE SCANNING TOKENS
    UnknownToken(char, i128),
    UnterminatedStringError(i128),
    FloatParsingError(String, i128),
    IntegerParsingError(String, i128),

    // AFTER SCANNING TOKEN
    InvalidToken(Token),
    ZeroDivisionError(Token),
    InvalidUnaryOperation(LiteralValue, Token),
    UnterminatedParenthesis(Token),
    InvalidBinaryOperation(LiteralValue, Token, LiteralValue),
    ExpectedAToken(Token, String),

    // PARSING ERROR
    ParsingError(String, i128),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error {
    pub fn to_string(self: &Self) -> String {
        match self {
            Error::UnknownToken(x, line_number) => {
                format!("UNKNOWN TOKEN : {}, at line {}", x, line_number)
            }
            Error::InvalidToken(token) => {
                format!(
                    "INVALID TOKEN : {}, at line {}",
                    token.lexeme, token.line_number
                )
            }
            Error::ZeroDivisionError(token) => {
                format!("ZERO DIVISION ERROR at line {}", token.line_number)
            }
            Error::ParsingError(msg, line_number) => {
                format!("PARSING ERROR : {} at line {}", msg, line_number)
            }
            Error::UnterminatedStringError(line_number) => {
                format!("UNTERMINATED STRING at line {}", line_number)
            }
            Error::FloatParsingError(msg, line_number) => {
                format!("{} at line {}", msg, line_number)
            }
            Error::IntegerParsingError(msg, line_number) => {
                format!("{} at line {}", msg, line_number)
            }
            Error::UnterminatedParenthesis(token) => {
                format!(
                    "UNTERMINATED PARENTHESIS : {}, at line {}",
                    token.lexeme, token.line_number
                )
            }
            Error::InvalidBinaryOperation(left, operand, right) => {
                format!(
                    "OPERATION {} is not defined for operands of type {}, {}.",
                    operand.token_type.to_string(),
                    left.to_string(),
                    right.to_string()
                )
            }
            Error::InvalidUnaryOperation(value, operation) => {
                format!(
                    "OPERATION {} is not defined for operands of type {}.",
                    operation.token_type.to_string(),
                    value.to_string()
                )
            }
            Error::ExpectedAToken(token, msg) => {
                format!("EXPECTED A TOKEN : {} on line {}", msg, token.line_number)
            }
        }
    }
}
