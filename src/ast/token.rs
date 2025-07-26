use std::fmt::Debug;

use smol_str::SmolStr;

use crate::TrivialPrint;

#[derive(Debug, Clone, TrivialPrint!)]
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
}

#[derive(Default, Clone, TrivialPrint!)]
pub struct Trivia {
    // TODO make private
    pub list: Vec<Trivium>,
}

impl Trivia {
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Debug for Trivia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Trivia { list } = self;
        // intentionally switch back to non-fancy formatting
        write!(f, "{list:?}")
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

    #[derive(Clone, crate::TrivialPrint!)]
    #[derive_args(where(T: Print))]
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
    keywords(Mod(mod), Pub(pub), In(in), Type(type), Fn(fn), Const(const), Unsafe(unsafe));
    tokens(
        Semi(;),
        Comma(,),
        Dot(.),
        At(@),
        Pound(#),
        Tilde(~),
        Question(?),
        Colon(:),
        ColonColon(::),
        Dollar($),
        Eq(=),
        Bang(!),
        Lt(<),
        Gt(>),
        Minus(-),
        And(&),
        Or(|),
        Plus(+),
        Star(*),
        Slash(/),
        Caret(^),
        Percent(%),
        RArrow(->),
    );
}

#[derive(Clone, TrivialPrint!)]
pub struct Ident(pub SmolStr);

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, TrivialPrint!)]
pub struct Literal {
    pub symbol: SmolStr,
    pub suffix: SmolStr,
}
