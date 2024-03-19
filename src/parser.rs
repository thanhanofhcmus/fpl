use crate::ast::{AstNode, Value};
use crate::lexer::Lexer;
use crate::token::Token;

/*
parse = multi
mutli = aoc ("," aoc)+
aoc = assign | clause
assign = IDENTIFIER "=" clause
clause = if | call | binary
if = "if" clause "do" multi "else" multi "end"
call = IDENTIFIER "(" ARGS ")"
binary = primary (BIN_OP primary)+
primary = NIL, BOOL, NUMBER, STRING, IDENTIFIER, fn
fn = IDENTIFIER ARGS "do" multi "end"
*/

fn eof_error<T>() -> Result<T, String> {
    Err("no next token".to_string())
}

pub type ParseResult = Result<AstNode, String>;

pub fn parse(lexer: &mut Lexer) -> ParseResult {
    multi(lexer)
}

fn multi(lexer: &mut Lexer) -> ParseResult {
    let mut nodes = comma_list(lexer, aoc)?;
    if nodes.len() == 1 {
        Ok(nodes.pop().unwrap())
    } else {
        Ok(AstNode::Multi(nodes))
    }
}

fn aoc(lexer: &mut Lexer) -> ParseResult {
    if let Some(Ok(Token::Equal)) = lexer.peek_two_token() {
        assign(lexer)
    } else {
        clause(lexer)
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
    match lexer.peek_token() {
        Some(Ok(Token::If)) => if_(lexer),
        Some(Ok(Token::Identifier(_))) => match lexer.peek_two_token() {
            Some(Ok(Token::LRoundParen)) => call(lexer),
            _ => binary(lexer),
        },
        _ => binary(lexer),
    }
}

fn if_(lexer: &mut Lexer) -> ParseResult {
    consume_token(lexer, &[Token::If])?;
    let cond = clause(lexer)?;
    consume_token(lexer, &[Token::Do])?;
    let true_ = multi(lexer)?;
    consume_token(lexer, &[Token::Else])?;
    let false_ = multi(lexer)?;
    consume_token(lexer, &[Token::End])?;
    Ok(AstNode::If {
        cond: Box::new(cond),
        true_: Box::new(true_),
        false_: Box::new(false_),
    })
}

fn call(lexer: &mut Lexer) -> ParseResult {
    let ident_token = extract_token(lexer)?;
    let Token::Identifier(ident) = ident_token else {
        return Err(format!("need ident token, got {:?}", ident_token));
    };
    consume_token(lexer, &[Token::LRoundParen])?;
    // TODO: args
    consume_token(lexer, &[Token::RRountParen])?;
    Ok(AstNode::Call {
        ident,
        args: vec![],
    })
}

fn binary(lexer: &mut Lexer) -> ParseResult {
    binary_pratt(lexer, 0)
}

fn binary_pratt(lexer: &mut Lexer, min_bp: u8) -> ParseResult {
    let mut l = primary(lexer)?;
    loop {
        let op = peek_token(lexer)?;
        // TODO: if op is Eof|NewLine return
        let (l_bp, r_bp) = match infix_binding_power(&op) {
            Ok(v) => v,
            Err(e) => {
                dbg!(e);
                break;
            }
        };
        // old left > new left
        if min_bp > l_bp {
            break;
        }
        consume_token(lexer, &[])?; // consume op
        let r = binary_pratt(lexer, r_bp)?;
        l = AstNode::Binary {
            left: Box::new(l),
            op,
            right: Box::new(r),
        }
    }

    Ok(l)
}

fn infix_binding_power(op: &Token) -> Result<(u8, u8), String> {
    use Token::*;
    let bps = match op {
        Plus | Minus => (1, 2),
        Star | Slash => (3, 4),
        And | Or => (5, 6),
        Less | LessEqual | Greater | GreaterEqual => (7, 8),
        EqualEqual | BangEqual => (9, 10),
        _ => return Err(format!("invalid operator for infix binding power {:?}", op)),
    };
    Ok(bps)
}

fn primary(lexer: &mut Lexer) -> ParseResult {
    let token = extract_token(lexer)?;
    let prim = match token {
        Token::Nil => AstNode::Primary(Value::Nil),
        Token::Bool(b) => AstNode::Primary(Value::Bool(b)),
        Token::Number(n) => AstNode::Primary(Value::Number(n)),
        Token::Str(s) => AstNode::Primary(Value::Str(s)),
        Token::Identifier(ident) => AstNode::Variable(ident),
        Token::Fn => fn_decl(lexer)?,
        t => return Err(format!("invlalid token {:?}", t)),
    };
    Ok(prim)
}

fn fn_decl(lexer: &mut Lexer) -> ParseResult {
    // consume_token(lexer, &[Token::Fn])?;
    // TODO: args
    consume_token(lexer, &[Token::Do])?;
    let body = multi(lexer)?;
    consume_token(lexer, &[Token::End])?;
    Ok(AstNode::Primary(Value::Fn {
        args: vec![], // TODO:
        body: Box::new(body),
    }))
}

fn comma_list(
    lexer: &mut Lexer,
    lower_fn: fn(&mut Lexer) -> ParseResult,
) -> Result<Vec<AstNode>, String> {
    let prim = lower_fn(lexer)?;
    let mut nodes = vec![prim];
    while let Ok(Token::Comma) = peek_token(lexer) {
        consume_token(lexer, &[Token::Comma])?;
        let node = lower_fn(lexer)?;
        nodes.push(node);
    }
    Ok(nodes)
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
