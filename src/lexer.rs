use crate::token::{Token, TokenResult};
use logos::{Lexer, Logos};

pub struct PeekableLexer<'source> {
    lexer: Lexer<'source, Token>,
    peeked: Option<Option<TokenResult>>,
}

impl<'source> PeekableLexer<'source> {
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

impl<'source> Iterator for PeekableLexer<'source> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<TokenResult> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }
}
