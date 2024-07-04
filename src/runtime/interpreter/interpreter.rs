use crate::runtime::interpreter::environment::{Environment, Value};
use crate::runtime::parser::ast::{Expression, Operator, Statement};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match self.execute(statement) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("{}", error);
                    return;
                }
            }
        }
    }

    fn execute(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::VariableDeclaration {
                name,
                value,
                value_type,
            } => {
                let val = self.evaluate(value);
                self.environment.define(name, val, value_type);
            }
            Statement::VariableAssignment {
                name,
                value,
                value_type: _,
            } => {
                let val = self.evaluate(value);
                self.environment.assign(name, val)?;
            }
            Statement::Print(value) => {
                let val = self.evaluate(value);
                println!("{}", val);
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition).is_truthy() {
                    self.execute(*then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(*else_branch)?;
                }
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::StringLiteral(lit) => Value::StringLiteral(lit),
            Expression::NumberLiteral(num) => Value::NumberLiteral(num),
            Expression::BooleanLiteral(b) => Value::BooleanLiteral(b),
            Expression::Identifier(name) => self.environment.get(&name).unwrap_or(Value::Nil),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left);
                let right = self.evaluate(*right);
                match operator {
                    Operator::Plus => left + right,
                    Operator::Minus => left - right,
                    Operator::Star => left * right,
                    Operator::Slash => left / right,
                }
            }
        }
    }
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::NumberLiteral(n) => *n != 0.0,
            Value::StringLiteral(s) => !s.is_empty(),
            Value::BooleanLiteral(b) => *b,
            Value::Nil => false,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::StringLiteral(s) => write!(f, "{}", s),
            Value::NumberLiteral(n) => write!(f, "{}", n),
            Value::BooleanLiteral(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}
