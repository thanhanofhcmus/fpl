use crate::ast::{AstNode, Value};
use crate::lexer::Token;

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
                _ => return Err("invalid op".to_owned()),
            }
        }
        AstNode::Primary(v) => Ok(v),
    }
}
