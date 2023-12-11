use crate::expressions::Expression;

pub enum Statement {
    ExpressionStatement(Expression),
    PrintStatement(Expression),
}

impl Statement {}
