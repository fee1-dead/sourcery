use super::Path;
use crate::ast::{Brackets, Expr, Trivia, Token};
use crate::TrivialPrint;

#[derive(Debug, TrivialPrint!)]
pub struct ArrayTy {
    pub elem: Box<Ty>,
    pub t2: Trivia,
    pub semi: Token![;],
    pub t3: Trivia,
    pub len: Expr,
}

#[derive(Debug, TrivialPrint!)]
pub enum Ty {
    Path(Path),
    Slice(Brackets<(Trivia, Box<Ty>, Trivia)>),
    Array(Brackets<(Trivia, ArrayTy, Trivia)>)
}
