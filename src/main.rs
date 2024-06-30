mod interpreter;
mod lexer;
mod parser;

fn execute(input: &str) {
    let tokens = lexer::lexer::lexer(input);
    let mut parser = parser::parser::Parser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = interpreter::interpreter::Interpreter::new();
            interpreter.interpret(statements);
        }
        Err(error) => eprintln!("{}", error),
    }
}

fn main() {
    let input_dynamic = r#"
    var name = "Nehuén";
    var age = 15;
    var isDev = true;

    name = "Pedro";
    age = 25;
    isDev = false;

    print(age);
    "#;

    execute(input_dynamic);

    let input_static = r#"
    var name: String = "Nehuén";
    var age: Int = 15;
    var isDev: Bool = true;

    name = "Pedro";
    age = 25;
    isDev = false;

    print(name);
    "#;

    execute(input_static);
}
