use crate::{expressions::Expression, scanner::Token};

#[derive(PartialEq)]
pub enum Statement {
    ExpressionStatement(Expression),
    PrintStatement(Expression),
    Variable(Token, Expression),
}

impl Statement {}
