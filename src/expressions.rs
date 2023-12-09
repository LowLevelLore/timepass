use crate::scanner::{LiteralValue, Token};
#[allow(dead_code)]
// impl LiteralValue {
//     #[allow(dead_code)]
//     pub fn to_string(self: &Self) -> String {
//         match self {
//             LiteralValue::IntValue(x) => x.to_string(),
//             LiteralValue::FValue(x) => x.to_string(),
//             LiteralValue::StringValue(x) => x.clone(),

//             LiteralValue::True => "True".to_string(),
//             LiteralValue::False => "False".to_string(),
//             LiteralValue::Nil => "NIL".to_string(),
//         }
//     }
// }
#[allow(dead_code)]
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
}
