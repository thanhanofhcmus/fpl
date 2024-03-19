use std::collections::HashMap;
use std::iter::zip;

use crate::ast::{AstNode, Value};
use crate::token::Token;

#[derive(Default)]
pub struct Environment<'pa> {
    variables: HashMap<String, Value>,
    parent: Option<&'pa Environment<'pa>>,
}

impl<'pa> Environment<'pa> {
    fn with_parent(parent: &'pa Environment<'pa>) -> Self {
        Self {
            variables: HashMap::default(),
            parent: Some(parent),
        }
    }

    fn get_local(&self, ident: &'_ str) -> Option<Value> {
        self.variables.get(ident).map(|t| t.to_owned())
    }

    fn get_with_parent(&self, ident: &'_ str) -> Option<Value> {
        self.get_local(ident)
            .or_else(|| self.parent.and_then(|pa| pa.get_with_parent(ident)))
    }
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
        AstNode::Variable(ident) => Ok(env.get_with_parent(&ident).unwrap_or(Value::Nil)),
        AstNode::Multi(mut nodes) => {
            // nodes is sure to have aleast one value in this state
            let last_node = nodes.pop().unwrap();
            for node in nodes {
                interpret(env, node)?;
            }
            interpret(env, last_node)
        }
        AstNode::Assign { ident, body } => {
            let expr = interpret(env, *body)?;
            env.variables.insert(ident, expr);
            // TODO
            Ok(Value::Nil)
        }
        AstNode::Call { ident, args } => {
            let Some(Value::Fn { params, body }) = env.get_with_parent(&ident) else {
                return Ok(Value::Nil);
            };
            if params.len() != args.len() {
                return Err(format!(
                    "invalid arity, want {} but got {}",
                    params.len(),
                    args.len(),
                ));
            }
            let mut child_env = Environment::with_parent(env);
            for (p, a) in zip(params, args) {
                // TODO: maybe interpret(env, a)
                let v = interpret(&mut child_env, a)?;
                child_env.variables.insert(p, v);
            }
            interpret(&mut child_env, *body.to_owned())
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
        Plus | Minus | Star | Slash | Less | LessEqual | Greater | GreaterEqual => {
            let (Value::Number(a), Value::Number(b)) = (&le, &re) else {
                return Err(format!("expect two number, have {:?}, {:?}", le, re));
            };
            let v = match op {
                Plus => Value::Number(a + b),
                Minus => Value::Number(a - b),
                Star => Value::Number(a * b),
                Slash => Value::Number(a / b),
                Less => Value::Bool(a < b),
                LessEqual => Value::Bool(a <= b),
                Greater => Value::Bool(a > b),
                GreaterEqual => Value::Bool(a >= b),
                _ => panic!(),
            };
            Ok(v)
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
