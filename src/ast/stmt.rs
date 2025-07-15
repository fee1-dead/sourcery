use crate::ast::{Attribute, Braces, Expr, List, Token, Trivia, TrivialPrint};

#[derive(Debug, TrivialPrint!)]
pub struct Stmt {
    pub attrs: List<Attribute>,
    pub kind: StmtKind,
}

#[derive(Debug, TrivialPrint!)]
pub struct BlockInner {
    pub t0: Trivia,
    pub stmts: List<Stmt>,
}

pub type Block = Braces<BlockInner>;

#[derive(Debug, TrivialPrint!)]
pub enum StmtKind {
    Empty(Token![;]),
    Semi(Expr, Trivia, Token![;]),
    Expr(Expr),
}
