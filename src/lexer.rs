use crate::token::{Token, TokenResult};
use logos::{Lexer as LogosLExer, Logos};

pub struct Lexer<'source> {
    lexer: LogosLExer<'source, Token>,
    peek_1: Option<Option<TokenResult>>,
    peek_2: Option<Option<TokenResult>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peek_1: None,
            peek_2: None,
        }
    }

    pub fn peek_token_flatten(&mut self) -> Option<Token> {
        match self.peek_token() {
            Some(Ok(t)) => Some(t),
            _ => None,
        }
    }

    pub fn peek_two_token_flatten(&mut self) -> Option<Token> {
        match self.peek_two_token() {
            Some(Ok(t)) => Some(t),
            _ => None,
        }
    }

    pub fn peek_token(&mut self) -> Option<TokenResult> {
        if self.peek_1.is_none() {
            self.peek_1 = Some(self.lexer.next());
        }
        self.peek_1.clone().unwrap()
    }

    pub fn peek_two_token(&mut self) -> Option<TokenResult> {
        if self.peek_2.is_none() {
            self.peek_token();
            self.peek_2 = Some(self.lexer.next());
        }
        self.peek_2.clone().unwrap()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<TokenResult> {
        if let Some(peeked) = self.peek_1.take() {
            self.peek_1 = self.peek_2.take();
            peeked
        } else {
            self.lexer.next()
        }
    }
}
