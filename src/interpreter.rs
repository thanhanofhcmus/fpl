use crate::ast::{AstNode, Value};
use crate::token::Token;

pub fn interpret(ast: AstNode) -> Result<Value, String> {
    match ast {
        AstNode::Binary { l, op, r } => binary(*l, op, *r),
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

fn binary(l: AstNode, op: Token, r: AstNode) -> Result<Value, String> {
    use Token::*;
    let le = interpret(l)?;
    let re = interpret(r)?;
    match op {
        EqualEqual | BangEqual => {
            let mut is_equal = false;
            if let (Value::Bool(a), Value::Bool(b)) = (&le, &re) {
                is_equal = a == b;
            }
            if let (Value::Number(a), Value::Number(b)) = (&le, &re) {
                is_equal = a == b;
            }
            if let (Value::Str(a), Value::Str(b)) = (&le, &re) {
                is_equal = a == b;
            }
            Ok(Value::Bool((op == BangEqual) ^ is_equal))
        }
        Plus | Minus | Star | Slash => {
            let (Value::Number(a), Value::Number(b)) = (&le, &re) else {
                return Err(format!("expect two number, have {:?}, {:?}", le, re));
            };
            let v = match op {
                Plus => a + b,
                Minus => a - b,
                Star => a * b,
                Slash => a / b,
                _ => panic!(),
            };
            Ok(Value::Number(v))
        }
        And | Or => {
            let (Value::Bool(a), Value::Bool(b)) = (&le, &re) else {
                return Err(format!("expect two booleans, have {:?}, {:?}", le, re));
            };
            let (a, b) = (*a, *b);
            let v = match op {
                And => a && b,
                Or => a || b,
                _ => panic!(),
            };
            Ok(Value::Bool(v))
        }
        _ => Err("invalid op".to_owned()),
    }
}
