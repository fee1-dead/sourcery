use std::fmt::Debug;

use smol_str::SmolStr;

#[derive(Debug)]
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
        let Trivia { list } = self;
        // intentionally switch back to non-fancy formatting
        write!(f, "{list:?}")
    }
}

pub(crate) mod grouping {
    use crate::print::Print;

    #[derive(Debug)]
    pub struct Braces<T>(pub T);

    impl<T: Print> Print for Braces<T> {
        fn print(&self, dest: &mut String) {
            dest.push('{');
            self.0.print(dest);
            dest.push('}');
        }
    }

    #[derive(Debug)]
    pub struct Parens<T>(pub T);

    impl<T: Print> Print for Parens<T> {
        fn print(&self, dest: &mut String) {
            dest.push('(');
            self.0.print(dest);
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
                    fn print(&self, out: &mut String) {
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
                    fn print(&self, out: &mut String) {
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

pub struct Ident(pub SmolStr);

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
