use crate::parse::attr::AttrKind;
use crate::prelude::*;

impl<'src> super::Parser<'src> {
    pub fn parse_expr(&mut self) -> L<Expr> {
        self.parse_expr_inner(true)
    }
    fn peek_expr(&self) -> bool {
        self.peek(|tt| matches!(tt, TokenTree::Ident(i) if i.0 != "as"))
        || self.peek(|tt| matches!(tt, TokenTree::Group(_) | TokenTree::Literal(_)))
        || self.check_punct(Punct::Bang)
        || self.check_punct(Punct::Minus)
        || self.check_punct(Punct::Star)
        || self.check_punct(Punct::Or)
        || self.check_punct(Punct::And)
        || self.check_punct(Punct::DotDot)
    }
    fn parse_expr_inner(&mut self, allow_struct: bool) -> L<Expr> {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let L(t1, kind) = self.parse_atom_expr();
        attrs.push_trivia(t1);
        t0 << Expr { attrs, kind }
    }
    fn parse_atom_expr(&mut self) -> L<ExprKind> {
        if let Some(L(t, l)) = self.eat_literal() {
            t << ExprKind::Literal(l)
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
        } else if self.check_ident("try") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("try").unwrap().0;
            let L(t1, block) = self.parse_block();
            t << ExprKind::TryBlock(TryBlock {
                token: Token![try],
                t1,
                block,
            })
        } else if self.check_ident("const") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("const").unwrap().0;
            let L(t1, block) = self.parse_block();
            t << ExprKind::Const(ConstBlock {
                token: Token![const],
                t1,
                block,
            })
        } else if self.check_ident("unsafe") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("unsafe").unwrap().0;
            let L(t1, block) = self.parse_block();
            t << ExprKind::Unsafe(UnsafeBlock {
                token: Token![unsafe],
                t1,
                block,
            })
        } else if self.peek(|tt| tt.is_delim(Delimiter::Braces)) {
            self.parse_block().map(|block| LabeledBlock { label: None, block }).map(ExprKind::Block)
        } else if let Some(t) = self.eat_kw("if") {
            t << ExprKind::If(self.parse_expr_if())
        } else if self.check_ident("while") {
            self.parse_expr_while().map(ExprKind::While)
        } else if self.check_ident("for") {
            self.parse_expr_for().map(ExprKind::For)
        } else if self.check_ident("loop") {
            self.parse_expr_loop().map(ExprKind::Loop)
        } else if self.peek(|tt| matches!(tt, TokenTree::Lifetime(_))) {
            self.parse_labeled_atom_expr()
        } else {
            panic!("not an expr anymore: {:?}", self.token)
        }
    }

    fn parse_label(&mut self) -> L<(Ident, Trivia, Token![:])> {
        let lbl = self.bump().map(|x| x.into_lifetime().unwrap());
        let t = self.eat_punct(Punct::Colon).unwrap();
        lbl.map(|i| (i, t, Token![:]))
    }

    fn parse_labeled_atom_expr(&mut self) -> L<ExprKind> {
        let L(t0, (lt, t1, colon)) = self.parse_label();
        let L(t2, mut e) = if self.check_ident("while") {
            self.parse_expr_while().map(ExprKind::While)
        } else if self.check_ident("for") {
            self.parse_expr_for().map(ExprKind::For)
        } else if self.check_ident("loop") {
            self.parse_expr_loop().map(ExprKind::Loop)
        } else {
            self.parse_block().map(|block| LabeledBlock { label: None, block }).map(ExprKind::Block)
        };

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

    fn parse_expr_if(&mut self) -> IfExpr {
        let L(t1, cond) = self.parse_expr_inner(false);
        let L(t2, then) = self.parse_block();
        let else_ = if let Some(t3) = self.eat_kw("else") {
            let L(t4, kind) = if let Some(t4) = self.eat_kw("if") {
                let if_ = self.parse_expr_if();
                t4 << ElseKind::ElseIf(Box::new(if_))
            } else {
                self.parse_block().map(ElseKind::Else)
            };
            Some(Else { t3, token: Token![else], t4, kind })
        } else {
            None
        };
        IfExpr {
            token: Token![if],
            t1,
            cond: Box::new(cond),
            t2,
            then,
            else_,
        }
    }

    fn parse_expr_loop(&mut self) -> L<Loop> {
        let t0 = self.eat_kw("loop").unwrap();
        let L(t1, block) = self.parse_block();
        t0 << Loop { label: None, token: Token![loop], t1, block }
    }

    fn parse_expr_for(&mut self) -> L<For> {
        let t0 = self.eat_kw("for").unwrap();
        let L(t1, pat) = self.parse_pat();
        let t2 = self.eat_kw("in").unwrap();
        let L(t3, expr) = self.parse_expr_inner(false);
        let L(t4, block) = self.parse_block();
        t0 << For { label: None, token: Token![for], t1, pat, t2, in_: Token![in], t3, expr: Box::new(expr), t4, block }
    }

    fn parse_expr_while(&mut self) -> L<While> {
        let t0 = self.eat_kw("while").unwrap();
        let L(t1, cond) = self.parse_expr_inner(false);
        let L(t2, then) = self.parse_block();
        t0 << While { label: None, token: Token![while], t1, cond: Box::new(cond), t2, then }
    }
}
