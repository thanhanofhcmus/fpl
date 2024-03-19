use std::collections::HashMap;

use crate::ast::{AstNode, Value};
use crate::token::Token;

#[derive(Default)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

pub fn interpret(env: &mut Environment, ast: AstNode) -> Result<Value, String> {
    match ast {
        AstNode::Binary { left, op, right } => binary(env, *left, op, *right),
        AstNode::If {
            cond,
            true_,
            false_,
        } => {
            let cond = interpret(env, *cond)?;
            let Value::Bool(c) = &cond else {
                return Err(format!("invalid if condition {:?}", cond));
            };
            let c = *c;
            let v = interpret(env, if c { *true_ } else { *false_ })?;
            Ok(v)
        }
        AstNode::Primary(v) => Ok(v),
        AstNode::Variable(ident) => Ok(env
            .variables
            .get(&ident)
            .map(|v| v.to_owned())
            .unwrap_or(Value::Nil)),
        AstNode::Assign { ident, body } => {
            let expr = interpret(env, *body)?;
            env.variables.insert(ident, expr);
            // TODO
            Ok(Value::Nil)
        }
        AstNode::Call { ident, args } => {
            let Some(Value::Fn { args: f_args, body }) = env.variables.get(&ident) else {
                return Ok(Value::Nil);
            };
            // TODO: new env
            _ = f_args;
            _ = args;
            interpret(env, *body.to_owned())
        }
    }
}

fn binary(env: &mut Environment, l: AstNode, op: Token, r: AstNode) -> Result<Value, String> {
    use Token::*;
    let le = interpret(env, l)?;
    let re = interpret(env, r)?;
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
