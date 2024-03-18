use crate::ast::{AstNode, Value};
use crate::lexer::Token;

pub fn interpret(ast: AstNode) -> Result<Value, String> {
    match ast {
        AstNode::Binary { l, op, r } => {
            let le = interpret(*l)?;
            let re = interpret(*r)?;
            let (Value::Number(a), Value::Number(b)) = (le, re) else {
                return Err("invalid type".to_string());
            };
            let v = match op {
                Token::Plus => a + b,
                Token::Minus => a - b,
                Token::Star => a * b,
                Token::Slash => a / b,
                _ => return Err("invalid op".to_owned()),
            };
            Ok(Value::Number(v))
        }
        AstNode::Primary(v) => Ok(v),
    }
}
