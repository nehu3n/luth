#[derive(Debug)]
pub enum Statement {
    VariableDeclaration { name: String, value: Expression },
    VariableAssignment { name: String, value: Expression },
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Identifier(String),
}
