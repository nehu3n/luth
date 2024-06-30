mod lexer;
mod parser;

fn main() {
    let input = r#"
    var name = "Nehuén";
    var age = 15;
    var isDev = true;

    name = "Pedro";
    age = 25;
    isDev = false;
    "#;

    let tokens = lexer::lexer::lexer(input);

    let mut parser = parser::parser::Parser::new(tokens);
    let statements = parser.parse();
    for statement in statements {
        println!("{:?}", statement);
    }
}
