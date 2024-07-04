use crate::runtime::lexer::token::Token;
use crate::runtime::parser::ast::{Expression, Operator, Statement};

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
            if let Ok(stmt) = self.statement() {
                statements.push(stmt);
            }
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Var => self.variable_declaration(),
            Token::Identifier(_) => self.variable_assignment(),
            Token::Print => self.print_statement(),
            Token::If => self.if_statement(),
            _ => Err("Unexpected token in statement".to_string()),
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, String> {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => return Err("Expected variable name".to_string()),
        };

        let mut value_type = None;
        if let Token::Colon = self.peek() {
            self.advance();
            value_type = Some(self.parse_type()?);
        }

        if let Token::Assign = self.advance() {
            let value = self.expression()?;
            if let Token::Semicolon = self.advance() {
                return Ok(Statement::VariableDeclaration {
                    name,
                    value,
                    value_type,
                });
            }
        }
        Err("Invalid variable declaration".to_string())
    }

    fn variable_assignment(&mut self) -> Result<Statement, String> {
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => return Err("Expected variable name".to_string()),
        };

        if let Token::Assign = self.advance() {
            let value = self.expression()?;
            if let Token::Semicolon = self.advance() {
                return Ok(Statement::VariableAssignment {
                    name,
                    value,
                    value_type: None,
                });
            }
        }
        Err("Invalid variable assignment".to_string())
    }

    fn print_statement(&mut self) -> Result<Statement, String> {
        self.advance();
        if self.peek() == Token::LeftParen {
            self.advance();
            let value = self.expression()?;
            if self.peek() == Token::RightParen {
                self.advance();
                if self.peek() == Token::Semicolon {
                    self.advance();
                    return Ok(Statement::Print(value));
                }
            }
        }
        Err("Invalid print statement".to_string())
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        self.advance();

        let condition = self.expression()?;
        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.peek() == Token::Else {
            self.advance();
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.term()
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;

        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let operator = self.parse_operator()?;
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;

        while matches!(self.peek(), Token::Star | Token::Slash) {
            let operator = self.parse_operator()?;
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, String> {
        match self.peek() {
            Token::StringLiteral(lit) => {
                self.advance();
                Ok(Expression::StringLiteral(lit))
            }
            Token::NumberLiteral(num) => {
                self.advance();
                Ok(Expression::NumberLiteral(num))
            }
            Token::BooleanLiteral(b) => {
                self.advance();
                Ok(Expression::BooleanLiteral(b))
            }
            Token::Identifier(id) => {
                self.advance();
                Ok(Expression::Identifier(id))
            }
            _ => Err("Unexpected token in expression".to_string()),
        }
    }

    fn parse_operator(&mut self) -> Result<Operator, String> {
        match self.advance() {
            Token::Plus => Ok(Operator::Plus),
            Token::Minus => Ok(Operator::Minus),
            Token::Star => Ok(Operator::Star),
            Token::Slash => Ok(Operator::Slash),
            _ => Err("Unexpected token in operator".to_string()),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.advance() {
            Token::StringType => Ok(Type::String),
            Token::IntType => Ok(Type::Int),
            Token::BooleanType => Ok(Type::Boolean),
            _ => Err("Unexpected token in type".to_string()),
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
