use std::collections::HashMap;

use crate::scanner::LiteralValue;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(self: &mut Self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(self: &mut Self, name: String) -> Result<LiteralValue, String> {
        match self.values.get(&name) {
            Some(value) => Ok(value.clone()),
            None => Err("Variable declaration not found ".to_string()),
        }
    }
}
