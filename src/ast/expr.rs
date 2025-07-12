use crate::TrivialPrint;
use crate::ast::{Attribute, List};

use super::Literal;

#[derive(Debug, TrivialPrint!)]
pub enum ExprKind {
    Literal(Literal),
}

#[derive(Debug, TrivialPrint!)]
pub struct Expr {
    pub attributes: List<Attribute>,
    pub kind: ExprKind,
}
