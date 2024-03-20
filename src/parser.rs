use crate::ast::{AstNode, Value};
use crate::lexer::Lexer;
use crate::token::Token;

/*
parse = expr
expr = single | multi
mutli = "do" (aoc ",")* "end"
single = aoc
aoc = assign | clause
assign = IDENTIFIER "=" clause
clause = binary
binary = primary (BIN_OP primary)+
binary_lower = if | when | primary
if = "if" clause "then" expr "else" expr
when = "when" (clause "then" expr)*
primary = NIL | BOOL | NUMBER | STRING | IDENTIFIER, fn_decl | call
call = IDENTIFIER "(" ARGS ")"
fn_decl = "fn" PARAMS -> expr

when true then 1 <MARKER> false then <MARKER> end

f = fn n -> n + 1
f = fn n -> do a = n + 1, a * 2 end
if true then 1 else 2
if a == 1 then do a + 1 end else do a - 2 end
when
    c1 -> a1,
    c2 -> do
        t1, t2
    end,
    c3 -> a3,
    c4 -> do
        t5
    end

when c1 -> a1, c2 -> do t1, t2 end, c3 -> a3, c4 -> do t5 end
when c1 then a1, c2 then do t1, t2 end, c3 then a3, c4 then do t5 end
when c1 then a1 c2 then a2 c3 then a3

1 + if true then 2 else 3 * 3

when
    a then 2,
    b then 1,
end
*/

pub type ParseResult = Result<AstNode, String>;

pub fn parse(lexer: &mut Lexer) -> ParseResult {
    expr(lexer)
}

fn expr(lexer: &mut Lexer) -> ParseResult {
    if let Some(Ok(Token::Do)) = lexer.peek_token() {
        multi(lexer)
    } else {
        single(lexer)
    }
}

fn multi(lexer: &mut Lexer) -> ParseResult {
    consume_token(lexer, &[Token::Do])?;
    let nodes = comma_list(lexer, aoc, Token::End)?;
    consume_token(lexer, &[Token::End])?;
    Ok(AstNode::Multi(nodes))
}

fn single(lexer: &mut Lexer) -> ParseResult {
    aoc(lexer)
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
    binary(lexer)
}

fn if_(lexer: &mut Lexer) -> ParseResult {
    consume_token(lexer, &[Token::If])?;
    let cond = clause(lexer)?;
    consume_token(lexer, &[Token::Then])?;
    let true_ = multi(lexer)?;
    consume_token(lexer, &[Token::Else])?;
    let false_ = multi(lexer)?;
    Ok(AstNode::If {
        cond: Box::new(cond),
        true_: Box::new(true_),
        false_: Box::new(false_),
    })
}

fn when(lexer: &mut Lexer) -> ParseResult {
    consume_token(lexer, &[Token::When])?;
    let cases = comma_list(lexer, when_lower, Token::End)?;
    // TODO: conider remove end at the end of a when clause
    consume_token(lexer, &[Token::End])?;
    Ok(AstNode::When(cases))
}

fn when_lower(lexer: &mut Lexer) -> Result<(AstNode, AstNode), String> {
    let cond = clause(lexer)?;
    consume_token(lexer, &[Token::RThinArr])?;
    let body = multi(lexer)?;
    Ok((cond, body))
}

fn call(lexer: &mut Lexer) -> ParseResult {
    let ident_token = extract_token(lexer)?;
    let Token::Identifier(ident) = ident_token else {
        return Err(format!("need ident token, got {:?}", ident_token));
    };
    consume_token(lexer, &[Token::LRoundParen])?;
    let args = comma_list(lexer, clause, Token::RRountParen)?;
    consume_token(lexer, &[Token::RRountParen])?;
    Ok(AstNode::Call { ident, args })
}

fn binary(lexer: &mut Lexer) -> ParseResult {
    binary_pratt(lexer, 0)
}

