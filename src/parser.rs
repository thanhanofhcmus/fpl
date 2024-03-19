use crate::ast::{AstNode, Value};
use crate::lexer::Lexer;
use crate::token::Token;

fn eof_error<T>() -> Result<T, String> {
    Err("no next token".to_string())
}

pub type ParseResult = Result<AstNode, String>;

pub fn parse(lexer: &mut Lexer) -> ParseResult {
    expr(lexer)
}

pub fn expr(lexer: &mut Lexer) -> ParseResult {
    let token = peek_token(lexer)?;
    match token {
        Token::If => if_(lexer),
        Token::Identifier(_) => {
            if let Some(Ok(Token::Equal)) = lexer.peek_two_token() {
                assign(lexer)
            } else {
                binary(lexer)
            }
        }
        _ => binary(lexer),
    }
}

fn assign(lexer: &mut Lexer) -> ParseResult {
    let ident_token = extract_token(lexer)?;
    let Token::Identifier(ident) = ident_token else {
        return Err(format!("need ident token, got {:?}", ident_token));
    };
    consume_token(lexer, &[Token::Equal])?;
    let clause = clause(lexer)?;
    Ok(AstNode::Assign {
        ident,
        body: Box::new(clause),
    })
}

fn clause(lexer: &mut Lexer) -> ParseResult {
    binary(lexer)
}

fn binary(lexer: &mut Lexer) -> ParseResult {
    use Token::*;
    let l = primary(lexer)?;
    let op = peek_token(lexer)?;
    match op {
        // TODO: precedence
        Plus | Minus | Star | Slash | And | Or | EqualEqual | BangEqual => {
            consume_token(lexer, &[])?;
            let r = primary(lexer)?;
            Ok(AstNode::Binary {
                left: Box::new(l),
                op,
                right: Box::new(r),
            })
        }
        _ => Ok(l),
    }
}

fn primary(lexer: &mut Lexer) -> ParseResult {
    let token = extract_token(lexer)?;
    match token {
        Token::Bool(b) => Ok(AstNode::Primary(Value::Bool(b))),
        Token::Number(n) => Ok(AstNode::Primary(Value::Number(n))),
        Token::Str(s) => Ok(AstNode::Primary(Value::Str(s))),
        Token::Identifier(ident) => Ok(AstNode::Variable(ident)),
        t => Err(format!("invlalid token {:?}", t)),
    }
}

fn if_(lexer: &mut Lexer) -> ParseResult {
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

fn extract_token(lexer: &mut Lexer) -> Result<Token, String> {
    match lexer.next() {
        Some(token_result) => token_result.map_err(|e| format!("got lex extract error {}", e)),
        None => eof_error(),
    }
}

fn peek_token(lexer: &mut Lexer) -> Result<Token, String> {
    match lexer.peek_token() {
        Some(token_result) => token_result.map_err(|e| format!("got lex peek error {:?}", e)),
        None => eof_error(),
    }
}

fn consume_token(lexer: &mut Lexer, expects: &'static [Token]) -> Result<Token, String> {
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
