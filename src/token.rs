use std::{fmt::Debug, ops::Range};

#[derive(Debug)]
pub enum TriviumKind {
    Whitespace,
    LineComment,
    BlockComment,
}

#[derive(Debug)]
pub struct Trivium {
    pub kind: TriviumKind,
    pub span: Range<u32>,
}

pub fn conv_span(x: Range<u32>) -> Range<usize> {
    x.start as usize..x.end as usize
}

#[derive(Default)]
pub struct Trivia {
    pub list: Vec<Trivium>,
}

impl Debug for Trivia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = crate::parse::SRC.with_borrow(Clone::clone) {
            #[derive(Debug)]
            #[expect(dead_code)] // we're only using this for the `Debug` impl
            enum TriviumKindWrap<'a> {
                Whitespace(&'a str),
                LineComment(&'a str),
                BlockComment(&'a str),
            }
            let v = self.list.iter().map(|tv| match tv.kind {
                TriviumKind::BlockComment => TriviumKindWrap::BlockComment(&s[conv_span(tv.span.clone())]),
                TriviumKind::LineComment => TriviumKindWrap::LineComment(&s[conv_span(tv.span.clone())]),
                TriviumKind::Whitespace => TriviumKindWrap::Whitespace(&s[conv_span(tv.span.clone())]),
            }).collect::<Vec<_>>();
            write!(f, "{v:?}")
        } else {
            f.debug_list().entries(self.list.iter().map(|t| (&t.kind, &t.span))).finish()
        }
    }
}

pub(crate) mod kw {
    #[derive(Debug)]
    pub struct Mod;
}

pub(crate) mod tok {
    #[derive(Debug)]
    pub struct Semi;
}

pub(crate) mod grouping {
    #[derive(Debug)]
    pub struct Braces<T>(pub T);
}

#[macro_export]
macro_rules! token {
    [mod] => (crate::kw::Mod);
    [;] => (crate::tok::Semi);
}

pub struct Ident(pub Range<u32>);

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = crate::parse::SRC.with_borrow(Clone::clone) {
            s[conv_span(self.0.clone())].fmt(f)
        } else {
            f.debug_tuple("Ident").field(&self.0).finish()
        }
    }
}


