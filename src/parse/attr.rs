use ra_ap_rustc_lexer::TokenKind;

use crate::ast::{Attribute, AttributeInner, AttributeStyle, AttributeValue, Brackets, List, Token, Trivia};
use crate::parse::Parser;

impl<'src> Parser<'src> {
    pub fn maybe_parse_attr(&mut self) -> Option<(Trivia, Attribute)> {
        let t0 = self.eat(TokenKind::Pound)?;
        let style = if let Some(t) = self.eat(TokenKind::Bang) {
            AttributeStyle::Inner(t, Token![!])
        } else {
            AttributeStyle::Outer
        };
        let t1 = self.eat(TokenKind::OpenBracket).unwrap();
        
        let inner = self.parse_attr_inner();
        Some((t0, Attribute {
            pound: Token![#],
            style,
            t1,
            inner: Brackets(inner),
        }))
    }
    pub fn parse_attr_inner(&mut self) -> AttributeInner {
        let (t2, path) = self.parse_path();
        let value = if let Some(t3) = self.eat(TokenKind::Eq) {
            // TODO
            let (t4, expr) = self.parse_atom_expr();
            AttributeValue::Value { t3, eq: Token![=], t4, expr }
        } else {
            AttributeValue::None
        };
        let tlast = self.eat(TokenKind::CloseBracket).unwrap();
        AttributeInner { t2, path, value, tlast }
    }
    pub fn parse_attrs(&mut self) -> Option<(Trivia, List<Attribute>)> {
        let (t0, attr) = self.maybe_parse_attr()?;
        let mut list = List::single(attr);
        while let Some((t, attr)) = self.maybe_parse_attr() {
            list.push(t, attr);
        }
        Some((t0, list))
    }
}
