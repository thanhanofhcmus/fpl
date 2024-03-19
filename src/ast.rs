use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Str(String),
    Nil,
}

#[derive(Debug)]
pub enum AstNode {
    Primary(Value),
    Variable(String),
    Binary {
        left: Box<AstNode>,
        op: Token,
        right: Box<AstNode>,
    },
    If {
        cond: Box<AstNode>,
        true_: Box<AstNode>,
        false_: Box<AstNode>,
    },
    Assign {
        ident: String,
        body: Box<AstNode>,
    },
}
