use crate::runtime;

use super::utils::read_file;

pub fn run_action(file_path: &str) {
    let input = read_file(file_path);

    let tokens = runtime::lexer::lexer::lexer(&input);
    let mut parser = runtime::parser::parser::Parser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = runtime::interpreter::interpreter::Interpreter::new();
            interpreter.interpret(statements);
        }
        Err(error) => eprintln!("{}", error),
    }
}
