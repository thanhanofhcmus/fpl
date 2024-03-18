use crate::ast::{AstNode, Value};
use crate::token::Token;

pub fn interpret(ast: AstNode) -> Result<Value, String> {
    match ast {
        AstNode::Binary { l, op, r } => {
            let le = interpret(*l)?;
            let re = interpret(*r)?;
            match op {
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                    let (Value::Number(a), Value::Number(b)) = (&le, &re) else {
                        return Err(format!("expect number, have {:?}, {:?}", le, re));
                    };
                    let v = match op {
                        Token::Plus => a + b,
                        Token::Minus => a - b,
                        Token::Star => a * b,
                        Token::Slash => a / b,
                        _ => panic!(),
                    };
                    Ok(Value::Number(v))
                }
                Token::And | Token::Or => {
                    let (Value::Bool(a), Value::Bool(b)) = (&le, &re) else {
                        return Err(format!("expect boolean, have {:?}, {:?}", le, re));
                    };
                    let (a, b) = (*a, *b);
                    let v = match op {
                        Token::And => a && b,
                        Token::Or => a || b,
                        _ => panic!(),
                    };
                    Ok(Value::Bool(v))
                }
                _ => Err("invalid op".to_owned()),
            }
        }
        AstNode::If {
            cond,
            true_,
            false_,
        } => {
            let cond = interpret(*cond)?;
            let Value::Bool(c) = &cond else {
                return Err(format!("invalid if condition {:?}", cond));
            };
            let c = *c;
            let v = interpret(if c { *true_ } else { *false_ })?;
            Ok(v)
        }
        AstNode::Primary(v) => Ok(v),
    }
}
