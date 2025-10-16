use sourcery_derive::Walk;

use crate::TrivialPrint;

use super::{Attribute, List, Literal};

#[derive(Debug, TrivialPrint!, Walk)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug, TrivialPrint!, Walk)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
