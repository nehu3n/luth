#[derive(Debug, PartialEq)]
pub enum Token {
    Var,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Assign,
    EOF,
}