mod runtime;
mod cli;

fn execute(input: &str) {
    let tokens = runtime::lexer::lexer::lexer(input);
    let mut parser = runtime::parser::parser::Parser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = runtime::interpreter::interpreter::Interpreter::new();
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

    name = 10;

    print(name);

    name = "Juan";
    print(name);
    "#;

    execute(input_dynamic);

    let input_static = r#"
    var name: String = "Nehuén";
    var age: Int = 15;
    var isDev: Bool = true;

    name = 10;

    print(name);

    name = "Juan";
    print(name);
    "#;

    execute(input_static);
}
