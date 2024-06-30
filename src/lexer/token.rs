#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Assign,
    Semicolon,
    EOF,
}