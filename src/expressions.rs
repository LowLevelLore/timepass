use crate::scanner::{LiteralValue, Token, TokenType};

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: LiteralValue,
    },
}

impl Expression {
    #[allow(dead_code)]
    pub fn to_string(self: &Self) -> String {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                return format!(
                    "({} {} {})",
                    operator.lexeme.clone().to_string(),
                    left.to_string(),
                    right.to_string()
                )
                .to_string();
            }
            Expression::Unary { operator, right } => {
                return format!("({} {})", operator.lexeme.clone(), (*right).to_string())
                    .to_string();
            }
            Expression::Grouping { expression } => {
                return format!("(GROUP : {})", expression.to_string()).to_string();
            }
            Expression::Literal { value } => {
                return format!("{}", value.to_string()).to_string();
            }
        }
    }

    // Task for tomorrow : Define edge cases showing more plausible errors
    // Check for division by zero
    pub fn evaluate(self: &Self) -> Result<LiteralValue, String> {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (left, operator.token_type.clone(), right) {
                    // ARITHMETIC
                    (LiteralValue::IntValue(x), TokenType::PLUS, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::IntValue(x + y));
                    }
                    (LiteralValue::FValue(x), TokenType::PLUS, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::FValue(x + (y as f64)));
                    }
                    (LiteralValue::IntValue(x), TokenType::PLUS, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue((x as f64) + y));
                    }
                    (LiteralValue::FValue(x), TokenType::PLUS, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue(x + y));
                    }
                    (LiteralValue::IntValue(x), TokenType::MINUS, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::IntValue(x - y));
                    }
                    (LiteralValue::FValue(x), TokenType::MINUS, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::FValue(x - (y as f64)));
                    }
                    (LiteralValue::IntValue(x), TokenType::MINUS, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue((x as f64) - y));
                    }
                    (LiteralValue::FValue(x), TokenType::MINUS, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue(x - y));
                    }
                    (LiteralValue::IntValue(x), TokenType::STAR, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::IntValue(x * y));
                    }
                    (LiteralValue::FValue(x), TokenType::STAR, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::FValue(x * (y as f64)));
                    }
                    (LiteralValue::IntValue(x), TokenType::STAR, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue((x as f64) * y));
                    }
                    (LiteralValue::FValue(x), TokenType::STAR, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue(x * y));
                    }
                    (LiteralValue::IntValue(x), TokenType::SLASH, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::FValue((x as f64 / y as f64) as f64));
                    }
                    (LiteralValue::FValue(x), TokenType::SLASH, LiteralValue::IntValue(y)) => {
                        return Ok(LiteralValue::FValue(x / (y as f64)));
                    }
                    (LiteralValue::IntValue(x), TokenType::SLASH, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue((x as f64) / y));
                    }
                    (LiteralValue::FValue(x), TokenType::SLASH, LiteralValue::FValue(y)) => {
                        return Ok(LiteralValue::FValue(x / y));
                    }
                    (LiteralValue::StringValue(_), TokenType::PLUS, LiteralValue::IntValue(_)) => {
                        return Err(format!(
                            "Invalid operation '+' between string and integer ! "
                        ));
                    }
                    (LiteralValue::IntValue(_), TokenType::PLUS, LiteralValue::StringValue(_)) => {
                        return Err(format!(
                            "Invalid operation '+' between string and integer ! "
                        ));
                    }
                    (LiteralValue::StringValue(_), TokenType::PLUS, LiteralValue::FValue(_)) => {
                        return Err(format!("Invalid operation '+' between string and float ! "));
                    }
                    (LiteralValue::FValue(_), TokenType::PLUS, LiteralValue::StringValue(_)) => {
                        return Err(format!("Invalid operation '+' between string and float ! "));
                    }
                    (LiteralValue::StringValue(x), TokenType::STAR, LiteralValue::IntValue(y)) => {
                        let mut ans = String::new();
                        for _ in 0..y {
                            ans.push_str(&x);
                        }
                        return Ok(LiteralValue::StringValue(ans));
                    }
                    (LiteralValue::IntValue(x), TokenType::STAR, LiteralValue::StringValue(y)) => {
                        let mut ans = String::new();
                        for _ in 0..x {
                            ans.push_str(&y);
                        }
                        return Ok(LiteralValue::StringValue(ans));
                    }
                    (LiteralValue::StringValue(_), TokenType::STAR, LiteralValue::FValue(_)) => {
                        return Err(format!("Invalid operation '*' between string and float ! "));
                    }
                    (LiteralValue::FValue(_), TokenType::STAR, LiteralValue::StringValue(_)) => {
                        return Err(format!("Invalid operation '*' between string and float ! "));
                    }
                    (
                        LiteralValue::StringValue(x),
                        TokenType::PLUS,
                        LiteralValue::StringValue(y),
                    ) => {
                        return Ok(LiteralValue::StringValue(format!("{}{}", x, y)));
                    }
                    (LiteralValue::IntValue(x), TokenType::GREATER, LiteralValue::IntValue(y)) => {
                        if x > y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::GREATER, LiteralValue::IntValue(y)) => {
                        if x > (y as f64) {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::IntValue(x), TokenType::GREATER, LiteralValue::FValue(y)) => {
                        if (x as f64) > y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::GREATER, LiteralValue::FValue(y)) => {
                        if x > y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::IntValue(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::IntValue(y),
                    ) => {
                        if x >= y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::FValue(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::IntValue(y),
                    ) => {
                        if x >= (y as f64) {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::IntValue(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::FValue(y),
                    ) => {
                        if (x as f64) >= y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::FValue(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::FValue(y),
                    ) => {
                        if x >= y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::IntValue(x), TokenType::LESS, LiteralValue::IntValue(y)) => {
                        if x < y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::LESS, LiteralValue::IntValue(y)) => {
                        if x < (y as f64) {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::IntValue(x), TokenType::LESS, LiteralValue::FValue(y)) => {
                        if (x as f64) < y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::LESS, LiteralValue::FValue(y)) => {
                        if x < y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::IntValue(x),
                        TokenType::LESS_EQUAL,
                        LiteralValue::IntValue(y),
                    ) => {
                        if x <= y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::LESS_EQUAL, LiteralValue::IntValue(y)) => {
                        if x <= (y as f64) {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::IntValue(x), TokenType::LESS_EQUAL, LiteralValue::FValue(y)) => {
                        if (x as f64) <= y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::LESS_EQUAL, LiteralValue::FValue(y)) => {
                        if x <= y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::IntValue(x),
                        TokenType::BANG_EQUAL,
                        LiteralValue::IntValue(y),
                    ) => {
                        if x != y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::BANG_EQUAL, LiteralValue::IntValue(y)) => {
                        if x != (y as f64) {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::IntValue(x), TokenType::BANG_EQUAL, LiteralValue::FValue(y)) => {
                        if (x as f64) != y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::BANG_EQUAL, LiteralValue::FValue(y)) => {
                        if x != y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::IntValue(x),
                        TokenType::EQUAL_EQUAL,
                        LiteralValue::IntValue(y),
                    ) => {
                        if x == y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::FValue(x),
                        TokenType::EQUAL_EQUAL,
                        LiteralValue::IntValue(y),
                    ) => {
                        if x == (y as f64) {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::IntValue(x),
                        TokenType::EQUAL_EQUAL,
                        LiteralValue::FValue(y),
                    ) => {
                        if (x as f64) == y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (LiteralValue::FValue(x), TokenType::EQUAL_EQUAL, LiteralValue::FValue(y)) => {
                        if x == y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::StringValue(x),
                        TokenType::BANG_EQUAL,
                        LiteralValue::StringValue(y),
                    ) => {
                        if x != y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    (
                        LiteralValue::StringValue(x),
                        TokenType::EQUAL_EQUAL,
                        LiteralValue::StringValue(y),
                    ) => {
                        if x == y {
                            return Ok(LiteralValue::True);
                        } else {
                            return Ok(LiteralValue::False);
                        }
                    }
                    _ => {
                        return Err("OPERATION NOT IMPLEMENTED !".to_string());
                    }
                }
            }
            Expression::Unary { operator, right } => {
                let right = right.evaluate()?;
                match (right.clone(), operator.token_type.clone()) {
                    (LiteralValue::IntValue(x), TokenType::MINUS) => {
                        return Ok(LiteralValue::IntValue(-x));
                    }
                    (_, TokenType::MINUS) => {
                        return Err(format!(
                            "Minus is not implemented for {}",
                            right.to_string()
                        ));
                    }
                    (mut any, TokenType::BANG) => {
                        return Ok(any.is_falsy().expect("SOMETHING HAPPENED"));
                    }
                    _ => {
                        todo!();
                    }
                }
            }
            Expression::Grouping { expression } => {
                return expression.evaluate();
            }
            Expression::Literal { value } => {
                return Ok(value.clone());
            }
        }
    }
}
