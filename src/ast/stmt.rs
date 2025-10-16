use sourcery_derive::Walk;

use crate::ast::{Attribute, Braces, Expr, List, Token, Trivia, Print};

#[derive(Debug, Print, Walk)]
pub struct Stmt {
    pub attrs: List<Attribute>,
    pub kind: StmtKind,
}

#[derive(Debug, Print, Walk)]
pub struct BlockInner {
    pub t0: Trivia,
    pub stmts: List<Stmt>,
}

pub type Block = Braces<BlockInner>;

#[derive(Debug, Print, Walk)]
pub enum StmtKind {
    Empty(Token![;]),
    Semi(Expr, Trivia, Token![;]),
    Expr(Expr),
}
