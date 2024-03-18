use parser::parse;

use crate::interpreter::interpret;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

fn main() {
    let s = r#" "a" == "a" 1 == 1 if true and false then 1 else 2 end true or false 12 + 2 / 3"#;

    let mut lexer = crate::lexer::PeekableLexer::new(s);

    let ast = parse(&mut lexer).unwrap();

    println!("{:?}", &ast);

    let v = interpret(ast).unwrap();

    println!("{:?}", v);
}
