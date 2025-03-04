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
            Token::While => self.while_statement(),
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

        if matches!(
            self.peek(),
            Token::Increment | Token::Decrement | Token::Assign
        ) {
            let expr = match self.advance() {
                Token::Increment => {
                    Expression::Increment(Box::new(Expression::Identifier(name.clone())))
                }
                Token::Decrement => {
                    Expression::Decrement(Box::new(Expression::Identifier(name.clone())))
                }
                Token::Assign => {
                    let value = self.expression()?;
                    Expression::Binary {
                        left: Box::new(Expression::Identifier(name.clone())),
                        operator: Operator::Assign,
                        right: Box::new(value),
                    }
                }
                _ => {
                    return Err("Expected increment, decrement, or assignment operator".to_string())
                }
            };

            if let Token::Semicolon = self.advance() {
                return Ok(Statement::VariableAssignment {
                    name,
                    value: expr,
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
        let then_branch = Box::new(self.block()?);

        let else_branch = if self.peek() == Token::Else {
            self.advance();
            Some(Box::new(self.block()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, String> {
        self.advance();

        let condition = self.expression()?;
        let body = Box::new(self.block()?);

        Ok(Statement::While { condition, body })
    }

    fn block(&mut self) -> Result<Statement, String> {
        let mut statements = Vec::new();

        if self.peek() != Token::LeftBrace {
            return Err("Expected '{' to start block".to_string());
        }
        self.advance();

        while self.peek() != Token::RightBrace {
            statements.push(self.statement()?);
        }

        if self.peek() != Token::RightBrace {
            return Err("Expected '}' to end block".to_string());
        }
        self.advance();

        Ok(Statement::Block(statements))
    }

    fn inline_if(&mut self) -> Result<Expression, String> {
        self.advance();
        let condition = Box::new(self.expression()?);

        if self.peek() != Token::Colon {
            return Err("Expected ':' after if condition".to_string());
        }
        self.advance();

        let then_branch = Box::new(self.expression()?);

        let mut elif_branches = Vec::new();
        let mut else_branch = None;

        while self.peek() == Token::Elif {
            self.advance();
            let elif_condition = Box::new(self.expression()?);

            if self.peek() != Token::Colon {
                return Err("Expected ':' after elif condition".to_string());
            }
            self.advance();

            let elif_branch = Box::new(self.expression()?);
            elif_branches.push((elif_condition, elif_branch));
        }

        if self.peek() == Token::Else {
            self.advance();

            if self.peek() != Token::Colon {
                return Err("Expected ':' after else".to_string());
            }
            self.advance();

            else_branch = Some(Box::new(self.expression()?));
        }

        Ok(Expression::InlineIf {
            condition,
            then_branch,
            elif_branches,
            else_branch: else_branch.unwrap_or(Box::new(Expression::Nil)),
        })
    }

    fn expression(&mut self) -> Result<Expression, String> {
        if self.peek() == Token::If {
            self.inline_if()
        } else {
            self.comparison()
        }
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.term()?;

        while matches!(
            self.peek(),
            Token::EqualEqual
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEqual
                | Token::GreaterThan
                | Token::GreaterThanEqual
        ) {
            let operator = self.parse_operator()?;
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
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
        let mut expr = self.logical()?;

        while matches!(
            self.peek(),
            Token::Star | Token::Slash | Token::Percent | Token::Pow
        ) {
            let operator = self.parse_operator()?;
            let right = self.logical()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;

        while matches!(self.peek(), Token::And | Token::Or) {
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
        if matches!(self.peek(), Token::Not) {
            let operator = self.parse_operator()?;
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

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
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                if self.peek() == Token::RightParen {
                    self.advance();
                    Ok(expr)
                } else {
                    Err("Expected ')' after expression".to_string())
                }
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
            Token::Percent => Ok(Operator::Percent),
            Token::Pow => Ok(Operator::Pow),

            Token::And => Ok(Operator::And),
            Token::Or => Ok(Operator::Or),
            Token::Not => Ok(Operator::Not),

            Token::EqualEqual => Ok(Operator::EqualEqual),
            Token::NotEqual => Ok(Operator::NotEqual),
            Token::LessThan => Ok(Operator::LessThan),
            Token::LessThanEqual => Ok(Operator::LessThanEqual),
            Token::GreaterThan => Ok(Operator::GreaterThan),
            Token::GreaterThanEqual => Ok(Operator::GreaterThanEqual),

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

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Int,
    Boolean,
}
