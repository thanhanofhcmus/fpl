use crate::ast::{AstNode, Value};
use crate::lexer::Token;
use logos::Lexer;

fn eof_error<T>() -> Result<T, String> {
    Err("no next token".to_string())
}

pub fn parse(lexer: &mut Lexer<'_, Token>) -> Result<AstNode, String> {
    binary(lexer)
}

fn binary(lexer: &mut Lexer<'_, Token>) -> Result<AstNode, String> {
    use Token::*;
    let l = primary(lexer)?;
    let op = extract_token(lexer)?;
    match op {
        Plus | Minus | Star | Slash | And | Or => {}
        _ => return Err(format!("invalid binary op {:?}", op)),
    }
    let r = primary(lexer)?;

    Ok(AstNode::Binary {
        l: Box::new(l),
        op,
        r: Box::new(r),
    })
}

fn primary(lexer: &mut Lexer<'_, Token>) -> Result<AstNode, String> {
    let token = extract_token(lexer)?;

    match token {
        Token::Bool(b) => Ok(AstNode::Primary(Value::Bool(b))),
        Token::Number(n) => Ok(AstNode::Primary(Value::Number(n))),
        Token::Str(s) => Ok(AstNode::Primary(Value::Str(s))),
        t => Err(format!("invlalid token {:?}", t)),
    }
}

fn extract_token(lexer: &mut Lexer<'_, Token>) -> Result<Token, String> {
    match lexer.next() {
        Some(token_result) => token_result.map_err(|e| format!("got lexr error {}", e)),
        None => eof_error(),
    }
}
