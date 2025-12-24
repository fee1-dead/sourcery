use sourcery_derive::{Respace, Walk};

use crate::Print;
use super::{Attribute, List, Literal};

#[derive(Debug, Print, Walk, Respace)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
