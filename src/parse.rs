use std::cell::RefCell;
use std::mem;
use std::ops::Range;

use ra_ap_rustc_lexer::TokenKind;

use crate::ast::{File, Item, ItemMod, List, Module};
use crate::grouping::Braces;
use crate::{conv_span, token, Ident, Lexer, Trivia};

thread_local! {
    pub static SRC: RefCell<Option<String>> = const { RefCell::new(None) };
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: Range<u32>,
}

pub struct Parser<'src> {
    orig: &'src str,
    lexer: Lexer<'src>,
    token: (Trivia, Token),
}

impl<'src> Parser<'src> {
    pub fn new(s: &'src str) -> Self {
        SRC.set(Some(s.to_owned()));
        let lexer = crate::lex::tokenize(s);
        let mut p = Parser { orig: s, lexer, token: (Trivia::default(), Token { kind: TokenKind::Whitespace, span: 0..0 }) };
        p.bump();
        p
    }
    pub fn bump(&mut self) -> (Trivia, Token) {
        let (trivia, kind, span) = self.lexer.next();
        mem::replace(&mut self.token, (trivia, Token { kind, span }))
    }
    pub fn check(&self, tok: TokenKind) -> bool {
        self.token.1.kind == tok
    }
    pub fn snippet(&self) -> &'src str {
        &self.orig[conv_span(self.token.1.span.clone())]
    }
    pub fn check_ident(&self, s: &str) -> bool {
        self.check(TokenKind::Ident) && self.snippet() == s
    }
    pub fn eat(&mut self, tok: TokenKind) -> Option<Trivia> {
        self.check(tok).then(|| self.bump().0)
    }
    pub fn eat_ident(&mut self, s: &str) -> Option<Trivia> {
        self.check_ident(s).then(|| self.bump().0)
    }
    pub fn parse_ident(&mut self) -> (Trivia, Ident) {
        assert!(self.check(TokenKind::Ident));
        let (t, tok) = self.bump();
        (t, Ident(tok.span))
    }
    pub fn parse_item(&mut self) -> (Trivia, Item) {
        if let Some(t0) = self.eat_ident("mod") {
            let (t1, name) = self.parse_ident();
            let (t2, semi, content) = if let Some(t2) = self.eat(TokenKind::Semi) {
                (t2, Some(token![;]), None)
            } else if let Some(t2) = self.eat(TokenKind::OpenBrace) {
                let module = self.parse_module_before(TokenKind::CloseBrace);
                (t2, None, Some(Braces(module)))
            } else {
                unimplemented!()
            };
            (t0, Item::Mod(ItemMod {
                kw: token![mod],
                t1,
                name,
                t2,
                semi,
                content,
            }))
        } else {
            unimplemented!("{:?}, {:?}", self.token, self.snippet())
        }
    }
    pub fn parse_module_before(&mut self, tok: TokenKind) -> Module {
        let mut module = Module {
            t1: Trivia::default(),
            items: List::default(),
            tlast: Trivia::default(),
        };
        if let Some(tlast) = self.eat(tok) {
            module.tlast = tlast;
            return module;
        }
        let (t1, item) = self.parse_item();
        module.t1 = t1;
        module.items = List::single(item);

        loop {
            if let Some(tlast) = self.eat(tok) {
                module.tlast = tlast;
                return module;
            }
            let (t, i) = self.parse_item();
            module.items.push(t, i);
        }
    }
    pub fn parse_module(&mut self) -> Module {
        self.parse_module_before(TokenKind::Eof)
    }
}

pub fn parse(s: &str) -> File {
    let mut p = Parser::new(s);
    let module = p.parse_module();
    File { module }
}
