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

    While,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Pow,

    And,
    Or,
    Not,

    EqualEqual,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    Increment,
    Decrement,

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    EOF,
}
