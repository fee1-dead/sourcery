use crate::ast::{Pat, Trivia};

impl<'src> super::Parser<'src> {
    pub fn parse_pat(&mut self) -> (Trivia, Pat) {
        let (t0, ident) = self.parse_ident();
        (t0, Pat::Ident(ident))
    }
}
