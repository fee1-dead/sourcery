use crate::TrivialPrint;
use crate::ast::token::grouping::Brackets;
use crate::ast::{Delimited, Expr, Path, Token, Trivia};
use crate::parse::TokenStream;

#[derive(Debug, TrivialPrint!)]
pub struct Attribute {
    pub pound: Token![#],
    pub style: AttributeStyle,
    pub t1: Trivia,
    pub inner: Brackets<AttributeInner>,
}

#[derive(Debug, TrivialPrint!)]
pub struct AttributeInner {
    pub t2: Trivia,
    pub path: Path,
    pub value: AttributeValue,
    pub tlast: Trivia,
}

#[derive(Debug, TrivialPrint!)]
pub enum AttributeStyle {
    Outer,
    Inner(Trivia, Token![!]),
}

#[derive(Debug, TrivialPrint!)]
pub enum AttributeValue {
    None,
    Value {
        t3: Trivia,
        eq: Token![=],
        t4: Trivia,
        expr: Expr,
    },
    List(Trivia, Delimited<TokenStream>),
}
