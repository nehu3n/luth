use crate::interpreter::environment::{Environment, Value};
use crate::parser::ast::{Expression, Statement};

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
            self.execute(statement);
        }
    }

    fn execute(&mut self, statement: Statement) {
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
                value_type,
            } => {
                let val = self.evaluate(value);
                match self.environment.assign(name, val) {
                    Ok(()) => (),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Statement::Print(value) => {
                let val = self.evaluate(value);
                print!("{}", val);
            }
        }
    }

    fn evaluate(&self, expression: Expression) -> Value {
        match expression {
            Expression::StringLiteral(lit) => Value::StringLiteral(lit),
            Expression::NumberLiteral(num) => Value::NumberLiteral(num),
            Expression::BooleanLiteral(b) => Value::BooleanLiteral(b),
            Expression::Identifier(id) => match self.environment.get(&id) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("{}", err);
                    Value::Nil
                }
            },
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
