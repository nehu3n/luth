use crate::runtime::lexer::token::Token;
use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum LexerToken {
    #[regex(r"[ \t\r\x0c]+", logos::skip)]
    Ignored,

    #[token("var")]
    Var,

    #[token("String")]
    StringType,

    #[token("Int")]
    IntType,

    #[token("Bool")]
    BooleanType,

    #[regex("[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^\n"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();

        slice[1..slice.len()-1].to_string()
    })]
    StringLiteral(String),

    #[regex(r"-?\d+(\.\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    NumberLiteral(f64),

    #[regex("true|false", |lex| lex.slice().parse::<bool>().unwrap())]
    BooleanLiteral(bool),

    #[token("=")]
    Assign,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token("print")]
    Print,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[end]
    EOF,
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut lexer = LexerToken::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(LexerToken::Ignored) => continue,
            Ok(LexerToken::Var) => tokens.push(Token::Var),
            Ok(LexerToken::StringType) => tokens.push(Token::StringType),
            Ok(LexerToken::IntType) => tokens.push(Token::IntType),
            Ok(LexerToken::BooleanType) => tokens.push(Token::BooleanType),
            Ok(LexerToken::Identifier(id)) => tokens.push(Token::Identifier(id)),
            Ok(LexerToken::StringLiteral(lit)) => tokens.push(Token::StringLiteral(lit)),
            Ok(LexerToken::NumberLiteral(num)) => tokens.push(Token::NumberLiteral(num)),
            Ok(LexerToken::BooleanLiteral(b)) => tokens.push(Token::BooleanLiteral(b)),
            Ok(LexerToken::Semicolon) => tokens.push(Token::Semicolon),
            Ok(LexerToken::Assign) => tokens.push(Token::Assign),
            Ok(LexerToken::Colon) => tokens.push(Token::Colon),
            Ok(LexerToken::EOF) => tokens.push(Token::EOF),
            Ok(LexerToken::Print) => tokens.push(Token::Print),
            Ok(LexerToken::LeftParen) => tokens.push(Token::LeftParen),
            Ok(LexerToken::RightParen) => tokens.push(Token::RightParen),

            Err(_) => continue,
        }
    }

    tokens
}
