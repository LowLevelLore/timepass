extern crate lazy_static;

use crate::errors::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_string(), TokenType::AND);
        m.insert("class".to_string(), TokenType::CLASS);
        m.insert("else".to_string(), TokenType::ELSE);
        m.insert("false".to_string(), TokenType::FALSE);
        m.insert("for".to_string(), TokenType::FOR);
        m.insert("fun".to_string(), TokenType::FUN);
        m.insert("if".to_string(), TokenType::IF);
        m.insert("nil".to_string(), TokenType::NIL);
        m.insert("or".to_string(), TokenType::OR);
        m.insert("print".to_string(), TokenType::PRINT);
        m.insert("return".to_string(), TokenType::RETURN);
        m.insert("super".to_string(), TokenType::SUPER);
        m.insert("this".to_string(), TokenType::THIS);
        m.insert("true".to_string(), TokenType::TRUE);
        m.insert("var".to_string(), TokenType::VAR);
        m.insert("while".to_string(), TokenType::WHILE);
        return m;
    };
}

pub struct Scanner<'a> {
    source_as_bytes: &'a [u8],
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub fn is_digit(ch: char) -> bool {
    return (ch as u8 >= '0' as u8) && (ch as u8 <= '9' as u8);
}

pub fn is_alpha(ch: char) -> bool {
    return ((ch as u8 >= 'a' as u8) && (ch as u8 <= 'z' as u8))
        || ((ch as u8 >= 'A' as u8) && (ch as u8 <= 'Z' as u8));
}

pub fn is_valid_literal(ch: char) -> bool {
    return is_alpha(ch) || is_digit(ch) || ch == '_';
}

