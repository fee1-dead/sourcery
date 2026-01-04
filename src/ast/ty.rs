use sourcery_derive::{Respace, Walk};

use crate::ast::{Brackets, Expr, QPath, Token, Trivia};
use crate::Print;

// [Ty; N]
#[derive(Debug, Print, Walk, Respace)]
pub struct TyArray {
    #[sourcery(spaces = 0)]
    pub t1: Trivia,
    pub elem: Box<Ty>,
    #[sourcery(spaces = 0)]
    pub t2: Trivia,
    pub semi: Token![;],
    #[sourcery(spaces = 1)]
    pub t3: Trivia,
    pub len: Box<Expr>,
    #[sourcery(spaces = 0)]
    pub t4: Trivia,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct TySlice {
    #[sourcery(spaces = 0)]
    pub t1: Trivia,
    pub ty: Box<Ty>,
    #[sourcery(spaces = 0)]
    pub tlast: Trivia,
}

#[derive(Debug, Print, Walk, Respace)]
pub enum Ty {
    Path(QPath),
    Slice(Brackets<TySlice>),
    Array(Brackets<TyArray>)
}
