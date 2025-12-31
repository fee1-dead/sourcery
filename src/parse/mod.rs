use std::ops::Shl;
use std::{mem, vec};

use sourcery_derive::Walk;

use crate::ast::{Delimited, Delimiter, Parens, QPath, QSelf, TriviaN};
use crate::ast::{File, List, Module, Path, PathSegment, VisRestricted, Visibility};
use crate::ast::{Ident, Trivia};
use crate::ast::{Literal, Token};
use crate::parse::attr::AttrKind;
use crate::parse::glue::Gluer;
use crate::passes::Visit;
use crate::{Print, lex};

mod attr;
mod expr;
mod generics;
mod glue;
mod item;
mod pat;
mod path;
mod stmt;
mod ty;

#[derive(Default, Clone, Debug, Print, Walk)]
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
    fn next(&mut self) -> WithLeadingTrivia<TokenTree>;
    fn snapshot(&self) -> Box<dyn TokenIterator + '_>;
}

impl TokenIterator for TokenStreamIter {
    fn snapshot(&self) -> Box<dyn TokenIterator + '_> {
        Box::new(self.clone())
    }
    fn next(&mut self) -> WithLeadingTrivia<TokenTree> {
        match self.inner.next() {
            Some((tt, trivia)) => {
                let t = mem::replace(&mut self.tprev, trivia);
                t << tt
            }
            None => {
                let mut prev = self.tprev.take();
                prev.extend(self.last.take());
                prev << TokenTree::Eof
            }
        }
    }
}

impl TokenIterator for Gluer<'_> {
    fn snapshot(&self) -> Box<dyn TokenIterator + '_> {
        Box::new(self.snapshot())
    }
    fn next(&mut self) -> WithLeadingTrivia<TokenTree> {
        Gluer::next(self)
    }
}

