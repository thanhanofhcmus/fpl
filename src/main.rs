use parser::parse;

use crate::interpreter::interpret;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

fn main() {
    // let s = r#" "a" == "a" 1 == 1 if true and false then 1 else 2 end true or false 12 + 2 / 3"#;

    repl();
}

fn repl() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line == "quit" {
            return;
        }

        let mut lexer = crate::lexer::PeekableLexer::new(line.as_str());

        let ast = match parse(&mut lexer) {
            Err(err) => {
                eprintln!("parse failed: {}", err);
                continue;
            }
            Ok(a) => a,
        };

        println!("{:?}", &ast);

        let v = match interpret(ast) {
            Err(err) => {
                eprintln!("intepret failed: {}", err);
                continue;
            }
            Ok(a) => a,
        };

        println!("{:?}", v);
    }
}
