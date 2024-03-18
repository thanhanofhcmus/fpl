use lexer::Token;
use logos::Logos;
use parser::parse;

use crate::interpreter::interpret;

mod ast;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let s = "12 + 2 / 3";

    let mut lexer = Token::lexer(s);

    let ast = parse(&mut lexer).unwrap();

    println!("{:?}", &ast);

    let v = interpret(ast).unwrap();

    println!("{:?}", v);
}
