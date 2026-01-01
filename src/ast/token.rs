use std::fmt::{self, Debug};

use smol_str::SmolStr;

use crate::Print;

#[derive(Debug, Clone, Print)]
pub enum Trivium {
    Whitespace(SmolStr),
    LineComment(SmolStr),
    BlockComment(SmolStr),
}

impl Trivium {
    pub fn snippet(&self) -> &SmolStr {
        let (Trivium::Whitespace(s) | Trivium::LineComment(s) | Trivium::BlockComment(s)) = self;
        s
    }
    pub const fn single_space() -> Self {
        const { Self::Whitespace(SmolStr::new_inline(" ")) }
    }
}

/// Like [`Trivia`] but cannot be empty.
pub struct TriviaN {
    inner: Trivia,
}

impl Print for TriviaN {
    fn print(&self, dest: &mut String) {
        self.inner.print(dest)
    }
}

impl TriviaN {
    pub fn new(t: Trivia) -> TriviaN {
        assert!(!t.is_empty());
        TriviaN { inner: t }
    }

    pub fn single_space() -> TriviaN {
        let mut t = Trivia::default();
        t.extend([Trivium::Whitespace(SmolStr::new_inline(" "))]);
        Self::new(t)
    }
    
    pub fn take(&mut self) -> TriviaN {
        TriviaN { inner: self.inner.take() }
    }
}

impl fmt::Debug for TriviaN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Default, Clone, Print)]
pub struct Trivia {
    list: Vec<Trivium>,
}

impl Trivia {
    pub fn with_capacity(cap: usize) -> Trivia {
        Self {
            list: Vec::with_capacity(cap),
        }
    }
    pub fn push(&mut self, x: Trivium) {
        self.list.push(x);
    }
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn last(&self) -> Option<&Trivium> {
        self.list.last()
    }
    pub fn iter(&'_ self) -> impl Iterator<Item = &'_ Trivium> {
        self.list.iter()
    }
    pub fn iter_mut(&'_ mut self) -> impl Iterator<Item = &'_ mut Trivium> {
        self.list.iter_mut()
    }
    pub fn trim_whitespace(self) -> Trivia {
        let left = self
            .list
            .iter()
            .take_while(|x| matches!(x, Trivium::Whitespace(..)))
            .count();
        let remaining = (self.list.len() - left).saturating_sub(
            self.list
                .iter()
                .rev()
                .take_while(|x| matches!(x, Trivium::Whitespace(..)))
                .count(),
        );
        Trivia {
            list: self.list.into_iter().skip(left).take(remaining).collect(),
        }
    }
    pub fn take(&mut self) -> Trivia {
        Trivia {
            list: std::mem::take(&mut self.list),
        }
    }
}

impl IntoIterator for Trivia {
    type IntoIter = std::vec::IntoIter<Trivium>;
    type Item = Trivium;
    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl Debug for Trivia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Trivia { list } = self;
        // intentionally switch back to non-fancy formatting
        write!(f, "{list:?}")
    }
}

impl Extend<Trivium> for Trivia {
    fn extend<T: IntoIterator<Item = Trivium>>(&mut self, iter: T) {
        self.list.extend(iter)
    }
}

impl Extend<Trivia> for Trivia {
    fn extend<T: IntoIterator<Item = Trivia>>(&mut self, iter: T) {
        self.list.extend(iter.into_iter().flat_map(|x| x.list))
    }
}

impl From<TriviaN> for Trivia {
    fn from(value: TriviaN) -> Self {
        value.inner
    }
}

pub(crate) mod grouping {
    use std::fmt::Debug;

    use crate::print::Print;

    #[derive(Debug, Clone, Copy)]
    pub struct Braces<T>(pub T);

