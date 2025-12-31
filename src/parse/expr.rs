use crate::prelude::*;
use crate::parse::attr::AttrKind;

impl<'src> super::Parser<'src> {
    pub fn parse_expr(&mut self) -> (Trivia, Expr) {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let (t1, kind) = self.parse_atom_expr();
        attrs.push_trivia(t1);
        (t0, Expr { attrs, kind })
    }
    fn parse_atom_expr(&mut self) -> (Trivia, ExprKind) {
        if let Some((t, l)) = self.eat_literal() {
            (t, ExprKind::Literal(l))
        } else if self.check_ident("async") && self.peek2(|tt| tt.is_delim(Delimiter::Braces) || (tt.is_ident("move") && self.peek3(|tt| tt.is_delim(Delimiter::Braces)))) {
            let t = self.eat_ident("async").unwrap().0;
            let (t1, block) = self.parse_block();
            (t, ExprKind::AsyncBlock(AsyncBlock { token: Token![async], t1, block }))
        } else if self.check_ident("try") && self.peek2(|tt| tt.is_delim(Delimiter::Braces)) {
            let t = self.eat_ident("try").unwrap().0;
            let (t1, block) = self.parse_block();
            (t, ExprKind::TryBlock(TryBlock { token: Token![try], t1, block }))
        } else if self.peek(|tt| tt.is_delim(Delimiter::Braces)) {
            let (t, block) = self.parse_block();
            (t, ExprKind::Block(block))
        } else {
            panic!("not an expr anymore: {:?}", self.token)
        }
    }
}
