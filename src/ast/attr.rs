use sourcery_derive::{Respace, Walk};

use crate::Print;
use crate::ast::token::grouping::Brackets;
use crate::ast::{Delimited, Expr, List, Path, Token, Trivia};
use crate::parse::TokenStream;

#[derive(Debug, Print, Walk, Respace)]
pub struct Attribute {
    pub pound: Token![#],
    pub style: AttributeStyle,
    #[sourcery(spaces = 0)]
    pub t1: Trivia,
    pub inner: Brackets<AttributeInner>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct AttributeInner {
    #[sourcery(spaces = 0)]
    pub t2: Trivia,
    pub path: Path,
    pub value: AttributeValue,
    #[sourcery(spaces = 0)]
    pub tlast: Trivia,
}

#[derive(Debug, Print, Walk, Respace)]
pub enum AttributeStyle {
    Outer,
    Inner(#[sourcery(spaces = 0)] Trivia, Token![!]),
}

#[derive(Debug, Print, Walk, Respace)]
pub enum AttributeValue {
    None,
    Value {
        #[sourcery(spaces = 1)]
        t3: Trivia,
        eq: Token![=],
        #[sourcery(spaces = 0)]
        t4: Trivia,
        expr: Expr,
    },
    List(#[sourcery(spaces = 0)] Trivia, #[sourcery(spaces = "ignore")] Delimited<TokenStream>),
}

use crate::passes::style::spaces::*; 

// todo, these should be newlines instead
impl Respace for List<Attribute> {
    fn respace(&mut self, v: &mut Spaces) {
        let (mut attrs, mut last) = self.take().into_parts();
        for (a, t) in &mut attrs {
            a.respace(v);
            s1(t);
        }
        if attrs.is_empty() {
            s0(&mut last);
        } else {
            s1(&mut last);
        }

        *self = List::from_parts(attrs, last);
    }
}