    impl<T: Print> Print for Braces<T> {
        fn print(&self, dest: &mut String) {
            dest.push('{');
            self.0.print(dest);
            dest.push('}');
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Brackets<T>(pub T);

    impl<T: Print> Print for Brackets<T> {
        fn print(&self, dest: &mut String) {
            dest.push('[');
            self.0.print(dest);
            dest.push(']');
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Parens<T>(pub T);

    impl<T: Print> Print for Parens<T> {
        fn print(&self, dest: &mut String) {
            dest.push('(');
            self.0.print(dest);
            dest.push(')');
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Delimiter {
        Braces,
        Brackets,
        Parens,
    }

    #[derive(Clone, crate::Print)]
    pub enum Delimited<T> {
        Braces(Braces<T>),
        Brackets(Brackets<T>),
        Parens(Parens<T>),
    }

    impl<T> Delimited<T> {
        pub fn into_inner(self) -> T {
            let (Delimited::Braces(Braces(x))
            | Delimited::Brackets(Brackets(x))
            | Delimited::Parens(Parens(x))) = self;
            x
        }

        pub fn inner_mut(&mut self) -> &mut T {
            let (Delimited::Braces(Braces(x))
            | Delimited::Brackets(Brackets(x))
            | Delimited::Parens(Parens(x))) = self;
            x
        }

        pub fn delimiter(&self) -> Delimiter {
            match self {
                Delimited::Braces(_) => Delimiter::Braces,
                Delimited::Brackets(_) => Delimiter::Brackets,
                Delimited::Parens(_) => Delimiter::Parens,
            }
        }
    }

    impl<T: Debug> Debug for Delimited<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Braces(b) => b.fmt(f),
                Self::Brackets(b) => b.fmt(f),
                Self::Parens(p) => p.fmt(f),
            }
        }
    }
}

macro_rules! define_tokens {
    (keywords($($kname:ident($kt:tt)),*$(,)?); tokens($($tname:ident($tt:tt)),*$(,)?);) => {
        pub mod kw {
            $(
                #[derive(Debug)]
                pub struct $kname;

                impl crate::print::Print for $kname {
                    fn print(&self, out: &mut String) {
                        out.push_str(stringify!($kt))
                    }
                }
                impl crate::passes::Visit for $kname {
                    #[inline]
                    fn visit<P: crate::passes::Pass + ?Sized>(&mut self, p: &mut P) {
                        p.visit_token(const { stringify!($kname).len() })
                    }
                }
                impl crate::passes::style::spaces::Respace for $kname {
                    fn respace(&mut self, _: &mut crate::passes::style::spaces::Spaces) {}
                }
            )*
        }
        pub mod tokens {
            $(
                #[derive(Debug)]
                pub struct $tname;

                impl crate::print::Print for $tname {
                    fn print(&self, out: &mut String) {
                        out.push_str(stringify!($tt))
                    }
                }

                impl crate::passes::Visit for $tname {
                    #[inline]
                    fn visit<P: crate::passes::Pass + ?Sized>(&mut self, p: &mut P) {
                        p.visit_token(const { stringify!($tt).len() })
                    }
                }

                impl crate::passes::style::spaces::Respace for $tname {
                    fn respace(&mut self, _: &mut crate::passes::style::spaces::Spaces) {}
                }
            )*
        }

        #[macro_export]
        macro_rules! Token {
            $(
                [$kt] => ($crate::ast::kw::$kname);
            )*
            $(
                [$tt] => ($crate::ast::tokens::$tname);
            )*
        }

        pub use Token;
    };
}

define_tokens! {
    keywords(
        Mod(mod), Pub(pub), In(in), Type(type), Fn(fn), Const(const), Static(static), Unsafe(unsafe), Async(async),
        Try(try), As(as), If(if), Else(else), While(while), Loop(loop), For(for), Match(match), Break(break), Continue(continue),
        Return(return), Yield(yield), Become(become),
    );
    tokens(
        Semi(;),
        Comma(,),
        Dot(.),
        DotDot(..),
        DotDotDot(...),
        DotDotEq(..=),
        At(@),
        Pound(#),
        Tilde(~),
        Question(?),
        Colon(:),
        ColonColon(::),
        Dollar($),
        Eq(=), EqEq(==),
        Bang(!), BangEq(!=),
        Lt(<), LtEq(<=), LtLtEq(<<=),
        Gt(>), GtEq(>=), GtGtEq(>>=),
        Minus(-), MinusEq(-=),
        And(&), AndAnd(&&), AndEq(&=),
        Or(|), OrOr(||), OrEq(|=),
        Plus(+), PlusEq(+=),
        Star(*), StarEq(*=),
        Slash(/), SlashEq(/=),
        Caret(^), CaretEq(^=),
        Percent(%), PercentEq(%=),
        RThinArrow(->), RFatArrow(=>), LThinArrow(<-),
    );
}

#[derive(Clone, Print)]
pub struct Ident(pub SmolStr);

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Print)]
pub struct Literal {
    pub symbol: SmolStr,
    pub suffix: SmolStr,
}
