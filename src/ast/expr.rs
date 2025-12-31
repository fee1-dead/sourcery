use sourcery_derive::{Respace, Walk};

use crate::Print;
use crate::ast::{Block, Trivia, Token};
use super::{Attribute, List, Literal};

#[derive(Debug, Print, Walk, Respace)]
pub enum ExprKind {
    Literal(Literal),
    Block(Block),
    AsyncBlock(AsyncBlock),
    TryBlock(TryBlock),
}

#[derive(Debug, Print, Walk, Respace)]
pub struct AsyncBlock {
    pub token: Token![async],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    // TODO `move`
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct TryBlock {
    pub token: Token![try],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    // TODO `move`
    pub block: Block,
}


#[derive(Debug, Print, Walk, Respace)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
