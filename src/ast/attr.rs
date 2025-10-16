use sourcery_derive::Walk;

use crate::TrivialPrint;
use crate::ast::token::grouping::Brackets;
use crate::ast::{Delimited, Expr, Path, Token, Trivia};
use crate::parse::TokenStream;

#[derive(Debug, TrivialPrint!, Walk)]
pub struct Attribute {
    pub pound: Token![#],
    pub style: AttributeStyle,
    pub t1: Trivia,
    pub inner: Brackets<AttributeInner>,
}

#[derive(Debug, TrivialPrint!, Walk)]
pub struct AttributeInner {
    pub t2: Trivia,
    pub path: Path,
    pub value: AttributeValue,
    pub tlast: Trivia,
}

#[derive(Debug, TrivialPrint!, Walk)]
pub enum AttributeStyle {
    Outer,
    Inner(Trivia, Token![!]),
}

#[derive(Debug, TrivialPrint!, Walk)]
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
