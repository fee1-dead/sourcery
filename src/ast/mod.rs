use std::fmt::Debug;

mod attr;
pub use attr::{Attribute, AttributeInner, AttributeStyle, AttributeValue};
mod expr;
pub use expr::{Expr, ExprKind};
mod token;
pub use token::grouping::{Braces, Brackets, Delimited, Delimiter, Parens};
pub use token::{Ident, Literal, Trivia, Trivium};
pub use token::{Token, kw, tokens};
mod item;
pub use item::{Item, ItemKind, Mod, TyAlias, Fn, FnParam, FnRet};
mod ty;
pub use ty::{ArrayTy, Ty};
mod stmt;
pub use stmt::{Stmt, StmtKind, Block, BlockInner};
mod pat;
pub use pat::Pat;

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

    pub fn push_value(&mut self, x: T) {
        self.optimize();
        assert!(self.tlast.is_empty());
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
