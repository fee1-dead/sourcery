use crate::TrivialPrint;

use super::{Attribute, List, Literal};

#[derive(Debug, TrivialPrint!)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug, TrivialPrint!)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
