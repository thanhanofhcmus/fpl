use crate::token::{Token, TokenResult};
use logos::{Lexer as LogosLExer, Logos};

pub struct Lexer<'source> {
    lexer: LogosLExer<'source, Token>,
    peeked: Option<Option<TokenResult>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<TokenResult> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        self.peeked.clone().unwrap()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<TokenResult> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }
}
