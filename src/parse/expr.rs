use crate::parse::attr::AttrKind;
use crate::prelude::*;

impl<'src> super::Parser<'src> {
    pub fn parse_expr(&mut self) -> L<Expr> {
        self.parse_expr_inner(true)
    }
    fn peek_expr(&self) -> bool {
        self.peek(|tt| matches!(tt, TokenTree::Ident(i) if i.0 != "as"))
            || self.peek(|tt| {
                matches!(
                    tt,
                    TokenTree::Group(_)
                        | TokenTree::Literal(_)
                        | TokenTree::Lifetime(_)
                        | TokenTree::Punct(
                            Punct::Bang
                                | Punct::Minus
                                | Punct::Star
                                | Punct::Or
                                | Punct::And
                                | Punct::AndAnd
                                | Punct::DotDot
                                | Punct::Lt
                                | Punct::ColonColon
                                | Punct::Pound
                        )
                )
            })
    }
    fn parse_expr_inner(&mut self, allow_struct: bool) -> L<Expr> {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let L(t1, kind) = self.parse_atom_expr(allow_struct);
        // TODO audit every usage of this. It is not semantically correct but it sure is convenient
        attrs.push_trivia(t1);
        t0 << Expr { attrs, kind }
    }
    fn choose_generics_over_qpath(&self) -> bool {
        self.check_punct(Punct::Lt)
            && (self.peek2(|tt| tt.is_punct(Punct::Gt))
                || self.peek2(|tt| tt.is_punct(Punct::Pound))
                || (self.peek2(|tt| matches!(tt, TokenTree::Lifetime(_) | TokenTree::Ident(_)))
                    && self.peek3(|tt| {
                        matches!(
                            tt,
                            TokenTree::Punct(Punct::Gt | Punct::Comma | Punct::Colon | Punct::Eq)
                        )
                    }))
                || self.peek2(|tt| tt.is_ident("const")))
    }
    fn choose_generics_over_qpath_after_keyword(&self) -> bool {
        let mut this = self.snapshot();
        this.parse_ident();
        this.choose_generics_over_qpath()
    }
    fn parse_expr_with_earlier_boundary_rule(&mut self) -> L<Expr> {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        // TODO audit every usage of this. It is not semantically correct but it sure is convenient
        let L(t1, kind) = self
            .parse_expr_if()
            .or_else(|| self.parse_expr_while())
            .or_else(|| {
                (self.check_ident("for") && !self.choose_generics_over_qpath_after_keyword())
                    .then(|| self.parse_expr_for())
                    .flatten()
            })
            .or_else(|| self.parse_expr_loop())
            // .or_else(|| self.parse_expr_match);
            .or_else(|| self.parse_try_block())
            .or_else(|| self.parse_unsafe_block())
            .or_else(|| self.parse_const_block())
            .unwrap_or_else(|| {
                if self.peek(|tt| tt.is_delim(Delimiter::Braces)) {
                    self.parse_block()
                        .map(|block| ExprKind::Block(LabeledBlock { label: None, block }))
                } else if self.peek(|tt| matches!(tt, TokenTree::Lifetime(_))) {
                    self.parse_labeled_atom_expr()
                } else {
                    self.parse_unary_expr(true)
                }
            });
        // TODO continue parsing
        attrs.push_trivia(t1);
        t0 << Expr { attrs, kind }
    }
    fn parse_try_block(&mut self) -> Option<L<ExprKind>> {
        if self.check_ident("try") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("try").unwrap().0;
            let L(t1, block) = self.parse_block();
            Some(
                t << ExprKind::TryBlock(TryBlock {
                    token: Token![try],
                    t1,
                    block,
                }),
            )
        } else {
            None
        }
    }
    fn parse_const_block(&mut self) -> Option<L<ExprKind>> {
        if self.check_ident("const") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("const").unwrap().0;
            let L(t1, block) = self.parse_block();
            Some(
                t << ExprKind::Const(ConstBlock {
                    token: Token![const],
                    t1,
                    block,
                }),
            )
        } else {
            None
        }
    }
    fn parse_unsafe_block(&mut self) -> Option<L<ExprKind>> {
        if self.check_ident("unsafe") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("unsafe").unwrap().0;
            let L(t1, block) = self.parse_block();
            Some(
                t << ExprKind::Unsafe(UnsafeBlock {
                    token: Token![unsafe],
                    t1,
                    block,
                }),
            )
        } else {
            None
        }
    }
    fn parse_array_or_repeat(&mut self) -> Option<L<ExprKind>> {
        self.eat_delim(Delimiter::Brackets, |t0, mut this| {
            if let Some(t1) = this.eat_eof() {
                return t0
                    << ExprKind::Array(Brackets(TupleOrArrayContents {
                        t1,
                        contents: SeparatedList::new(),
                    }));
            }
            let L(t1, first) = this.parse_expr();
            if let Some(t2) = this.eat_punct(Punct::Semi) {
                let L(t3, len) = this.parse_expr().map(Box::new);
                let t4 = this.eat_eof().unwrap();
                t0 << ExprKind::Repeat(Brackets(ExprRepeat {
                    t1,
                    elem: Box::new(first),
                    t2,
                    semi: Token![;],
                    t3,
                    len,
                    t4,
                }))
            } else {
                let mut contents = SeparatedList::new_single(first);
                loop {
                    if let Some(tlast) = this.eat_eof() {
                        contents.push_trivia(tlast);
                        break t0
                            << ExprKind::Array(Brackets(TupleOrArrayContents { t1, contents }));
                    }
                    let tnext = this.eat_punct(Punct::Comma).unwrap();
                    contents.push_sep(tnext, Token![,]);
                    if let Some(tlast) = this.eat_eof() {
                        contents.push_trivia(tlast);
                        break t0
                            << ExprKind::Array(Brackets(TupleOrArrayContents { t1, contents }));
                    }
                    let L(tnext, x) = this.parse_expr();
                    contents.push_value(tnext, x);
                }
            }
        })
    }
    fn parse_closure_arg(&mut self) -> L<ClosureArg> {
        let (mut t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let L(t0_5, pat) = self.parse_pat();
        if attrs.is_empty() {
            t0 = t0_5;
        } else {
            attrs.push_trivia(t0_5);
        }
        let ty = self.eat_punct(Punct::Colon).map(|t1| {
            let L(t2, ty) = self.parse_ty();
            (t1, Token![:], t2, ty)
        });
        let comma = self.eat_punct(Punct::Comma).map(|t| (t, Token![,]));
        t0 << ClosureArg {
            attrs,
            pat,
            ty,
            comma,
        }
    }
    fn parse_closure_args(&mut self) -> L<List<ClosureArg>> {
        if self.check_punct(Punct::Or) {
            return Trivia::default() << List::default();
        }
        let L(t0, arg) = self.parse_closure_arg();
        let mut has_comma = arg.comma.is_some();
        let mut list = List::single(arg);
        loop {
            if !has_comma || self.check_punct(Punct::Or) {
                let tlast = self.eat_punct(Punct::Or).unwrap();
                list.push_trivia(tlast);
                break t0 << list;
            }
            let L(t, arg) = self.parse_closure_arg();
            has_comma = arg.comma.is_some();
            list.push(t, arg);
        }
    }
    fn parse_closure(&mut self, allow_struct: bool) -> Option<L<Closure>> {
        let t0 = self.eat_punct(Punct::Or)?;
        let L(t1, args) = self.parse_closure_args();
        let ret = self.parse_fn_ret();
        let L(t2, body) = self.parse_expr_inner(allow_struct).map(Box::new);
        Some(
            t0 << Closure {
                bar1: Token![|],
                t1,
                args,
                bar2: Token![|],
                ret,
                t2,
                body,
            },
        )
    }
    fn parse_arm(&mut self) -> L<Arm> {
        let attrs = self.parse_attrs(AttrKind::Outer);
        let L(t1, pat) = self.parse_multi_pat_with_leading_vert();
        let (t0, attrs) = if let Some((t, mut l)) = attrs {
            l.push_trivia(t1);
            (t, l)
        } else {
            (t1, List::default())
        };
        let guard = if let Some(tbeforeif) = self.eat_kw("if") {
            let L(t2, e) = self.parse_expr().map(Box::new);
            Some((tbeforeif, Token![if], t2, e))
        } else { None };
        let t1 = self.eat_punct(Punct::RFatArrow).unwrap();
        let L(t2, body) = self.parse_expr_inner(false).map(Box::new);
        let comma = self.eat_punct(Punct::Comma).map(|t| (t, Token![,]));
        t0 << Arm {
            attrs, pat, guard, t1, arrow: Token![=>], t2, body, comma
        }
    }
    fn parse_arms(&mut self) -> L<Braces<(Trivia, List<Arm>)>> {
        self.eat_delim(Delimiter::Braces, |t0, mut this| {
            if let Some(eof) = this.eat_eof() {
                return t0 << Braces((eof, List::default()));
            }
            let L(t1, arm) = this.parse_arm();
            let mut list = List::single(arm);
            loop {
                if let Some(eof) = this.eat_eof() {
                    list.push_trivia(eof);
                    break t0 << Braces((t1, list));
                }
                let L(t, arm) = this.parse_arm();
                list.push(t, arm);
            }
        }).unwrap()
    }
    fn parse_expr_match(&mut self) -> Option<L<ExprKind>> {
        let t0 = self.eat_kw("match")?;
        let L(t1, expr) = self.parse_expr_inner(false).map(Box::new);
        let L(t2, arms) = self.parse_arms();
        Some(
            t0 << ExprKind::Match(Match {
                token: Token![match],
                t1,
                expr,
                t2,
                arms,
            }),
        )
    }
    fn parse_unary_expr(&mut self, allow_struct: bool) -> L<ExprKind> {
        // TODO
        self.parse_atom_expr(allow_struct)
    }
    fn parse_atom_expr(&mut self, allow_struct: bool) -> L<ExprKind> {
        // TODO: let, range
        if let Some(L(t, l)) = self.eat_literal() {
            t << ExprKind::Literal(l)
        } else if self.peek(|x| x.is_delim(Delimiter::Parens)) {
            self.parse_paren_or_tuple()
        } else if self.check_ident("async")
            && self.peek2(|tt| {
                tt.is_delim(Delimiter::Braces)
                    || (tt.is_ident("move") && self.peek3(|tt| tt.is_delim(Delimiter::Braces)))
            })
        {
            let t = self.eat_ident("async").unwrap().0;
            let L(t1, block) = self.parse_block();
            t << ExprKind::AsyncBlock(AsyncBlock {
                token: Token![async],
                t1,
                block,
            })
        } else if let Some(e) = self
            .parse_try_block()
            .or_else(|| self.parse_const_block())
            .or_else(|| self.parse_unsafe_block())
            .or_else(|| self.parse_expr_if())
            .or_else(|| self.parse_array_or_repeat())
            .or_else(|| {
                self.parse_closure(allow_struct)
                    .map(|x| x.map(ExprKind::Closure))
            })
            .or_else(|| self.parse_expr_while())
            .or_else(|| self.parse_expr_for())
            .or_else(|| self.parse_expr_loop())
            .or_else(|| self.parse_expr_match())
        {
            e
        } else if self.peek(|tt| tt.is_delim(Delimiter::Braces)) {
            self.parse_block()
                .map(|block| LabeledBlock { label: None, block })
                .map(ExprKind::Block)
        } else if self.peek(|tt| matches!(tt, TokenTree::Lifetime(_))) {
            self.parse_labeled_atom_expr()
        } else if let Some(t) = self.eat_kw("break") {
            let label = if self.peek(|tt| matches!(tt, TokenTree::Lifetime(_))) {
                Some(self.bump().map(|t| t.into_lifetime().unwrap()))
            } else {
                None
            };
            let expr = if self.peek_expr()
                && (allow_struct || !self.peek(|t| t.is_delim(Delimiter::Braces)))
            {
                Some(self.parse_expr().map(Box::new))
            } else {
                None
            };

            t << ExprKind::Break(Break {
                token: Token![break],
                label,
                expr,
            })
        } else if let Some(t) = self.eat_kw("continue") {
            let label = if self.peek(|tt| matches!(tt, TokenTree::Lifetime(_))) {
                Some(self.bump().map(|t| t.into_lifetime().unwrap()))
            } else {
                None
            };

            t << ExprKind::Continue(Continue {
                token: Token![continue],
                label,
            })
        } else if let Some(t) = self.eat_kw("return") {
            let expr = if self.peek_expr() {
                Some(self.parse_expr().map(Box::new))
            } else {
                None
            };

            t << ExprKind::Return(Return {
                token: Token![return],
                expr,
            })
        } else if let Some(t) = self.eat_kw("yield") {
            let expr = if self.peek_expr() {
                Some(self.parse_expr().map(Box::new))
            } else {
                None
            };

            t << ExprKind::Yield(Yield {
                token: Token![yield],
                expr,
            })
        } else if let Some(t) = self.eat_kw("become") {
            let L(t1, expr) = self.parse_expr().map(Box::new);
            t << ExprKind::Become(Become {
                token: Token![become],
                t1,
                expr,
            })
        } else {
            let L(t, qpath) = self.parse_qpath();
            t << self.parse_rest_of_path_or_macro_or_struct(qpath, allow_struct)
        }
    }

    fn parse_paren_or_tuple(&mut self) -> L<ExprKind> {
        self.eat_delim(Delimiter::Parens, |t0, mut this| {
            if let Some(tlast) = this.eat_eof() {
                let mut list = SeparatedList::new();
                list.push_trivia(tlast);
                return t0
                    << ExprKind::Tuple(Parens(TupleOrArrayContents {
                        t1: Trivia::default(),
                        contents: list,
                    }));
            }
            let L(t1, first) = this.parse_expr();
            if let Some(t2) = this.eat_eof() {
                return t0
                    << ExprKind::Paren(Parens(ExprParen {
                        t1,
                        expr: Box::new(first),
                        t2,
                    }));
            }
            let mut elems = SeparatedList::new_single(first);
            let tlast = loop {
                let t = this.eat_punct(Punct::Comma).unwrap();
                elems.push_sep(t, Token![,]);
                if let Some(tlast) = this.eat_eof() {
                    break tlast;
                }
                let L(t, expr) = this.parse_expr();
                elems.push_value(t, expr);
                if let Some(tlast) = this.eat_eof() {
                    break tlast;
                }
            };
            elems.push_trivia(tlast);

            t0 << ExprKind::Tuple(Parens(TupleOrArrayContents {
                t1,
                contents: elems,
            }))
        })
        .unwrap()
    }

    fn parse_field_value(&mut self) -> L<ExprStructField> {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let L(t, ident) = self.parse_ident();
        attrs.push_trivia(t);
        let expr = if self.check_punct(Punct::Colon)
            || ident.0.as_bytes().iter().all(|i| i.is_ascii_digit())
        {
            let colon = self.eat_punct(Punct::Colon).unwrap();
            let value = self.parse_expr().map(Box::new);
            Some((colon << Token![:], value))
        } else {
            None
        };
        t0 << ExprStructField { attrs, ident, expr }
    }

    fn parse_rest_of_struct(&mut self, qpath: QPath) -> ExprStruct {
        self.eat_delim(Delimiter::Braces, |t0, mut this| {
            let mut builder = SeparatedListBuilder::new();
            let tlast = loop {
                if let Some(tlast) = this.eat_eof() {
                    break tlast;
                };
                if this.check_punct(Punct::DotDot) {
                    let L(t1, list) = builder.build();
                    let dot2 = this.bump().map(|_| Token![..]);
                    let rest = (!this.check_eof()).then(|| this.parse_expr().map(Box::new));
                    let tlast = this.eat_eof().unwrap();
                    return ExprStruct {
                        qpath,
                        t0,
                        fields: Braces(ExprStructFields {
                            t1,
                            list,
                            dot2: Some(dot2),
                            rest,
                            tlast,
                        }),
                    };
                }
                let L(t, field) = this.parse_field_value();
                builder.push_value(t, field);
                if let Some(tlast) = this.eat_eof() {
                    break tlast;
                };
                let t = this.eat_punct(Punct::Comma).unwrap();
                builder.push_sep(t, Token![,]);
            };

            let L(t1, list) = builder.build();

            ExprStruct {
                qpath,
                t0,
                fields: Braces(ExprStructFields {
                    t1,
                    list,
                    dot2: None,
                    rest: None,
                    tlast,
                }),
            }
        })
        .unwrap()
    }

    fn parse_rest_of_path_or_macro_or_struct(
        &mut self,
        qpath: QPath,
        allow_struct: bool,
    ) -> ExprKind {
        if qpath.qself.is_none() && self.check_punct(Punct::Bang) && qpath.path.has_no_args() {
            let t1 = self.eat_punct(Punct::Bang).unwrap();
            let L(t2, inner) = self.eat_delimited().unwrap();
            ExprKind::Macro(MacroCall {
                path: qpath.path,
                t1,
                bang: Token![!],
                t2,
                inner,
            })
        } else if allow_struct && self.peek(|tt| tt.is_delim(Delimiter::Braces)) {
            ExprKind::Struct(self.parse_rest_of_struct(qpath))
        } else {
            ExprKind::QPath(qpath)
        }
    }

    fn parse_label(&mut self) -> L<(Ident, Trivia, Token![:])> {
        let lbl = self.bump().map(|x| x.into_lifetime().unwrap());
        let t = self.eat_punct(Punct::Colon).unwrap();
        lbl.map(|i| (i, t, Token![:]))
    }

    fn parse_labeled_atom_expr(&mut self) -> L<ExprKind> {
        let L(t0, (lt, t1, colon)) = self.parse_label();
        let L(t2, mut e) = self
            .parse_expr_while()
            .or_else(|| self.parse_expr_for())
            .or_else(|| self.parse_expr_loop())
            .unwrap_or_else(|| {
                self.parse_block()
                    .map(|block| LabeledBlock { label: None, block })
                    .map(ExprKind::Block)
            });

        match &mut e {
            ExprKind::Block(LabeledBlock { label, block: _ })
            | ExprKind::Loop(Loop { label, .. })
            | ExprKind::While(While { label, .. })
            | ExprKind::For(For { label, .. }) => {
                *label = Some(Label { lt, t1, colon, t2 });
            }
            _ => unreachable!(),
        }
        t0 << e
    }

    fn parse_expr_if_inner(&mut self) -> Option<L<IfExpr>> {
        let t0 = self.eat_kw("if")?;
        let L(t1, cond) = self.parse_expr_inner(false);
        let L(t2, then) = self.parse_block();
        let else_ = if let Some(t3) = self.eat_kw("else") {
            let L(t4, kind) = self
                .parse_expr_if_inner()
                .map(|x| x.map(Box::new).map(ElseKind::ElseIf))
                .unwrap_or_else(|| self.parse_block().map(ElseKind::Else));
            Some(Else {
                t3,
                token: Token![else],
                t4,
                kind,
            })
        } else {
            None
        };
        Some(
            t0 << IfExpr {
                token: Token![if],
                t1,
                cond: Box::new(cond),
                t2,
                then,
                else_,
            },
        )
    }

    fn parse_expr_if(&mut self) -> Option<L<ExprKind>> {
        self.parse_expr_if_inner().map(|x| x.map(ExprKind::If))
    }

    fn parse_expr_loop(&mut self) -> Option<L<ExprKind>> {
        let t0 = self.eat_kw("loop")?;
        let L(t1, block) = self.parse_block();
        Some(
            t0 << ExprKind::Loop(Loop {
                label: None,
                token: Token![loop],
                t1,
                block,
            }),
        )
    }

    fn parse_expr_for(&mut self) -> Option<L<ExprKind>> {
        let t0 = self.eat_kw("for")?;
        let L(t1, pat) = self.parse_pat();
        let t2 = self.eat_kw("in").unwrap();
        let L(t3, expr) = self.parse_expr_inner(false);
        let L(t4, block) = self.parse_block();
        Some(
            t0 << ExprKind::For(For {
                label: None,
                token: Token![for],
                t1,
                pat,
                t2,
                in_: Token![in],
                t3,
                expr: Box::new(expr),
                t4,
                block,
            }),
        )
    }

    fn parse_expr_while(&mut self) -> Option<L<ExprKind>> {
        let t0 = self.eat_kw("while")?;
        let L(t1, cond) = self.parse_expr_inner(false);
        let L(t2, then) = self.parse_block();
        Some(
            t0 << ExprKind::While(While {
                label: None,
                token: Token![while],
                t1,
                cond: Box::new(cond),
                t2,
                then,
            }),
        )
    }
}
