use sourcery_derive::Walk;

use crate::Print;

use super::{Attribute, List, Literal};

#[derive(Debug, Print, Walk)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug, Print, Walk)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
