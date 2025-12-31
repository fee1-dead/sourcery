use crate::prelude::*;

impl<'src> super::Parser<'src> {
    pub fn parse_pat(&mut self) -> L<Pat> {
        let L(t0, ident) = self.parse_ident();
        t0 << Pat::Ident(ident)
    }
}
