use crate::lexer::token::Token;
use crate::parser::ast::{Expression, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Token::Var => self.variable_declaration(),
            Token::Identifier(_) => self.variable_assignment(),
            Token::Print => self.print_statement(),
            _ => None,
        }
    }

    fn variable_declaration(&mut self) -> Option<Statement> {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => return None,
        };

        let mut value_type = None;
        if let Token::Colon = self.peek() {
            self.advance();
            match self.advance() {
                Token::StringType => value_type = Some(Type::String),
                Token::IntType => value_type = Some(Type::Int),
                Token::BooleanType => value_type = Some(Type::Boolean),
                _ => {
                    return None;
                }
            }
        }

        if let Token::Assign = self.advance() {
            if let Some(value) = self.expression() {
                if let Token::Semicolon = self.advance() {
                    return Some(Statement::VariableDeclaration {
                        name,
                        value,
                        value_type,
                    });
                }
            }
        }
        None
    }

    fn variable_assignment(&mut self) -> Option<Statement> {
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => return None,
        };

        if let Token::Assign = self.advance() {
            if let Some(value) = self.expression() {
                if let Token::Semicolon = self.advance() {
                    return Some(Statement::VariableAssignment {
                        name,
                        value,
                        value_type: None,
                    });
                }
            }
        }
        None
    }

    fn print_statement(&mut self) -> Option<Statement> {
        self.advance();
        if self.peek() == Token::LeftParen {
            self.advance();
            if let Some(value) = self.expression() {
                if self.peek() == Token::RightParen {
                    self.advance();
                    if self.peek() == Token::Semicolon {
                        self.advance();
                        return Some(Statement::Print(value));
                    }
                }
            }
        }
        None
    }

    fn expression(&mut self) -> Option<Expression> {
        match self.peek() {
            Token::StringLiteral(lit) => {
                self.advance();
                Some(Expression::StringLiteral(lit))
            }
            Token::NumberLiteral(num) => {
                self.advance();
                Some(Expression::NumberLiteral(num))
            }
            Token::BooleanLiteral(b) => {
                self.advance();
                Some(Expression::BooleanLiteral(b))
            }
            Token::Identifier(id) => {
                self.advance();
                Some(Expression::Identifier(id))
            }
            _ => None,
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[derive(Debug)]
pub enum Type {
    String,
    Int,
    Boolean,
}
