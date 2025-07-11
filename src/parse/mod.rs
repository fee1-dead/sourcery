use std::{mem, vec};

use ra_ap_rustc_lexer::TokenKind;
use smol_str::SmolStr;

use crate::ast::{Braces, Delimited, Delimiter, Parens};
use crate::ast::{File, Item, ItemMod, List, Module, Path, PathSegment, VisRestricted, Visibility};
use crate::ast::{Ident, Trivia};
use crate::ast::{ItemKind, Literal, Token};
use crate::parse::attr::AttrKind;
use crate::parse::glue::Gluer;
use crate::{Print, TrivialPrint, lex};

mod attr;
mod expr;
mod glue;

#[derive(Default, Clone, Debug, TrivialPrint!)]
pub struct TokenStream {
    pub t1: Trivia,
    pub tokens: List<TokenTree>,
}

impl TokenStream {
    pub fn into_iter(self) -> TokenStreamIter {
        let (v, last) = self.tokens.into_parts();
        TokenStreamIter {
            tprev: self.t1,
            inner: v.into_iter(),
            last,
        }
    }
}

#[derive(Clone)]
pub struct TokenStreamIter {
    tprev: Trivia,
    inner: vec::IntoIter<(TokenTree, Trivia)>,
    last: Trivia,
}

pub trait TokenIterator {
    fn next(&mut self) -> (Trivia, TokenTree);
    fn snapshot(&self) -> Box<dyn TokenIterator + '_>;
}

impl TokenIterator for TokenStreamIter {
    fn snapshot(&self) -> Box<dyn TokenIterator + '_> {
        Box::new(self.clone())
    }
    fn next(&mut self) -> (Trivia, TokenTree) {
        match self.inner.next() {
            Some((tt, trivia)) => {
                let t = mem::replace(&mut self.tprev, trivia);
                (t, tt)
            }
            None => {
                let mut prev = mem::take(&mut self.tprev);
                prev.list.extend(mem::take(&mut self.last).list);
                (prev, TokenTree::Eof)
            }
        }
    }
}

impl TokenIterator for Gluer<'_> {
    fn snapshot(&self) -> Box<dyn TokenIterator + '_> {
        Box::new(self.snapshot())
    }
    fn next(&mut self) -> (Trivia, TokenTree) {
        Gluer::next(self)
    }
}

#[derive(Clone, Debug, TrivialPrint!)]
pub enum TokenTree {
    Group(Box<Delimited<TokenStream>>),
    Punct(Punct),
    Ident(Ident),
    RawIdent(Ident),
    Lifetime(Ident),
    RawLifetime(Ident),
    Literal(Literal),
    Eof,
}

