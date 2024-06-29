mod lexer;

fn main() {
    let input = r#"
    var name = "Nehuén"
    var age = 15
    var isDev = true

    name = "Pedro"
    age = 25
    isDev = false
    "#;

    let tokens = lexer::lexer::lexer(input);
    for token in tokens {
        println!("{:?}", token);
    }
}
