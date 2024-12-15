use super::parser::Type;

#[derive(Debug, Clone)]
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
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
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
    Increment(Box<Expression>),
    Decrement(Box<Expression>),
    InlineIf {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        elif_branches: Vec<(Box<Expression>, Box<Expression>)>,
        else_branch: Box<Expression>,
    },
    Nil,
}

#[derive(Debug, Clone)]
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

    Assign,
}
