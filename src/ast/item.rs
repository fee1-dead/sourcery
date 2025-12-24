use std::fmt::Debug;
use sourcery_derive::{Respace, Walk};

use crate::ast::tokens::Semi;
use crate::ast::{Block, Expr, Parens, Pat, TriviaN, Ty};
use crate::Print;
use super::{List, Attribute, Trivia, Ident, Visibility, Braces, Module, Token};

#[derive(Debug, Print, Walk)]
pub enum ItemKind {
    Const(Const),
    Mod(Mod),
    TyAlias(TyAlias),
    Fn(Fn),
}

#[derive(Debug, Print, Walk)]
pub struct Item {
    pub attrs: List<Attribute>,
    pub kind: ItemKind,
}
 
#[derive(Print, Walk)]
pub struct Mod {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: Token![mod],
    pub t1: TriviaN,
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

#[derive(Debug, Print, Walk)]
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

#[derive(Debug, Print, Walk)]
pub struct FnParam {
    pub attrs: List<Attribute>,
    pub pat: Pat,
    pub t1: Trivia,
    pub colon: Token![:],
    pub t2: Trivia,
    pub ty: Ty,
    pub comma: Option<(Trivia, Token![,])>,
}

#[derive(Debug, Print, Walk)]
pub struct FnRet {
    pub arrow: Token![->],
    pub t2_5: Trivia,
    pub ty: Ty,
}

#[derive(Debug, Print, Walk)]
pub struct Fn {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: Token![fn],
    pub t1: Trivia,
    pub name: Ident,
    pub t2: Trivia,
    pub params: Parens<(Trivia, List<FnParam>)>,
    pub ret: Option<(Trivia, FnRet)>,
    pub t3: Trivia,
    pub block: Block,
}

// pub const NAME: Ty = expr;
#[derive(Debug, Print, Walk, Respace)]
pub struct Const {
    pub vis: Option<(Visibility, Trivia)>,
    pub kw: Token![const],
    #[sourcery(spaces = 1)]
    pub t1: TriviaN,
    pub name: Ident,
    #[sourcery(spaces = 0)]
    pub t2: Trivia,
    pub colon: Token![:],
    #[sourcery(spaces = 1)]
    pub t3: Trivia,
    pub ty: Ty,
    #[sourcery(spaces = 1)]
    pub t4: Trivia,
    pub eq: Token![=],
    #[sourcery(spaces = 1)]
    pub t5: Trivia,
    pub expr: Expr,
    #[sourcery(spaces = 0)]
    pub t6: Trivia,
    pub semi: Semi,
}
