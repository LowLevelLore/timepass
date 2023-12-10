use crate::{
    errors::Error,
    expressions::Expression,
    scanner::{LiteralValue, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser {
            tokens: tokens,
            current: 0,
        };
    }

    fn expression(self: &mut Self) -> Result<Expression, Error> {
        match self.equality() {
            Ok(expr) => return Ok(expr),
            Err(msg) => return Err(msg),
        }
    }

    fn equality(self: &mut Self) -> Result<Expression, Error> {
        match self.comparison() {
            Ok(mut expr) => {
                while self.match_tokens(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
                    let operator: Token = self.previous();
                    match self.comparison() {
                        Ok(right) => {
                            expr = Expression::Binary {
                                left: Box::from(expr),
                                operator: operator.clone(),
                                right: Box::from(right),
                            };
                        }
                        Err(err) => return Err(err),
                    }
                }
                return Ok(expr);
            }
            Err(msg) => {
                return Err(msg);
            }
        }
    }

    fn check(self: &mut Self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            return self.peek().token_type == *token_type;
        }
    }

    fn match_tokens(self: &mut Self, tokens: &[TokenType]) -> bool {
        for typ in tokens {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn advance(self: &mut Self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(self: &Self) -> Token {
        return self.tokens[((self.current as u128) - (1 as u128)) as usize].clone();
    }

    fn peek(self: &mut Self) -> Token {
        return self.tokens[self.current as usize].clone();
    }

    fn is_at_end(self: &mut Self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn comparison(self: &mut Self) -> Result<Expression, Error> {
        match self.term() {
            Ok(mut expr) => {
                while self.match_tokens(&[
                    TokenType::GREATER,
                    TokenType::GREATER_EQUAL,
                    TokenType::LESS,
                    TokenType::LESS_EQUAL,
                ]) {
                    let operator: Token = self.previous().clone();
                    match self.term() {
                        Ok(right) => {
                            expr = Expression::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            }
                        }
                        Err(msg) => return Err(msg),
                    }
                }
                return Ok(expr);
            }
            Err(msg) => return Err(msg),
        }
    }

    fn term(self: &mut Self) -> Result<Expression, Error> {
        match self.factor() {
            Ok(mut expr) => {
                while self.match_tokens(&[TokenType::MINUS, TokenType::PLUS]) {
                    let operator: Token = self.previous().clone();
                    match self.factor() {
                        Ok(right) => {
                            expr = Expression::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            };
                        }
                        Err(msg) => return Err(msg),
                    }
                }
                return Ok(expr);
            }
            Err(msg) => return Err(msg),
        }
    }

    fn factor(self: &mut Self) -> Result<Expression, Error> {
        match self.unary() {
            Ok(mut expr) => {
                while self.match_tokens(&[TokenType::SLASH, TokenType::STAR]) {
                    let operator: Token = self.previous().clone();
                    match self.unary() {
                        Ok(right) => {
                            expr = Expression::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            }
                        }
                        Err(msg) => return Err(msg),
                    }
                }
                return Ok(expr);
            }
            Err(err) => Err(err),
        }
    }

    fn unary(self: &mut Self) -> Result<Expression, Error> {
        if self.match_tokens(&[TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous().clone();
            let right = self.unary();
            match right {
                Ok(value) => {
                    return Ok(Expression::Unary {
                        operator: operator,
                        right: Box::new(value),
                    });
                }
                Err(err) => return Err(err),
            }
        }
        match self.primary() {
            Ok(expr) => return Ok(expr),
            Err(err) => return Err(err),
        }
    }

    fn primary(self: &mut Self) -> Result<Expression, Error> {
        if self.match_tokens(&[TokenType::FALSE]) {
            return Ok(Expression::Literal {
                value: LiteralValue::False,
            });
        } else if self.match_tokens(&[TokenType::TRUE]) {
            return Ok(Expression::Literal {
                value: LiteralValue::True,
            });
        } else if self.match_tokens(&[TokenType::NIL]) {
            return Ok(Expression::Literal {
                value: LiteralValue::Nil,
            });
        } else if self.match_tokens(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expression::Literal {
                value: self.previous().literal.clone().unwrap(),
            });
        } else if self.match_tokens(&[TokenType::LEFT_PAREN]) {
            match self.expression() {
                Ok(expr) => {
                    match self.consume(TokenType::RIGHT_PAREN) {
                        Ok(_) => (),
                        Err(err) => return Err(err),
                    }
                    return Ok(Expression::Grouping {
                        expression: Box::new(expr),
                    });
                }
                Err(msg) => return Err(msg),
            }
        } else {
            todo!();
            // return Ok(Expression::Literal {
            //     value: LiteralValue::IntValue(69),
            // });
        }
    }

    fn consume(self: &mut Self, typ: TokenType) -> Result<Token, Error> {
        if self.check(&typ) {
            return Ok(self.advance().clone());
        } else {
            return Err(Error::UnterminatedParenthesis(self.peek()));
        }
    }

    #[allow(dead_code)]
    fn synchronize(self: &mut Self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            } else {
                match self.peek().token_type {
                    TokenType::CLASS
                    | TokenType::FUN
                    | TokenType::VAR
                    | TokenType::FOR
                    | TokenType::IF
                    | TokenType::WHILE
                    | TokenType::PRINT
                    | TokenType::RETURN => {
                        return;
                    }
                    _ => {
                        ();
                    }
                }
                self.advance();
            }
        }
    }

    pub fn parse(self: &mut Self) -> Result<Expression, Error> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(err) => Err(err),
        }
    }
}
