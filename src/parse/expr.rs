use crate::parse::attr::AttrKind;
use crate::prelude::*;

impl<'src> super::Parser<'src> {
    pub fn parse_expr(&mut self) -> L<Expr> {
        self.parse_expr_inner(true)
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
        } else if self.peek(|tt| tt.is_delim(Delimiter::Braces)) {
            self.parse_block().map(ExprKind::Block)
        } else if let Some(t) = self.eat_kw("if") {
            t << ExprKind::If(self.parse_expr_if())
        } else {
            panic!("not an expr anymore: {:?}", self.token)
        }
    }

    fn parse_expr_if(&mut self) -> IfExpr {
        let L(t1, cond) = self.parse_expr_inner(false);
        let L(t2, then) = self.parse_block();
        IfExpr {
            token: Token![if],
            t1,
            cond: Box::new(cond),
            t2,
            then,
            else_: None,
        }
    }
}