impl TokenTree {
    pub fn is_delim(&self, delim: Delimiter) -> bool {
        match self {
            TokenTree::Group(delimited) => delimited.delimiter() == delim,
            _ => false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Punct {
    Semi,
    Comma,
    Dot,
    At,
    Pound,
    Tilde,
    Question,
    Colon,
    ColonColon,
    Dollar,
    Eq,
    Bang,
    Lt,
    Gt,
    Minus,
    And,
    Or,
    Plus,
    Star,
    Slash,
    Caret,
    Percent,
}

macro_rules! impl_print_for_punct {
    ($($Variant:ident),*$(,)?) => {
        impl Print for Punct {
            fn print(&self, out: &mut String) {
                match self {
                    $( Punct::$Variant => crate::ast::tokens::$Variant.print(out), )*
                }
            }
        }
    };
}
impl_print_for_punct!(
    Semi, Comma, Dot, At, Pound, Tilde, Question, Colon, ColonColon, Dollar, Eq, Bang, Lt, Gt,
    Minus, And, Or, Plus, Star, Slash, Caret, Percent,
);

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    snippet: SmolStr,
}

impl Print for Token {
    fn print(&self, dest: &mut String) {
        self.snippet.print(dest);
    }
}

pub struct Parser<'src> {
    tokens: Box<dyn TokenIterator + 'src>,
    token: (Trivia, TokenTree),
}

impl<'src> Parser<'src> {
    fn create(x: impl TokenIterator + 'src) -> Self {
        let mut p = Parser {
            tokens: Box::new(x),
            token: (Trivia::default(), TokenTree::Eof),
        };
        p.bump();
        p
    }
    pub fn new(s: &'src str) -> Self {
        Parser::create(Gluer::new(lex::tokenize(s)))
    }
    pub fn bump(&mut self) -> (Trivia, TokenTree) {
        mem::replace(&mut self.token, self.tokens.next())
    }
    pub fn snapshot(&'src self) -> Self {
        Parser {
            tokens: self.tokens.snapshot(),
            token: self.token.clone(),
        }
    }
    pub fn peek(&self, f: impl FnOnce(&TokenTree) -> bool) -> bool {
        f(&self.token.1)
    }
    #[must_use]
    pub fn check_ident(&self, s: &str) -> bool {
        matches!(&self.token.1, TokenTree::Ident(Ident(id)) if s == id)
    }
    #[must_use]
    pub fn check_punct(&mut self, punct: Punct) -> bool {
        matches!(self.token.1, TokenTree::Punct(got) if got == punct)
    }
    pub fn eat_punct(&mut self, punct: Punct) -> Option<Trivia> {
        self.check_punct(punct).then(|| self.bump().0)
    }
    pub fn eat_delim<T>(
        &mut self,
        delim: Delimiter,
        f: impl FnOnce(Trivia, Parser<'src>) -> T,
    ) -> Option<T> {
        if let Some((t, TokenTree::Group(tokens))) =
            self.eat(|t| matches!(t, TokenTree::Group(tokens) if tokens.delimiter() == delim))
        {
            let p = Parser::create(tokens.into_inner().into_iter());
            Some(f(t, p))
        } else {
            None
        }
    }
    pub fn eat_eof(&mut self) -> Option<Trivia> {
        self.eat(|tt| matches!(tt, TokenTree::Eof)).map(|(t, _)| t)
    }
    pub fn eat_literal(&mut self) -> Option<(Trivia, Literal)> {
        self.eat(|tt| matches!(tt, TokenTree::Literal(_))).map(|(t, tt)| (t, match tt {
            TokenTree::Literal(l) => l,
            _ => unreachable!()
        }))
    }
    pub fn eat(&mut self, f: impl FnOnce(&TokenTree) -> bool) -> Option<(Trivia, TokenTree)> {
        self.peek(f).then(|| self.bump())
    }
    fn peek_nth<R>(&mut self, n: usize, x: impl FnOnce(&(Trivia, TokenTree)) -> R) -> R {
        let mut parser = self.snapshot();
        for _ in 0..n {
            parser.bump();
        }
        x(&parser.token)
    }

    pub fn eat_ident(&mut self, s: &str) -> Option<(Trivia, Ident)> {
        self.check_ident(s).then(|| {
            let (t, tt) = self.bump();
            let TokenTree::Ident(id) = tt else {
                unreachable!()
            };
            (t, id)
        })
    }

    pub fn parse_ident(&mut self) -> (Trivia, Ident) {
        let (t, tok) = self.bump();
        let TokenTree::Ident(id) = tok else {
            panic!("expected ident")
        };
        (t, id)
    }

    pub fn parse_item(&mut self) -> (Trivia, Item) {
        let attrs = self.parse_attrs(AttrKind::Inner);
        let vis = self.parse_vis();
        if let Some((tbeforemod, _)) = self.eat_ident("mod") {
            let (t1, name) = self.parse_ident();
            let (t2, semi, content) = if let Some(t2) = self.eat_punct(Punct::Semi) {
                (t2, Some(Token![;]), None)
            } else if let Some((t2, module)) =
                self.eat_delim(Delimiter::Braces, |t2, mut this| (t2, this.parse_module()))
            {
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

    pub fn parse_path_segment(&mut self) -> (Trivia, PathSegment) {
        let (t0, ident) = self.parse_ident();
        (t0, PathSegment { ident })
    }
    pub fn parse_path(&mut self) -> (Trivia, Path) {
        let (t0, leading_colon, seg1) = if let Some(t0) = self.eat_punct(Punct::ColonColon) {
            let (t1, seg1) = self.parse_path_segment();
            (t0, Some((Token![::], t1)), seg1)
        } else {
            let (t0, seg1) = self.parse_path_segment();
            (t0, None, seg1)
        };

        let mut rest = vec![];

        while let Some(t1) = self.eat_punct(Punct::ColonColon) {
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
        let (t0, _) = self.eat_ident("pub")?;
        let vis = self
            .eat_delim(Delimiter::Parens, |t1, mut this| {
                let (t2, in_, path) = if let Some((t2, _)) = this.eat_ident("in") {
                    let (t2_5, path) = this.parse_path();
                    (t2, Some((Token![in], t2_5)), path)
                } else {
                    let (t2, ident) = this.parse_ident();
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
                let t3 = this.eat_eof().unwrap();

                Visibility::Restricted {
                    pub_: Token![pub],
                    t1,
                    parens: Parens(VisRestricted { t2, in_, path, t3 }),
                }
            })
            .unwrap_or_else(|| Visibility::Public { pub_: Token![pub] });

        Some((t0, vis))
    }
    pub fn parse_module(&mut self) -> Module {
        let (t1, attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let mut module = Module {
            t1,
            attrs,
            items: List::default(),
        };
        
        if let Some(tlast) = self.eat_eof() {
            module.items.push_trivia(tlast);
            return module;
        }
        let (t1, item) = self.parse_item();
        module.attrs.push_trivia(t1);
        module.items = List::single(item);

        loop {
            if let Some(tlast) = self.eat_eof() {
                module.items.push_trivia(tlast);
                return module;
            }
            let (t, i) = self.parse_item();
            module.items.push(t, i);
        }
    }
}

pub fn parse(s: &str) -> File {
    let mut p = Parser::new(s);
    let module = p.parse_module();
    File { module }
}