#[derive(Clone, Debug, Print, Walk)]
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
            _ => false,
        }
    }

    pub fn is_ident(&self, i: &str) -> bool {
        match self {
            TokenTree::Ident(i2) => i == i2.0,
            _ => false,
        }
    }

    pub fn is_punct(&self, punct: Punct) -> bool {
        match self {
            TokenTree::Punct(p2) => punct == *p2,
            _ => false,
        }
    }

    pub fn into_literal(self) -> Option<Literal> {
        match self {
            TokenTree::Literal(l) => Some(l),
            _ => None,
        }
    }

    pub fn into_lifetime(self) -> Option<Ident> {
        match self {
            TokenTree::Lifetime(l) => Some(l),
            _ => None,
        }
    }

    pub fn into_group(self) -> Option<Delimited<TokenStream>> {
        match self {
            TokenTree::Group(d) => Some(*d),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Punct {
    Semi,
    Comma,
    Dot,
    DotDot,
    DotDotEq,
    DotDotDot,
    At,
    Pound,
    Tilde,
    Question,
    Colon,
    ColonColon,
    Dollar,
    Eq,
    EqEq,
    Bang,
    BangEq,
    Lt,
    LtEq,
    LtLtEq,
    Gt,
    GtEq,
    GtGtEq,
    Minus,
    MinusEq,
    And,
    AndEq,
    Or,
    OrEq,
    Plus,
    PlusEq,
    Star,
    StarEq,
    Slash,
    SlashEq,
    Caret,
    CaretEq,
    Percent,
    PercentEq,
    RThinArrow,
    RFatArrow,
    LThinArrow,
}

impl Visit for Punct {
    fn visit<P: crate::passes::Pass + ?Sized>(&mut self, _: &mut P) {}
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
    Semi, Comma, Dot, DotDot, DotDotEq, DotDotDot, At, Pound, Tilde,
    Question, Colon, ColonColon, Dollar, Eq, EqEq, Bang, BangEq, Lt,
    LtEq, LtLtEq, Gt, GtEq, GtGtEq, Minus, MinusEq, And, AndEq, Or,
    OrEq, Plus, PlusEq, Star, StarEq, Slash, SlashEq, Caret, CaretEq,
    Percent, PercentEq, RThinArrow, RFatArrow, LThinArrow
);

#[derive(Clone, Debug)]
pub struct WithLeadingTrivia<T>(pub Trivia, pub T);

impl<T> WithLeadingTrivia<T> {
    pub fn map<F: FnOnce(T) -> R, R>(self, f: F) -> WithLeadingTrivia<R> {
        WithLeadingTrivia(self.0, f(self.1))
    }
}

pub use WithLeadingTrivia as L;

// I think someone will hate this :3
impl<T> Shl<T> for Trivia {
    type Output = WithLeadingTrivia<T>;
    fn shl(self, rhs: T) -> Self::Output {
        WithLeadingTrivia(self, rhs)
    }
}

pub struct Parser<'src> {
    tokens: Box<dyn TokenIterator + 'src>,
    token: WithLeadingTrivia<TokenTree>,
}

impl<'src> Parser<'src> {
    fn create(x: impl TokenIterator + 'src) -> Self {
        let mut p = Parser {
            tokens: Box::new(x),
            token: Trivia::default() << TokenTree::Eof,
        };
        p.bump();
        p
    }
    pub fn new(s: &'src str) -> Self {
        Parser::create(Gluer::new(lex::tokenize(s)))
    }
    pub fn bump(&mut self) -> WithLeadingTrivia<TokenTree> {
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
    pub fn peek2(&self, f: impl FnOnce(&TokenTree) -> bool) -> bool {
        self.peek_nth(1, move |L(_, t)| f(t))
    }
    pub fn peek3(&self, f: impl FnOnce(&TokenTree) -> bool) -> bool {
        self.peek_nth(2, move |L(_, t)| f(t))
    }
    #[must_use]
    pub fn check_ident(&self, s: &str) -> bool {
        matches!(&self.token.1, TokenTree::Ident(Ident(id)) if s == id)
    }
    #[must_use]
    pub fn check_punct(&self, punct: Punct) -> bool {
        matches!(self.token.1, TokenTree::Punct(got) if got == punct)
    }
    pub fn eat_punct(&mut self, punct: Punct) -> Option<Trivia> {
        self.check_punct(punct).then(|| self.bump().0)
    }
    pub fn eat_delimited(&mut self) -> Option<WithLeadingTrivia<Delimited<TokenStream>>> {
        self.eat(|tt| matches!(tt, TokenTree::Group(_)))
            .map(|tt| tt.map(|tt| tt.into_group().unwrap()))
    }
    pub fn eat_delim<T>(
        &mut self,
        delim: Delimiter,
        f: impl FnOnce(Trivia, Parser<'src>) -> T,
    ) -> Option<T> {
        if let Some(L(t, TokenTree::Group(tokens))) =
            self.eat(|t| matches!(t, TokenTree::Group(tokens) if tokens.delimiter() == delim))
        {
            let p = Parser::create(tokens.into_inner().into_iter());
            Some(f(t, p))
        } else {
            None
        }
    }
    pub fn eat_eof(&mut self) -> Option<Trivia> {
        self.eat(|tt| matches!(tt, TokenTree::Eof)).map(|L(t, _)| t)
    }
    pub fn eat_literal(&mut self) -> Option<WithLeadingTrivia<Literal>> {
        self.eat(|tt| matches!(tt, TokenTree::Literal(_)))
            .map(|tt| tt.map(|tt| tt.into_literal().unwrap()))
    }
    pub fn eat(
        &mut self,
        f: impl FnOnce(&TokenTree) -> bool,
    ) -> Option<WithLeadingTrivia<TokenTree>> {
        self.peek(f).then(|| self.bump())
    }
    fn peek_nth<R>(&self, n: usize, x: impl FnOnce(&L<TokenTree>) -> R) -> R {
        let mut parser = self.snapshot();
        for _ in 0..n {
            parser.bump();
        }
        x(&parser.token)
    }

    pub fn eat_kw(&mut self, s: &str) -> Option<Trivia> {
        self.eat_ident(s).map(|L(t, _)| t)
    }

    pub fn eat_ident(&mut self, s: &str) -> Option<L<Ident>> {
        self.check_ident(s).then(|| {
            let L(t, tt) = self.bump();
            let TokenTree::Ident(id) = tt else {
                unreachable!()
            };
            t << id
        })
    }

    pub fn parse_ident(&mut self) -> L<Ident> {
        let L(t, tok) = self.bump();
        let TokenTree::Ident(id) = tok else {
            panic!("expected ident")
        };
        t << id
    }

    pub fn parse_path_segment(&mut self) -> L<PathSegment> {
        let L(t0, ident) = self.parse_ident();
        t0 << PathSegment { ident }
    }
    pub fn parse_path(&mut self) -> L<Path> {
        let (t0, leading_colon, seg1) = if let Some(t0) = self.eat_punct(Punct::ColonColon) {
            let L(t1, seg1) = self.parse_path_segment();
            (t0, Some((Token![::], t1)), seg1)
        } else {
            let L(t0, seg1) = self.parse_path_segment();
            (t0, None, seg1)
        };

        let mut rest = vec![];

        while let Some(t1) = self.eat_punct(Punct::ColonColon) {
            let L(t2, seg) = self.parse_path_segment();
            rest.push((t1, Token![::], t2, seg));
        }

        t0 << Path {
            leading_colon,
            seg1,
            rest,
        }
    }

    pub fn parse_qpath(&mut self) -> L<QPath> {
        if let Some(t0) = self.eat_punct(Punct::Lt) {
            let L(t1, selfty) = self.parse_ty();
            let as_ = if let Some(L(t2, _)) = self.eat_ident("as") {
                let L(t3, p) = self.parse_path();
                Some((t2, Token![as], t3, p))
            } else {
                None
            };
            let tlast = self.eat_punct(Punct::Gt).unwrap();
            let L(tprev, path) = self.parse_path();

            t0 << QPath {
                qself: Some((
                    (QSelf {
                        left: Token![<],
                        t1,
                        ty: Box::new(selfty),
                        as_,
                        tlast,
                        right: Token![>],
                    }),
                    tprev,
                )),
                path,
            }
        } else {
            self.parse_path().map(|path| QPath { qself: None, path })
        }
    }

    pub fn parse_vis(&mut self) -> Option<L<Visibility>> {
        let L(t0, _) = self.eat_ident("pub")?;
        let vis = self
            .eat_delim(Delimiter::Parens, |t1, mut this| {
                let (t2, in_, path) = if let Some(L(t2, _)) = this.eat_ident("in") {
                    let L(t2_5, path) = this.parse_path();
                    (t2, Some((Token![in], TriviaN::new(t2_5))), path)
                } else {
                    let L(t2, ident) = this.parse_ident();
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
            .unwrap_or(Visibility::Public { pub_: Token![pub] });

        Some(t0 << vis)
    }
    pub fn parse_module(&mut self) -> Module {
        let (t1, attrs) = self.parse_attrs(AttrKind::Inner).unwrap_or_default();
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

#[cfg(test)]
pub fn parse_trivia(s: &str) -> Trivia {
    Parser::new(s).eat_eof().unwrap()
}

pub fn parse_to_tokenstream(s: &str) -> TokenStream {
    Gluer::new(lex::tokenize(s)).collect()
}
