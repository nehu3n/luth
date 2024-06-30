use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Nil,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
            Ok(())
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }

    pub fn get(&self, name: &String) -> Result<Value, String> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable '{}'.", name))
    }
}