fn binary_pratt(lexer: &mut Lexer, min_bp: u8) -> ParseResult {
    // TODO: unary not, -
    let mut l = binary_lower(lexer)?;
    loop {
        let op = match lexer.peek_token() {
            Some(Ok(t)) => t,
            Some(Err(err)) => return Err(err),
            None => break,
        };
        // TODO: if op is Eof|NewLine return
        let Some((l_bp, r_bp)) = infix_binding_power(&op) else {
            break;
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

fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    use Token::*;
    let bps = match op {
        Plus | Minus => (1, 2),
        Star | Slash => (3, 4),
        And | Or => (5, 6),
        Less | LessEqual | Greater | GreaterEqual => (7, 8),
        EqualEqual | BangEqual => (9, 10),
        // _ => return Err(format!("invalid operator for infix binding power {:?}", op)),
        _ => return None,
    };
    Some(bps)
}

fn binary_lower(lexer: &mut Lexer) -> ParseResult {
    match lexer.peek_token() {
        Some(Ok(Token::If)) => if_(lexer),
        Some(Ok(Token::When)) => when(lexer),
        _ => primary(lexer),
    }
}

fn primary(lexer: &mut Lexer) -> ParseResult {
    match lexer.peek_token() {
        Some(Ok(Token::Fn)) => return fn_decl(lexer),
        Some(Ok(Token::Identifier(ident))) => {
            if let Some(Ok(Token::LRoundParen)) = lexer.peek_two_token() {
                return call(lexer);
            } else {
                extract_token(lexer)?; //consume ident token
                return Ok(AstNode::Variable(ident));
            }
        }
        _ => {}
    }
    let token = extract_token(lexer)?;
    let prim = match token {
        Token::Nil => Value::Nil,
        Token::Bool(b) => Value::Bool(b),
        Token::Number(n) => Value::Number(n),
        Token::Str(s) => Value::Str(s),
        t => return Err(format!("invlalid token {:?}", t)),
    };
    Ok(AstNode::Primary(prim))
}

fn fn_decl(lexer: &mut Lexer) -> ParseResult {
    consume_token(lexer, &[Token::Fn])?;
    let params = comma_list(lexer, extract_ident, Token::RThinArr)?;
    consume_token(lexer, &[Token::RThinArr])?;
    let body = expr(lexer)?;
    Ok(AstNode::Primary(Value::Fn {
        params,
        body: Box::new(body),
    }))
}

fn comma_list<T>(
    lexer: &mut Lexer,
    lower_fn: fn(&mut Lexer) -> Result<T, String>,
    end_token: Token,
) -> Result<Vec<T>, String> {
    if lexer.peek_token() == Some(Ok(end_token)) {
        return Ok(vec![]);
    }
    let prim = lower_fn(lexer)?;
    let mut nodes = vec![prim];
    loop {
        match lexer.peek_token() {
            Some(Ok(Token::Comma)) => {
                consume_token(lexer, &[Token::Comma])?;
                let node = lower_fn(lexer)?;
                nodes.push(node);
            }
            // Some(Ok(t)) if t == end_token
            Some(Ok(_)) | None => break,
            Some(Err(err)) => return Err(err),
            // TODO trailing comma
        }
    }
    Ok(nodes)
}

fn extract_ident(lexer: &mut Lexer) -> Result<String, String> {
    let token = extract_token(lexer)?;
    if let Token::Identifier(ident) = token {
        Ok(ident)
    } else {
        Err(format!("expect identifier token, got {:?}", token))
    }
}

fn extract_token(lexer: &mut Lexer) -> Result<Token, String> {
    match lexer.next() {
        Some(token_result) => token_result.map_err(|e| format!("got lex extract error {}", e)),
        None => Err("no next token".to_string()),
    }
}

fn consume_token(lexer: &mut Lexer, expects: &'static [Token]) -> Result<Token, String> {
    let token = extract_token(lexer)?;
    if expects.is_empty() || expects.contains(&token) {
        Ok(token)
    } else {
        Err(format!(
            "unexpected when consume token, expect {:?}, got {:?}",
            expects, token
        ))
    }
}
