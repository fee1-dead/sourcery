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
    // TODO make private
    pub list: Vec<Trivium>,
}

impl Trivia {
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
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
            let v = self
                .list
                .iter()
                .map(|tv| match tv.kind {
                    TriviumKind::BlockComment => {
                        TriviumKindWrap::BlockComment(&s[conv_span(tv.span.clone())])
                    }
                    TriviumKind::LineComment => {
                        TriviumKindWrap::LineComment(&s[conv_span(tv.span.clone())])
                    }
                    TriviumKind::Whitespace => {
                        TriviumKindWrap::Whitespace(&s[conv_span(tv.span.clone())])
                    }
                })
                .collect::<Vec<_>>();
            write!(f, "{v:?}")
        } else {
            f.debug_list()
                .entries(self.list.iter().map(|t| (&t.kind, &t.span)))
                .finish()
        }
    }
}

pub(crate) mod grouping {
    use crate::print::Print;

    #[derive(Debug)]
    pub struct Braces<T>(pub T);

    impl<T: Print> Print for Braces<T> {
        fn print(&self, orig_src: &str, dest: &mut String) {
            dest.push('{');
            self.0.print(orig_src, dest);
            dest.push('}');
        }
    }

    #[derive(Debug)]
    pub struct Parens<T>(pub T);

    impl<T: Print> Print for Parens<T> {
        fn print(&self, orig_src: &str, dest: &mut String) {
            dest.push('(');
            self.0.print(orig_src, dest);
            dest.push(')');
        }
    }
}

macro_rules! define_tokens {
    (keywords($($kname:ident($kt:tt)),*$(,)?); tokens($($tname:ident($tt:tt)),*$(,)?);) => {
        pub(crate) mod kw {
            $(
                #[derive(Debug)]
                pub struct $kname;

                impl crate::print::Print for $kname {
                    fn print(&self, _: &str, out: &mut String) {
                        out.push_str(stringify!($kt))
                    }
                }
            )*
        }
        pub(crate) mod tok {
            $(
                #[derive(Debug)]
                pub struct $tname;

                impl crate::print::Print for $tname {
                    fn print(&self, _: &str, out: &mut String) {
                        out.push_str(stringify!($tt))
                    }
                }
            )*
        }

        #[macro_export]
        macro_rules! token {
            $(
                [$kt] => (crate::kw::$kname);
            )*
            $(
                [$tt] => (crate::tok::$tname);
            )*
        }

        pub use token;
    };
}

define_tokens! {
    keywords(Mod(mod), Pub(pub), In(in));
    tokens(Semi(;), ColonColon(::));
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
