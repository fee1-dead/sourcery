use crate::ast::{Expr, ExprKind, List, Trivia};
use crate::parse::attr::AttrKind;

impl<'src> super::Parser<'src> {
    pub fn parse_expr(&mut self) -> (Trivia, Expr) {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let (t1, mut expr) = self.parse_atom_expr();
        attrs.push_trivia(t1);
        expr.attributes = attrs;
        (t0, expr)
    }
    fn parse_atom_expr(&mut self) -> (Trivia, Expr) {
        if let Some((t, l)) = self.eat_literal() {
            // TODO
            (
                t,
                Expr {
                    attributes: List::default(),
                    kind: ExprKind::Literal(l),
                },
            )
        } else {
            panic!("not an expr anymore: {:?}", self.token)
        }
    }
}
