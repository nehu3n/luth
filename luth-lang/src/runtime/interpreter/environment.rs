use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Add, Div, Mul, Rem, Sub},
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

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::NumberLiteral(lhs), Value::NumberLiteral(rhs)) => {
                Value::NumberLiteral(lhs % rhs)
            }
            _ => Value::Nil,
        }
    }
}

impl Value {
    pub fn pow(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::NumberLiteral(base), Value::NumberLiteral(exp)) => {
                Value::NumberLiteral(base.powf(*exp))
            }
            _ => Value::Nil,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::StringLiteral(s1), Value::StringLiteral(s2)) => s1 == s2,
            (Value::NumberLiteral(n1), Value::NumberLiteral(n2)) => n1 == n2,
            (Value::BooleanLiteral(b1), Value::BooleanLiteral(b2)) => b1 == b2,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::NumberLiteral(lhs), Value::NumberLiteral(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
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
