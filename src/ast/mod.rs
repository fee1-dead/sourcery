use std::fmt::Debug;

mod attr;
pub use attr::{Attribute, AttributeInner, AttributeStyle, AttributeValue};
mod expr;
pub use expr::{Expr, ExprKind};
mod token;
pub use token::grouping::{Braces, Brackets, Delimited, Delimiter, Parens};
pub use token::{Ident, Literal, Trivia, Trivium};
pub use token::{Token, kw, tokens};
mod ty;

use crate::TrivialPrint;
use crate::print::Print;

/// A list of items separated by trivia. Does not contain leading trivia
/// but may contain trailing trivia.
#[derive(Clone)]
pub struct List<T> {
    inner: Vec<(T, Trivia)>,
    tlast: Trivia,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            inner: vec![],
            tlast: Trivia::default(),
        }
    }
}

impl<T: Print> Print for List<T> {
    fn print(&self, dest: &mut String) {
        let List { inner, tlast } = self;
        inner.print(dest);
        tlast.print(dest);
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();
        f.entries(
            self.inner
                .iter()
                .flat_map(|(tr, x)| [tr as &dyn Debug, x])
                .chain((!self.tlast.is_empty()).then_some(&self.tlast as &dyn Debug)),
        )
        .finish()
    }
}

impl<T> List<T> {
    pub fn single(x: T) -> List<T> {
        let mut l = List::default();
        l.push_value(x);
        l
    }

    fn optimize(&mut self) {
        if let Some((_, t)) = self.inner.last_mut() {
            t.list.extend(std::mem::take(&mut self.tlast.list))
        }
    }

    fn push_value(&mut self, x: T) {
        self.optimize();
        self.inner.push((x, Trivia::default()))
    }

    pub fn push(&mut self, t: Trivia, x: T) {
        self.push_trivia(t);
        self.push_value(x);
    }

    pub fn push_trivia(&mut self, t: Trivia) {
        self.tlast.list.extend(t.list);
    }

    pub fn into_parts(self) -> (Vec<(T, Trivia)>, Trivia) {
        (self.inner, self.tlast)
    }
}

#[derive(TrivialPrint!)]
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

#[derive(Debug, TrivialPrint!)]
pub struct PathSegment {
    pub ident: Ident,
}

#[derive(Debug, TrivialPrint!)]
pub struct Path {
    pub leading_colon: Option<(Token![::], Trivia)>,
    pub seg1: PathSegment,
    pub rest: Vec<(Trivia, Token![::], Trivia, PathSegment)>,
}

#[derive(Debug, TrivialPrint!)]
pub struct VisRestricted {
    pub t2: Trivia,
    pub in_: Option<(Token![in], Trivia)>,
    pub path: Path,
    pub t3: Trivia,
}

#[derive(Debug, TrivialPrint!)]
pub enum Visibility {
    Public {
        pub_: Token![pub],
    },
    Restricted {
        pub_: Token![pub],
        t1: Trivia,
        parens: Parens<VisRestricted>,
    },
}

#[derive(Debug, TrivialPrint!)]
pub enum ItemKind {
    Mod(ItemMod),
}

#[derive(Debug, TrivialPrint!)]
pub struct Item {
    pub attrs: List<Attribute>,
    pub kind: ItemKind,
}

#[derive(Debug, TrivialPrint!)]
pub struct Module {
    pub t1: Trivia,
    pub attrs: List<Attribute>,
    pub items: List<Item>,
}

#[derive(Debug, TrivialPrint!)]
pub struct File {
    // shebang, frontmatter
    pub module: Module,
}