impl<'a> Scanner<'a> {
    pub fn new(content_as_bytes: &'a [u8]) -> Self {
        return Self {
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            source_as_bytes: content_as_bytes,
        };
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line_number: self.line,
        });
        return Ok(self.tokens.clone());
    }

    fn is_at_end(self: &Self) -> bool {
        return self.current >= (self.source_as_bytes.len());
    }

    fn peek(self: &Self) -> char {
        if !self.is_at_end() {
            return self.source_as_bytes[self.current] as char;
        } else {
            return '\0';
        }
    }

    fn scan_token(self: &mut Self) -> Result<(), Error> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '%' => self.add_token(TokenType::MODULO),
            '!' => {
                if self.match_double('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_double('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_double('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.match_double('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.match_double('/') {
                    while self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' => {}
            '\n' => {
                self.line += 1;
            }
            '\t' => {}
            '\r' => {}
            '"' => match self.string() {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            },
            _ => {
                if is_digit(c) {
                    match self.number() {
                        Ok(_) => {}
                        Err(msg) => {
                            return Err(msg);
                        }
                    }
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(Error::UnknownToken(c, self.line as i128));
                }
            }
        }
        return Ok(());
    }

    fn identifier(self: &mut Self) {
        while is_valid_literal(self.peek()) {
            self.advance();
        }
        let mut identifier: String = String::new();
        for i in self.start..self.current {
            identifier.push(self.source_as_bytes[i] as char);
        }
        match KEYWORDS.get(&identifier) {
            Some(value) => {
                self.add_token(value.clone());
            }
            None => {
                self.add_token_to_scanner(
                    TokenType::IDENTIFIER,
                    Some(LiteralValue::IdentifierValue(identifier)),
                );
            }
        }
    }

    fn number(self: &mut Self) -> Result<(), Error> {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
            if !self.is_at_end() {
                let next = self.peek() as char;
                match next {
                    '=' | '!' | '*' | '+' | '-' | '/' | '>' | '<' | ' ' | ';' | ')' | '(' | '%' => {
                    }
                    _ => {
                        return Err(Error::ExpectedAToken(
                            Token {
                                token_type: TokenType::BAD_TOKEN,
                                lexeme: "".to_string(),
                                literal: None,
                                line_number: self.line,
                            },
                            "valid tokens : '=' | '!' | '*' | '+' | '-' | '/' | '>' | '<' | ' ' | ';' | ')' | '(' | '%' "
                                .to_string(),
                        ));
                    }
                }
            }
            let mut substr: String = String::new();
            for i in self.start..self.current {
                substr.push(self.source_as_bytes[i] as char);
            }
            match substr.parse() {
                Ok(value) => {
                    self.add_token_to_scanner(TokenType::NUMBER, Some(LiteralValue::FValue(value)));
                    return Ok(());
                }
                Err(_) => {
                    return Err(Error::ParsingError(
                        format!("cannot parse {} as Float", substr),
                        self.line as i128,
                    ));
                }
            }
        } else {
            if !self.is_at_end() {
                let next = self.peek() as char;
                match next {
                    '=' | '!' | '*' | '+' | '-' | '/' | '>' | '<' | ' ' | ';' | '.' | ')' | '('
                    | '%' => {}
                    _ => {
                        return Err(Error::ExpectedAToken(
                            Token {
                                token_type: TokenType::BAD_TOKEN,
                                lexeme: "".to_string(),
                                literal: None,
                                line_number: self.line,
                            },
                            "valid token : '=' | '!' | '*' | '+' | '-' | '/' | '>' | '<' | ' ' | ';' | '.' | ')' | '(' | '%' "
                                .to_string(),
                        ));
                    }
                }
            }
            let mut substr: String = String::new();
            for i in self.start..self.current {
                substr.push(self.source_as_bytes[i] as char);
            }
            match substr.parse() {
                Ok(value) => {
                    self.add_token_to_scanner(
                        TokenType::NUMBER,
                        Some(LiteralValue::IntValue(value)),
                    );
                    return Ok(());
                }
                Err(_) => {
                    return Err(Error::ParsingError(
                        format!("cannot parse {} as Integer", substr),
                        self.line as i128,
                    ));
                }
            }
        }
    }

    fn peek_next(self: &mut Self) -> char {
        if (self.current + 1) >= self.source_as_bytes.len() {
            return '\0';
        } else {
            return self.source_as_bytes[self.current + 1] as char;
        }
    }

    fn string(self: &mut Self) -> Result<(), Error> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(Error::UnterminatedStringError(self.line as i128));
        } else {
            self.advance();
            let mut value: String = String::new();
            for i in (self.start + 1)..(self.current - 1) {
                value.push(self.source_as_bytes[i] as char)
            }
            self.add_token_to_scanner(TokenType::STRING, Some(LiteralValue::StringValue(value)));
            return Ok(());
        }
    }

    fn advance(self: &mut Self) -> char {
        let res: char = self.source_as_bytes[(self.current) as usize] as char;
        self.current += 1;
        return res;
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_to_scanner(token_type, None);
    }

    fn add_token_to_scanner(
        self: &mut Self,
        token_type: TokenType,
        literal_value: Option<LiteralValue>,
    ) -> () {
        let mut text: String = String::new();
        let bytes = self.source_as_bytes;
        for i in self.start..self.current {
            text.push(bytes[i] as char);
        }
        self.tokens.push(Token {
            token_type: (token_type),
            lexeme: (text),
            literal: (literal_value),
            line_number: (self.line),
        })
    }

    fn match_double(self: &mut Self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if ((self.source_as_bytes[self.current]) as char) != expected {
                return false;
            } else {
                self.current += 1;
                return true;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    MODULO,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
    BAD_TOKEN,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum LiteralValue {
    IntValue(i128),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub(crate) fn is_falsy(self: &mut Self) -> Result<LiteralValue, Error> {
        match self {
            LiteralValue::IntValue(x) => {
                if x.clone() == 0 {
                    return Ok(LiteralValue::True);
                } else {
                    return Ok(LiteralValue::False);
                }
            }
            LiteralValue::FValue(x) => {
                if x.clone() == 0.0 {
                    return Ok(LiteralValue::True);
                } else {
                    return Ok(LiteralValue::False);
                }
            }
            LiteralValue::StringValue(x) => {
                if x.len() == (0 as usize) {
                    return Ok(LiteralValue::True);
                } else {
                    return Ok(LiteralValue::False);
                }
            }
            LiteralValue::IdentifierValue(_) => {
                todo!();
                // return Err(format!("Identifier is not implemented now ! "));
            }
            LiteralValue::True => {
                return Ok(LiteralValue::False);
            }
            LiteralValue::False => {
                return Ok(LiteralValue::True);
            }
            LiteralValue::Nil => {
                return Ok(LiteralValue::True);
            }
        }
    }

    pub fn to_string(self: &Self) -> String {
        match self {
            LiteralValue::IntValue(value) => {
                return value.to_string();
            }
            LiteralValue::FValue(value) => {
                return value.to_string();
            }
            LiteralValue::StringValue(value) => {
                return value.to_string();
            }
            LiteralValue::IdentifierValue(value) => {
                return value.to_string();
            }
            LiteralValue::True => {
                return "true".to_string();
            }
            LiteralValue::False => {
                return "false".to_string();
            }
            LiteralValue::Nil => {
                return "nil".to_string();
            }
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<LiteralValue>,
    pub(crate) line_number: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        return Self {
            token_type,
            lexeme,
            literal,
            line_number,
        };
    }
}

impl Token {
    #[allow(dead_code)]
    pub fn to_string(self: &Token) -> String {
        match &self.literal {
            Some(literal) => {
                return format!(
                    "{}, {}, {:?}, {}",
                    self.token_type, self.lexeme, literal, self.line_number
                );
            }
            None => {
                return format!(
                    "{}, {}, NO LITERAL !, {}",
                    self.token_type, self.lexeme, self.line_number
                );
            }
        }
    }
}
