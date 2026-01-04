use crate::prelude::*;
use crate::parse::attr::AttrKind;

impl<'src> Parser<'src> {
    pub fn parse_stmt(&mut self) -> (Trivia, Stmt) {
        let (t0, mut attrs) = self.parse_attrs(AttrKind::Outer).unwrap_or_default();
        let kind = if let Some(trivia) = self.eat_punct(Punct::Semi) {
            attrs.push_trivia(trivia);
            StmtKind::Empty(Token![;])
        } else {
            let L(t1, expr) = self.parse_expr_with_earlier_boundary_rule();
            attrs.push_trivia(t1);
            if let Some(t2) = self.eat_punct(Punct::Semi) {
                StmtKind::Semi(expr, t2, Token![;])
            } else {
                StmtKind::Expr(expr)
            }
        };

        (t0, Stmt { attrs, kind })
    }

    pub fn parse_block(&mut self) -> L<Block> {
        self.eat_delim(Delimiter::Braces, |t0, mut this | {
            // TODO inner attributes?
            let mut stmts = List::default();
            let mut tstart = None;
            let tend = loop {
                if let Some(tend) = this.eat_eof() {
                    break tend;
                }

                let (t, x) = this.parse_stmt();
                if tstart.is_none() {
                    tstart = Some(t);
                    stmts = List::single(x)
                } else {
                    stmts.push(t, x);
                }
            };
            let tstart = tstart.unwrap_or_default();
            stmts.push_trivia(tend);
            let b = BlockInner { t0: tstart, stmts };
            t0 << Braces(b)
        }).unwrap()
    }
}
