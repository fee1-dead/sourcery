use sourcery_derive::{Respace, Walk};

use crate::Print;
use crate::ast::stmt::LabeledBlock;
use crate::ast::{Block, Trivia, Token};
use super::{Attribute, List, Literal};

#[derive(Debug, Print, Walk, Respace)]
pub enum ExprKind {
    Literal(Literal),
    Block(LabeledBlock),
    AsyncBlock(AsyncBlock),
    TryBlock(TryBlock),
    If(IfExpr),
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
pub struct IfExpr {
    pub token: Token![if],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub cond: Box<Expr>,
    #[sourcery(spaces = 1)]
    pub t2: Trivia,
    pub then: Block,
    pub else_: Option<Else>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Else {
    #[sourcery(spaces = 1)]
    pub t3: Trivia,
    pub token: Token![else],
    #[sourcery(spaces = 1)]
    pub t4: Trivia,
    pub kind: ElseKind,
}

#[derive(Debug, Print, Walk, Respace)]
pub enum ElseKind {
    Else(Block),
    ElseIf(Box<IfExpr>),
}


#[derive(Debug, Print, Walk, Respace)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
