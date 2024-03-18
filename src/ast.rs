use crate::lexer::Token;

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Str(String),
}

#[derive(Debug)]
pub enum AstNode {
    Binary {
        l: Box<AstNode>,
        op: Token,
        r: Box<AstNode>,
    },
    Primary(Value),
}
