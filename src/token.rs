use std::ops::Range;

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

#[derive(Debug)]
pub struct Trivia {
    pub list: Vec<Trivium>,
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

#[derive(Debug)]
pub struct Ident(pub Range<u32>);


