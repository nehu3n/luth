#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var,
    Identifier(String),
    StringLiteral(String),
    StringType,
    NumberLiteral(f64),
    IntType,
    BooleanLiteral(bool),
    BooleanType,
    Assign,
    Semicolon,
    Colon,
    EOF,
}