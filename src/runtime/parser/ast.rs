use super::parser::Type;

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
        value_type: Option<Type>,
    },
    VariableAssignment {
        name: String,
        value: Expression,
        value_type: Option<Type>,
    },
    Print(Expression),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Identifier(String),

    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Operator {
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
}
