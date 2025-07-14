use crate::ast::{Trivia, Ty};

impl<'src> super::Parser<'src> {
    pub fn parse_ty(&mut self) -> (Trivia, Ty) {
        let (t0, path) = self.parse_path();
        (t0, Ty::Path(path))
    }
}
