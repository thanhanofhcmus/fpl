use crate::ast::{AstNode, Value};
use crate::lexer::PeekableLexer;
use crate::token::Token;

fn eof_error<T>() -> Result<T, String> {
    Err("no next token".to_string())
}

pub fn parse(lexer: &mut PeekableLexer) -> Result<AstNode, String> {
    expr(lexer)
}

pub fn expr(lexer: &mut PeekableLexer) -> Result<AstNode, String> {
    let token = peek_token(lexer)?;
    match token {
        Token::If => if_(lexer),
        _ => binary(lexer),
    }
}

fn binary(lexer: &mut PeekableLexer) -> Result<AstNode, String> {
    use Token::*;
    let l = primary(lexer)?;
    let op = peek_token(lexer)?;
    match op {
        // TODO: precedence
        Plus | Minus | Star | Slash | And | Or | EqualEqual | BangEqual => {
            consume_token(lexer, &[])?;
            let r = primary(lexer)?;
            Ok(AstNode::Binary {
                l: Box::new(l),
                op,
                r: Box::new(r),
            })
        }
        _ => Ok(l),
    }
}

fn primary(lexer: &mut PeekableLexer) -> Result<AstNode, String> {
    let token = extract_token(lexer)?;
    match token {
        Token::Bool(b) => Ok(AstNode::Primary(Value::Bool(b))),
        Token::Number(n) => Ok(AstNode::Primary(Value::Number(n))),
        Token::Str(s) => Ok(AstNode::Primary(Value::Str(s))),
        t => Err(format!("invlalid token {:?}", t)),
    }
}

fn if_(lexer: &mut PeekableLexer) -> Result<AstNode, String> {
    consume_token(lexer, &[Token::If])?;
    let cond = expr(lexer)?;
    consume_token(lexer, &[Token::Then])?;
    let true_ = expr(lexer)?;
    consume_token(lexer, &[Token::Else])?;
    let false_ = expr(lexer)?;
    consume_token(lexer, &[Token::End])?;
    Ok(AstNode::If {
        cond: Box::new(cond),
        true_: Box::new(true_),
        false_: Box::new(false_),
    })
}

fn extract_token(lexer: &mut PeekableLexer) -> Result<Token, String> {
    match lexer.next() {
        Some(token_result) => token_result.map_err(|e| format!("got lex extract error {}", e)),
        None => eof_error(),
    }
}

fn peek_token(lexer: &mut PeekableLexer) -> Result<Token, String> {
    match lexer.peek() {
        Some(token_result) => token_result.map_err(|e| format!("got lex peek error {:?}", e)),
        None => eof_error(),
    }
}

fn consume_token(lexer: &mut PeekableLexer, expects: &'static [Token]) -> Result<Token, String> {
    let token = extract_token(lexer)?;
    if expects.is_empty() || expects.contains(&token) {
        Ok(token)
    } else {
        Err(format!(
            "unexpected token, expect {:?}, got {:?}",
            expects, token
        ))
    }
}
