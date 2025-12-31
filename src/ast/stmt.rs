use crate::prelude::*;

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

// label with trailing trivia
#[derive(Debug, Print, Walk, Respace)]
pub struct Label {
    pub lt: Ident,
    #[sourcery(spaces = 0)]
    pub t1: Trivia,
    pub colon: Token![:],
    #[sourcery(spaces = 1)]
    pub t2: Trivia,
}

pub type Block = Braces<BlockInner>;

#[derive(Debug, Print, Walk, Respace)]
pub struct LabeledBlock {
    pub label: Option<Label>,
    pub block: Block,
}

impl Visit for LabeledBlock {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        self.label.visit(p);
        self.block.visit(p);
    }
}

#[derive(Debug, Print, Walk)]
pub enum StmtKind {
    Empty(Token![;]),
    Semi(Expr, Trivia, Token![;]),
    Expr(Expr),
}

impl Respace for Block {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}
