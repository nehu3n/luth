use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
};

use crate::runtime::parser::parser::Type;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, (Value, Option<Type>)>,
}

#[derive(Debug, Clone)]
pub enum Value {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Nil,
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::NumberLiteral(lhs), Value::NumberLiteral(rhs)) => {
                Value::NumberLiteral(lhs + rhs)
            }
            _ => Value::Nil,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::NumberLiteral(lhs), Value::NumberLiteral(rhs)) => {
                Value::NumberLiteral(lhs - rhs)
            }
            _ => Value::Nil,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::NumberLiteral(lhs), Value::NumberLiteral(rhs)) => {
                Value::NumberLiteral(lhs * rhs)
            }
            _ => Value::Nil,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::NumberLiteral(lhs), Value::NumberLiteral(rhs)) => {
                Value::NumberLiteral(lhs / rhs)
            }
            _ => Value::Nil,
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value, value_type: Option<Type>) {
        self.variables.insert(name, (value, value_type));
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        if let Some((_, Some(expected_type))) = self.variables.get(&name) {
            match (&value, expected_type) {
                (Value::StringLiteral(_), Type::String) => {
                    self.variables.insert(name, (value, Some(Type::String)));
                    Ok(())
                }
                (Value::NumberLiteral(_), Type::Int) => {
                    self.variables.insert(name, (value, Some(Type::Int)));
                    Ok(())
                }
                (Value::BooleanLiteral(_), Type::Boolean) => {
                    self.variables.insert(name, (value, Some(Type::Boolean)));
                    Ok(())
                }
                _ => Err(format!("Type mismatch for variable '{}'", name)),
            }
        } else if let Some((_, None)) = self.variables.get(&name) {
            self.variables.insert(name, (value, None));
            Ok(())
        } else {
            Err(format!("Variable '{}' not declared", name))
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some((value, _)) = self.variables.get(name) {
            Ok(value.clone())
        } else {
            Err(format!("Undefined variable '{}'", name))
        }
    }
}
