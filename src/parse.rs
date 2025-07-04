use std::mem;
use std::ops::Range;

use ra_ap_rustc_lexer::TokenKind;

use crate::ast::{File, Item};
use crate::{Lexer, Trivia};

pub struct Token {
    kind: TokenKind,
    span: Range<u32>,
}

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    token: (Trivia, Token),
}

impl<'src> Parser<'src> {
    pub fn bump(&mut self) -> (Trivia, Token) {
        let (trivia, kind, span) = self.lexer.next();
        mem::replace(&mut self.token, (trivia, Token { kind, span }))
    }
    pub fn check(&self, tok: TokenKind) -> bool {
        self.token.1.kind == tok
    }
    pub fn snippet(&self) -> &'src str {
        &self.lexer.inner.as_str()[self.token.1.span.start as usize..self.token.1.span.end as usize]
    }
    pub fn check_ident(&self, s: &str) -> bool {
        self.snippet() == s
    }
    pub fn eat(&mut self, tok: TokenKind) -> Option<Trivia> {
        self.check(tok).then(|| self.bump().0)
    }
    pub fn eat_ident(&mut self, s: &str) -> Option<Trivia> {
        self.check_ident(s).then(|| self.bump().0)
    }
    pub fn parse_item(&mut self) -> (Trivia, Item) {
        if let Some(t1) = self.eat_ident("mod") {
            
        }
        todo!()
    }
}

pub fn parse(s: &str) -> File {
    todo!()
}
