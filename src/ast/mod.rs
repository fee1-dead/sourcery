use std::fmt::Debug;

mod token;
pub use token::grouping::{Braces, Parens};
pub use token::{Ident, Trivia, Trivium};
pub use token::{Token, kw, tokens};

use crate::print::Print;

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
    fn print(&self, dest: &mut String) {
        let List { first, inner } = self;
        first.as_deref().print(dest);
        for (t, x) in inner {
            t.print(dest);
            x.print(dest);
        }
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();
        if let Some(first) = &self.first {
            f.entry(&first);
        }
        f.entries(self.inner.iter().flat_map(|(tr, x)| [tr as &dyn Debug, x]))
            .finish()
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

pub struct ItemMod {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: Token![mod],
    pub t1: Trivia,
    pub name: Ident,
    pub t2: Trivia,
    pub semi: Option<Token![;]>,
    pub content: Option<Braces<Module>>,
}

impl Debug for ItemMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ItemMod {
            vis,
            kw,
            t1,
            name,
            t2,
            semi,
            content,
        } = self;
        let mut f = f.debug_struct("ItemMod");
        if let Some(vis) = vis {
            f.field("vis", vis);
        }
        f.field("kw", kw)
            .field("t1", t1)
            .field("name", name)
            .field("t2", t2);
        if let Some(semi) = semi {
            f.field("semi", semi);
        }
        if let Some(content) = content {
            f.field("content", content);
        }
        f.finish()
    }
}

#[derive(Debug)]
pub struct PathSegment {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct Path {
    pub leading_colon: Option<(Token![::], Trivia)>,
    pub seg1: PathSegment,
    pub rest: Vec<(Trivia, Token![::], Trivia, PathSegment)>,
}

#[derive(Debug)]
pub struct VisRestricted {
    pub t2: Trivia,
    pub in_: Option<(Token![in], Trivia)>,
    pub path: Path,
    pub t3: Trivia,
}

#[derive(Debug)]
pub enum Visibility {
    Public(Token![pub]),
    Restricted {
        pub_: Token![pub],
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
