use ra_ap_rustc_lexer::TokenKind;

use crate::ast::{Expr, ExprKind, List, Trivia};

impl<'src> super::Parser<'src> {
    pub(crate) fn parse_atom_expr(&mut self) -> (Trivia, Expr) {
        if self.peek(|t| matches!(t.kind, TokenKind::Literal { .. })) {
            let (t, l) = self.parse_literal();
            // TODO
            (t, Expr { attributes: List::default(), kind: ExprKind::Literal(l) })
        } else {
            panic!("not an expr anymore")
        }
    }
}
