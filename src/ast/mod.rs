use std::fmt::Debug;

mod attr;
pub use attr::{Attribute, AttributeInner, AttributeStyle, AttributeValue};
mod expr;
pub use expr::*;
mod token;
pub use token::grouping::{Braces, Brackets, Delimited, Delimiter, Parens};
pub use token::{Ident, Literal, Trivia, TriviaN, Trivium};
pub use token::{Token, kw, tokens};
mod item;
pub use item::*;
mod ty;
pub use ty::{Ty, TyArray, TySlice};
mod stmt;
pub use stmt::*;
mod pat;
pub use pat::Pat;
mod path;
pub use path::*;
mod macros;
pub use macros::*;


use crate::prelude::*;

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
            t.extend(self.tlast.take())
        }
    }

    pub fn take(&mut self) -> List<T> {
        List { inner: std::mem::take(&mut self.inner), tlast: self.tlast.take() }
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
        self.tlast.extend(t);
    }

    pub fn into_parts(self) -> (Vec<(T, Trivia)>, Trivia) {
        (self.inner, self.tlast)
    }

    pub fn from_parts(inner: Vec<(T, Trivia)>, tlast: Trivia) -> Self {
        let mut this = List { inner, tlast };
        this.optimize();
        this
    }
}

impl<T: crate::passes::Visit> crate::passes::Walk for List<T> {
    fn walk<P: crate::passes::Pass + ?Sized>(&mut self, p: &mut P) {
        for (x, tr) in &mut self.inner {
            x.visit(p);
            p.visit_trivia(tr);
        }
        p.visit_trivia(&mut self.tlast);
    }
}

#[derive(Clone)]
enum SeparatedListInner<T, S> {
    Empty,
    NonEmpty {
        first: Box<T>,
        rest: Vec<(Trivia, S, Trivia, T)>,
        trailing: Option<L<S>>,
    }
}

/// A list of items separated by a punctuation and trivia. Does not contain leading trivia,
/// may contain trailing trivia, and may contain trailing comma. Example:
/// `( a, b, c, d, e, )`
/// This list represents `a, b, c, d, e, `. So you just need to make it
/// `Parens<(Trivia, SeparatedList<Ident, Token![,]>)>` to fully represent the source.
#[derive(Clone)]
pub struct SeparatedList<T, S> {
    inner: SeparatedListInner<T, S>,
    tlast: Trivia,
}

impl<T, S> SeparatedList<T, S> {
    pub fn new() -> SeparatedList<T, S> {
        SeparatedList { inner: SeparatedListInner::Empty, tlast: Trivia::default() }
    }

    pub fn new_single(x: T) -> SeparatedList<T, S> {
        SeparatedList { inner: SeparatedListInner::NonEmpty { first: Box::new(x), rest: Vec::new(), trailing: None }, tlast: Trivia::default() }
    }

    pub fn push_value(&mut self, t: Trivia, x: T) {
        match &mut self.inner {
            SeparatedListInner::Empty => {
                panic!("cannot push to an empty separated list, use new_single")
            }
            SeparatedListInner::NonEmpty { first: _, rest, trailing } => {
                let L(t0, s) = trailing.take().unwrap();
                rest.push((t0, s, t, x));
            }
        }
    }

    pub fn push_sep(&mut self, t: Trivia, s: S) {
        match &mut self.inner {
            SeparatedListInner::Empty => {
                panic!("cannot push to an empty separated list, use new_single")
            }
            SeparatedListInner::NonEmpty { first: _, rest: _, trailing } => {
                assert!(trailing.is_none(), "should not already have trailing separator");
                *trailing = Some(t << s)
            }
        }
    }

    pub fn push_trivia(&mut self, t: Trivia) {
        self.tlast.extend(t);
    }
}

pub struct SeparatedListBuilder<T, S> {
    t1: Trivia,
    l: SeparatedList<T, S>
}

impl<T, S> SeparatedListBuilder<T, S> {
    pub fn new() -> Self {
        Self { t1: Trivia::default(), l: SeparatedList::new() }
    }
    pub fn push_value(&mut self, t: Trivia, x: T) {
        match self.l.inner {
            SeparatedListInner::Empty => {
                self.t1 = t;
                self.l.inner = SeparatedListInner::NonEmpty { first: Box::new(x), rest: Vec::new(), trailing: None };
            }
            SeparatedListInner::NonEmpty { .. } => {
                self.l.push_value(t, x);
            }
        }
    }
    pub fn push_sep(&mut self, t: Trivia, s: S) {
        self.l.push_sep(t, s);
    }
    pub fn build(self) -> L<SeparatedList<T, S>> {
        self.t1 << self.l
    }
}

impl<T: Print, S: Print> Print for SeparatedList<T, S> {
    fn print(&self, dest: &mut String) {
        if let SeparatedListInner::NonEmpty { first, rest, trailing } = &self.inner {
            first.print(dest);
            rest.print(dest);
            trailing.print(dest);
        }
        self.tlast.print(dest)
    }
}

impl<T: Visit, S: Visit> Walk for SeparatedList<T, S> {
    fn walk<P: crate::passes::Pass + ?Sized>(&mut self, p: &mut P) {
        if let SeparatedListInner::NonEmpty { first, rest, trailing } = &mut self.inner {
            first.visit(p);
            for (t, s, t2, x) in rest {
                p.visit_trivia(t);
                s.visit(p);
                p.visit_trivia(t2);
                x.visit(p);
            }
            trailing.visit(p);
        }
        p.visit_trivia(&mut self.tlast);
    }
}

impl<T: Debug, S: Debug> Debug for SeparatedList<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();
        if let SeparatedListInner::NonEmpty { first, rest, trailing } = &self.inner {
            f.entry(first);
            f.entries(rest.iter().flat_map(|(a, b, c, d)| [a as &dyn Debug, b, c, d]));
            if let Some(L(a, b)) = trailing {
                if !a.is_empty() {
                    f.entry(a);
                }
                f.entry(b);
            }
        }

        if !self.tlast.is_empty() {
            f.entry(&self.tlast);
        }
        f.finish()
    }
}


#[derive(Debug, Print, Walk)]
pub struct VisRestricted {
    pub t2: Trivia,
    pub in_: Option<(Token![in], TriviaN)>,
    pub path: Path,
    pub t3: Trivia,
}

#[derive(Debug, Print, Walk)]
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

#[derive(Debug, Print, Walk)]
pub struct Module {
    pub t1: Trivia,
    pub attrs: List<Attribute>,
    pub items: List<Item>,
}

#[derive(Debug, Print, Walk)]
pub struct File {
    // shebang, frontmatter
    pub module: Module,
}
