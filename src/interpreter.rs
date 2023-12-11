use crate::{errors::Error, statements::Statement};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {};
    }

    pub fn interpret(self: &mut Self, sts: Vec<Statement>) -> Result<(), String> {
        let mut errors: Vec<Error> = Vec::new();
        for st in sts {
            match st {
                Statement::ExpressionStatement(expr) => match expr.evaluate() {
                    Ok(_) => {}
                    Err(err) => {
                        return Err(err.to_string());
                    }
                },
                Statement::PrintStatement(expr) => match expr.evaluate() {
                    Ok(value) => {
                        print!("{}", value.to_string());
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                },
            }
        }
        if errors.len() == 0 {
            Ok(())
        } else {
            let mut err_str = String::new();
            for e in errors {
                err_str += &format!("{}\n", e.to_string());
            }
            return Err(err_str);
        }
    }
}
