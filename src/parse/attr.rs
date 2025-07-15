use crate::ast::{
    Attribute, AttributeInner, AttributeStyle, AttributeValue, Brackets, Delimiter, List, Token,
    Trivia,
};
use crate::parse::{Parser, Punct, TokenTree};

#[derive(Clone, Copy)]
pub enum AttrKind {
    /// An attribute at the outer of an item, `#[inline]`
    Outer,
    /// An attribute inside something, `#![feature(const_trait_impl)]` inside a module
    Inner,
}

impl<'src> Parser<'src> {
    pub fn maybe_parse_attr(&mut self, kind: AttrKind) -> Option<(Trivia, Attribute)> {
        let is_attr = self.check_punct(Punct::Pound)
            && match kind {
                AttrKind::Outer => self.peek_nth(1, |(_, t)| t.is_delim(Delimiter::Brackets)),
                AttrKind::Inner => {
                    self.peek_nth(1, |(_, t)| matches!(t, TokenTree::Punct(Punct::Bang)))
                        && self.peek_nth(2, |(_, t)| t.is_delim(Delimiter::Brackets))
                }
            };
        if !is_attr {
            return None;
        }
        let t0 = self.eat_punct(Punct::Pound)?;
        let style = if let Some(t) = self.eat_punct(Punct::Bang) {
            AttributeStyle::Inner(t, Token![!])
        } else {
            AttributeStyle::Outer
        };
        let (t1, inner) = self
            .eat_delim(Delimiter::Brackets, |t1, mut this| {
                (t1, this.parse_attr_inner())
            })
            .unwrap();

        Some((
            t0,
            Attribute {
                pound: Token![#],
                style,
                t1,
                inner: Brackets(inner),
            },
        ))
    }
    pub fn parse_attr_inner(&mut self) -> AttributeInner {
        let (t2, path) = self.parse_path();
        let value = if let Some(t3) = self.eat_punct(Punct::Eq) {
            let (t4, expr) = self.parse_expr();
            AttributeValue::Value {
                t3,
                eq: Token![=],
                t4,
                expr,
            }
        } else if let Some((val, tt)) = self.eat_delimited() {
            AttributeValue::List(val, tt)
        } else {
            AttributeValue::None
        };
        let tlast = self.eat_eof().unwrap();
        AttributeInner {
            t2,
            path,
            value,
            tlast,
        }
    }
    pub fn parse_attrs(&mut self, kind: AttrKind) -> Option<(Trivia, List<Attribute>)> {
        let (t0, attr) = self.maybe_parse_attr(kind)?;
        let mut list = List::single(attr);
        while let Some((t, attr)) = self.maybe_parse_attr(kind) {
            list.push(t, attr);
        }
        Some((t0, list))
    }
}
