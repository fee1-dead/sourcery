use std::fmt::Debug;
use crate::ast::Ty;
use crate::TrivialPrint;
use super::{List, Attribute, Trivia, Ident, Visibility, Braces, Module, Token};

#[derive(Debug, TrivialPrint!)]
pub enum ItemKind {
    Mod(Mod),
    TyAlias(TyAlias),
}

#[derive(Debug, TrivialPrint!)]
pub struct Item {
    pub attrs: List<Attribute>,
    pub kind: ItemKind,
}
 
#[derive(TrivialPrint!)]
pub struct Mod {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: Token![mod],
    pub t1: Trivia,
    pub name: Ident,
    pub t2: Trivia,
    pub semi: Option<Token![;]>,
    pub content: Option<Braces<Module>>,
}

impl Debug for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Mod {
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
pub struct TyAlias {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: Token![type],
    pub t1: Trivia,
    pub name: Ident,
    pub t2: Trivia,
    pub eq: Token![=],
    pub t3: Trivia,
    pub ty: Ty,
    pub t4: Trivia,
    pub semi: Token![;],
}
