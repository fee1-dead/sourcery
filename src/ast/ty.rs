use sourcery_derive::Walk;

use super::Path;
use crate::ast::{Brackets, Expr, Trivia, Token};
use crate::Print;

#[derive(Debug, Print, Walk)]
pub struct TyArray {
    pub t1: Trivia,
    pub elem: Box<Ty>,
    pub t2: Trivia,
    pub semi: Token![;],
    pub t3: Trivia,
    pub len: Expr,
    pub t4: Trivia,
}

#[derive(Debug, Print, Walk)]
pub struct TySlice {
    pub t1: Trivia,
    pub ty: Box<Ty>,
    pub tlast: Trivia,
}

#[derive(Debug, Print, Walk)]
pub enum Ty {
    Path(Path),
    Slice(Brackets<TySlice>),
    Array(Brackets<TyArray>)
}
