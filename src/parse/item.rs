use crate::ast::{
    Attribute, Braces, Delimiter, Item, ItemKind, List, Mod, Token, Trivia, TyAlias, Visibility,
};
use crate::parse::attr::AttrKind;
use crate::parse::{Parser, Punct};

fn juggle_trivia(
    attrs: Option<(Trivia, List<Attribute>)>,
    vis: Option<(Trivia, Visibility)>,
    tbeforekw: Trivia,
) -> (Trivia, List<Attribute>, Option<(Visibility, Trivia)>) {
    match (attrs, vis) {
        (Some((t0, mut attrs)), Some((tsquash, vis))) => {
            attrs.push_trivia(tsquash);
            (t0, attrs, Some((vis, tbeforekw)))
        }
        (Some((t0, attrs)), None) => (t0, attrs, None),
        (None, Some((t0, vis))) => (t0, List::default(), Some((vis, tbeforekw))),
        (None, None) => (tbeforekw, List::default(), None),
    }
}

impl Parser<'_> {
    pub fn parse_item(&mut self) -> (Trivia, Item) {
        let attrs = self.parse_attrs(AttrKind::Inner);
        let vis = self.parse_vis();
        if let Some((tbeforemod, _)) = self.eat_ident("mod") {
            let (t0, attrs, vis) = juggle_trivia(attrs, vis, tbeforemod);
            let (t1, name) = self.parse_ident();
            let (t2, semi, content) = if let Some(t2) = self.eat_punct(Punct::Semi) {
                (t2, Some(Token![;]), None)
            } else if let Some((t2, module)) =
                self.eat_delim(Delimiter::Braces, |t2, mut this| (t2, this.parse_module()))
            {
                (t2, None, Some(Braces(module)))
            } else {
                unimplemented!()
            };

            let kind = ItemKind::Mod(Mod {
                vis,
                kw: Token![mod],
                t1,
                name,
                t2,
                semi,
                content,
            });
            (t0, Item { attrs, kind })
        } else if let Some((tbeforetype, _)) = self.eat_ident("type") {
            let (t0, attrs, vis) = juggle_trivia(attrs, vis, tbeforetype);
            let (t1, name) = self.parse_ident();
            let t2 = self.eat_punct(Punct::Eq).unwrap();
            let (t3, ty) = self.parse_ty();
            let t4 = self.eat_punct(Punct::Semi).unwrap();
            let kind = ItemKind::TyAlias(TyAlias {
                vis,
                kw: Token![type],
                t1,
                name,
                t2,
                eq: Token![=],
                t3,
                ty,
                t4,
                semi: Token![;],
            });
            (t0, Item { attrs, kind })
        } else {
            unimplemented!("{:?}", self.token)
        }
    }
}
