use crate::ast::{Attribute, List};

use super::Literal;

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug)]
pub struct Expr {
    pub attributes: List<Attribute>,
    pub kind: ExprKind,
}
