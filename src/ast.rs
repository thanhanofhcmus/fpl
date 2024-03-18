use crate::token::Token;

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
    If {
        cond: Box<AstNode>,
        true_: Box<AstNode>,
        false_: Box<AstNode>,
    },
}
