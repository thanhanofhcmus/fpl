use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Str(String),
    Fn {
        args: Vec<String>,
        body: Box<AstNode>,
    },
    Nil,
}

#[derive(Debug, Clone)]
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
    Call {
        ident: String,
        args: Vec<AstNode>,
    },
}
