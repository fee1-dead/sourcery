use crate::grouping::{Braces, Parens};
use crate::print::Print;
use crate::token::token;
use crate::{Ident, Trivia};

#[derive(Debug)]
pub struct List<T> {
    first: Option<Box<T>>,
    inner: Vec<(Trivia, T)>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            first: None,
            inner: vec![],
        }
    }
}

impl<T: Print> Print for List<T> {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let List { first, inner } = self;
        first.as_deref().print(orig_src, dest);
        for (t, x) in inner {
            t.print(orig_src, dest);
            x.print(orig_src, dest);
        }
    }
}

impl<T> List<T> {
    pub fn single(x: T) -> List<T> {
        List {
            first: Some(Box::new(x)),
            inner: vec![],
        }
    }

    pub fn push(&mut self, t: Trivia, x: T) {
        if self.first.is_none() {
            assert!(t.list.is_empty());
            self.first = Some(Box::new(x));
        } else {
            self.inner.push((t, x));
        }
    }
}

#[derive(Debug)]
pub struct ItemMod {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: token![mod],
    pub t1: Trivia,
    pub name: Ident,
    pub t2: Trivia,
    pub semi: Option<token![;]>,
    pub content: Option<Braces<Module>>,
}

#[derive(Debug)]
pub struct PathSegment {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct Path {
    pub leading_colon: Option<(token![::], Trivia)>,
    pub seg1: PathSegment,
    pub rest: Vec<(Trivia, token![::], Trivia, PathSegment)>,
}

#[derive(Debug)]
pub struct VisRestricted {
    pub t2: Trivia,
    pub in_: Option<(token![in], Trivia)>,
    pub path: Path,
    pub t3: Trivia,
}

#[derive(Debug)]
pub enum Visibility {
    Public(token![pub]),
    Restricted {
        pub_: token![pub],
        t1: Trivia,
        parens: Parens<VisRestricted>,
    },
}

#[derive(Debug)]
pub enum Item {
    Mod(ItemMod),
}

#[derive(Debug)]
pub struct Module {
    pub t1: Trivia,
    pub items: List<Item>,
    pub tlast: Trivia,
}

#[derive(Debug)]
pub struct File {
    // shebang, frontmatter
    pub module: Module,
}
