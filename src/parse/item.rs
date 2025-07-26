use crate::ast::{
    Attribute, Braces, Const, Delimiter, Fn, FnParam, FnRet, Item, ItemKind, List, Mod, Parens,
    Token, Trivia, TyAlias, Visibility,
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
    fn parse_fn_params(&mut self) -> (Trivia, Parens<(Trivia, List<FnParam>)>) {
        self.eat_delim(Delimiter::Parens, |t0, mut this| {
            if let Some(last) = this.eat_eof() {
                return (t0, Parens((last, List::default())));
            }
            let mut list = List::default();
            let mut tfirst = None;
            loop {
                let (tattr, mut attrs) = this.parse_attrs(AttrKind::Outer).unwrap_or_default();
                let (tbeforepat, pat) = this.parse_pat();
                attrs.push_trivia(tbeforepat);
                let t1 = this.eat_punct(Punct::Colon).unwrap();
                let (t2, ty) = this.parse_ty();
                let comma = this.eat_punct(Punct::Comma).map(|c| (c, Token![,]));
                let has_comma = comma.is_some();
                let p = FnParam {
                    attrs,
                    pat,
                    t1,
                    colon: Token![:],
                    t2,
                    ty,
                    comma,
                };

                if tfirst.is_none() {
                    tfirst = Some(tattr);
                    list = List::single(p);
                } else {
                    list.push(tattr, p);
                }
                let eof = this.eat_eof();
                if !has_comma || eof.is_some() {
                    if let Some(tlast) = eof {
                        list.push_trivia(tlast);
                    }
                    break (t0, Parens((tfirst.unwrap_or_default(), list)));
                }
            }
        })
        .unwrap()
    }

    fn parse_fn_ret(&mut self) -> Option<(Trivia, FnRet)> {
        let t1 = self.eat_punct(Punct::RArrow)?;
        let (t2_5, ty) = self.parse_ty();
        Some((
            t1,
            FnRet {
                arrow: Token![->],
                t2_5,
                ty,
            },
        ))
    }

    pub fn parse_item_mod(&mut self, vis: Option<(Visibility, Trivia)>) -> Mod {
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

        Mod {
            vis,
            kw: Token![mod],
            t1,
            name,
            t2,
            semi,
            content,
        }
    }
    pub fn parse_item_ty_alias(&mut self, vis: Option<(Visibility, Trivia)>) -> TyAlias {
        let (t1, name) = self.parse_ident();
        let t2 = self.eat_punct(Punct::Eq).unwrap();
        let (t3, ty) = self.parse_ty();
        let t4 = self.eat_punct(Punct::Semi).unwrap();
        TyAlias {
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
        }
    }
    pub fn parse_item_const(&mut self, vis: Option<(Visibility, Trivia)>) -> Const {
        let (t1, name) = self.parse_ident();
        let t2 = self.eat_punct(Punct::Colon).unwrap();
        let (t3, ty) = self.parse_ty();
        let t4 = self.eat_punct(Punct::Eq).unwrap();
        let (t5, expr) = self.parse_expr();
        let t6 = self.eat_punct(Punct::Semi).unwrap();
        Const {
            vis,
            t1,
            kw: Token![const],
            name,
            t2,
            colon: Token![:],
            t3,
            ty,
            t4,
            eq: Token![=],
            t5,
            expr,
            t6,
            semi: Token![;],
        }
    }
    pub fn parse_item(&mut self) -> (Trivia, Item) {
        let attrs = self.parse_attrs(AttrKind::Outer);
        let vis = self.parse_vis();
        if let Some((tbeforemod, _)) = self.eat_ident("mod") {
            let (t0, attrs, vis) = juggle_trivia(attrs, vis, tbeforemod);
            let kind = ItemKind::Mod(self.parse_item_mod(vis));
            (t0, Item { attrs, kind })
        } else if let Some((tbeforetype, _)) = self.eat_ident("type") {
            let (t0, attrs, vis) = juggle_trivia(attrs, vis, tbeforetype);
            let kind = ItemKind::TyAlias(self.parse_item_ty_alias(vis));
            (t0, Item { attrs, kind })
        } else if let Some((tbeforefn, _)) = self.eat_ident("fn") {
            // TODO parse leading modifiers (unsafe, const, extern)
            let (t0, attrs, vis) = juggle_trivia(attrs, vis, tbeforefn);
            let (t1, name) = self.parse_ident();
            let (t2, params) = self.parse_fn_params();
            let ret = self.parse_fn_ret();
            let (t3, block) = self.parse_block();
            let kind = ItemKind::Fn(Fn {
                vis,
                kw: Token![fn],
                t1,
                name,
                t2,
                params,
                ret,
                t3,
                block,
            });
            (t0, Item { attrs, kind })
        } else if let Some((tbeforeconst, _)) = self.eat_ident("const") {
            let (t0, attrs, vis) = juggle_trivia(attrs, vis, tbeforeconst);
            let kind = ItemKind::Const(self.parse_item_const(vis));
            (t0, Item { attrs, kind })
        } else {
            unimplemented!("{:?}", self.token)
        }
    }
}
