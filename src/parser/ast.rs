use super::parser::Type;

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration { name: String, value: Expression, value_type: Option<Type> },
    VariableAssignment { name: String, value: Expression, value_type: Option<Type> },
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Identifier(String),
}
