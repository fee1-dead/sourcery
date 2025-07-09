use std::mem;

use ra_ap_rustc_lexer::{Cursor, FrontmatterAllowed, TokenKind};
use smol_str::SmolStr;

use crate::Lexer;
use crate::ast::{Braces, Parens};
use crate::ast::{File, Item, ItemMod, List, Module, Path, PathSegment, VisRestricted, Visibility};
use crate::ast::{Ident, Trivia};
use crate::ast::{ItemKind, Literal, Token};

mod attr;
mod expr;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    snippet: SmolStr,
}

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    token: (Trivia, Token),
}

impl<'src> Parser<'src> {
    pub fn new(s: &'src str) -> Self {
        let lexer = crate::lex::tokenize(s);
        let mut p = Parser {
            lexer,
            token: (
                Trivia::default(),
                Token {
                    kind: TokenKind::Whitespace,
                    snippet: SmolStr::new_static(""),
                },
            ),
        };
        p.bump();
        p
    }
    pub fn bump(&mut self) -> (Trivia, Token) {
        let (trivia, kind, snippet) = self.lexer.next();
        mem::replace(&mut self.token, (trivia, Token { kind, snippet }))
    }
    pub fn peek(&self, f: impl FnOnce(&Token) -> bool) -> bool {
        f(&self.token.1)
    }
    pub fn check(&self, tok: TokenKind) -> bool {
        self.token.1.kind == tok
    }
    pub fn check_ident(&self, s: &str) -> bool {
        self.check(TokenKind::Ident) && self.token.1.snippet == s
    }
    pub fn eat(&mut self, tok: TokenKind) -> Option<Trivia> {
        self.check(tok).then(|| self.bump().0)
    }
    fn lookahead<R>(&mut self, n: usize, x: impl FnOnce(&(Trivia, Token)) -> R) -> R {
        let snapshot = self.lexer.inner.as_str();
        let pos_before = self.lexer.cur_pos;
        let mut restore = None;
        for _ in 0..n {
            let orig = self.bump();
            if restore.is_none() {
                restore = Some(orig);
            }
        }

        let r = x(&self.token);
        self.lexer.inner = Cursor::new(snapshot, FrontmatterAllowed::No);
        self.lexer.cur_pos = pos_before;
        if let Some(orig) = restore {
            self.token = orig;
        }
        r
    }

    /// eat two tokens, expecting no trivia between them.
    pub fn eat2(&mut self, tok1: TokenKind, tok2: TokenKind) -> Option<Trivia> {
        if self.check(tok1) && self.lookahead(1, |(tr0, tok)| tr0.is_empty() && tok.kind == tok2) {
            let (t, _) = self.bump();
            self.bump();
            Some(t)
        } else {
            None
        }
    }

    pub fn eat_ident(&mut self, s: &str) -> Option<Trivia> {
        self.check_ident(s).then(|| self.bump().0)
    }

    pub fn parse_ident(&mut self) -> (Trivia, Ident) {
        assert!(self.check(TokenKind::Ident));
        let (t, tok) = self.bump();
        (t, Ident(tok.snippet))
    }

    pub fn parse_item(&mut self) -> (Trivia, Item) {
        let attrs = self.parse_attrs();
        let vis = self.parse_vis();
        if let Some(tbeforemod) = self.eat_ident("mod") {
            let (t1, name) = self.parse_ident();
            let (t2, semi, content) = if let Some(t2) = self.eat(TokenKind::Semi) {
                (t2, Some(Token![;]), None)
            } else if let Some(t2) = self.eat(TokenKind::OpenBrace) {
                let module = self.parse_module_before(TokenKind::CloseBrace);
                (t2, None, Some(Braces(module)))
            } else {
                unimplemented!()
            };
            let (t0, attrs, vis) = match (attrs, vis) {
                (Some((t0, mut attrs)), Some((tsquash, vis))) => {
                    attrs.push_trivia(tsquash);
                    (t0, attrs, Some((vis, tbeforemod)))
                }
                (Some((t0, attrs)), None) => (t0, attrs, None),
                (None, Some((t0, vis))) => (t0, List::default(), Some((vis, tbeforemod))),
                (None, None) => (tbeforemod, List::default(), None),
            };
            (
                t0,
                Item {
                    attrs,
                    kind: ItemKind::Mod(ItemMod {
                        vis: vis,
                        kw: Token![mod],
                        t1,
                        name,
                        t2,
                        semi,
                        content,
                    }),
                },
            )
        } else {
            unimplemented!("{:?}", self.token)
        }
    }
    pub fn parse_module_before(&mut self, tok: TokenKind) -> Module {
        let mut module = Module {
            t1: Trivia::default(),
            items: List::default(),
        };
        if let Some(tlast) = self.eat(tok) {
            module.items.push_trivia(tlast);
            return module;
        }
        let (t1, item) = self.parse_item();
        module.t1 = t1;
        module.items = List::single(item);

        loop {
            if let Some(tlast) = self.eat(tok) {
                module.items.push_trivia(tlast);
                return module;
            }
            let (t, i) = self.parse_item();
            module.items.push(t, i);
        }
    }
    pub fn parse_path_segment(&mut self) -> (Trivia, PathSegment) {
        let (t0, ident) = self.parse_ident();
        (t0, PathSegment { ident })
    }
    pub fn parse_path(&mut self) -> (Trivia, Path) {
        let (t0, leading_colon, seg1) =
            if let Some(t0) = self.eat2(TokenKind::Colon, TokenKind::Colon) {
                let (t1, seg1) = self.parse_path_segment();
                (t0, Some((Token![::], t1)), seg1)
            } else {
                let (t0, seg1) = self.parse_path_segment();
                (t0, None, seg1)
            };

        let mut rest = vec![];

        while let Some(t1) = self.eat2(TokenKind::Colon, TokenKind::Colon) {
            let (t2, seg) = self.parse_path_segment();
            rest.push((t1, Token![::], t2, seg));
        }

        (
            t0,
            Path {
                leading_colon,
                seg1,
                rest,
            },
        )
    }

    pub fn parse_vis(&mut self) -> Option<(Trivia, Visibility)> {
        let t0 = self.eat_ident("pub")?;
        if let Some(t1) = self.eat(TokenKind::OpenParen) {
            let (t2, in_, path) = if let Some(t2) = self.eat_ident("in") {
                let (t2_5, path) = self.parse_path();
                (t2, Some((Token![in], t2_5)), path)
            } else {
                let (t2, ident) = self.parse_ident();
                (
                    t2,
                    None,
                    Path {
                        leading_colon: None,
                        seg1: PathSegment { ident },
                        rest: vec![],
                    },
                )
            };
            let t3 = self.eat(TokenKind::CloseParen).unwrap();
            Some((
                t0,
                Visibility::Restricted {
                    pub_: Token![pub],
                    t1,
                    parens: Parens(VisRestricted { t2, in_, path, t3 }),
                },
            ))
        } else {
            Some((t0, Visibility::Public(Token![pub])))
        }
    }
    pub fn parse_literal(&mut self) -> (Trivia, Literal) {
        let (t0, token) = self.bump();
        match token.kind {
            TokenKind::Literal {
                kind: _,
                suffix_start,
            } => {
                let suffix_start = suffix_start as usize;
                let symbol = SmolStr::new(&token.snippet[..suffix_start]);
                let suffix = SmolStr::new(&token.snippet[suffix_start..]);
                (t0, Literal { symbol, suffix })
            }
            t => panic!("wrong TokenKind for `parse_literal`: {t:?}"),
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
