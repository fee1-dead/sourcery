use crate::grouping::Braces;
use crate::{token, Ident, Trivia};

#[derive(Debug)]
pub struct List<T> {
    inner: Vec<(T, Trivia)>,

    last: Option<Box<T>>,
}

#[derive(Debug)]
pub struct ItemMod {
    pub kw: token![mod],
    pub t1: Trivia,
    pub name: Ident,
    pub t2: Trivia,
    pub semi: Option<token![;]>,
    pub content: Option<Braces<Module>>,
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


