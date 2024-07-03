#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var,
    Identifier(String),
    Assign,

    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    StringType,
    IntType,
    BooleanType,

    Semicolon,
    Colon,

    Print,

    If,
    Else,

    LeftParen,
    RightParen,

    EOF,
}
