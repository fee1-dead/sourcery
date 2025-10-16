use crate::ast::{Brackets, Delimiter, Token, Trivia, Ty, TyArray, TySlice};
use crate::parse::Punct;

impl<'src> super::Parser<'src> {
    pub fn parse_ty(&mut self) -> (Trivia, Ty) {
        if let Some((t0, ty)) = self.eat_delim(Delimiter::Brackets, |t0, mut this| {
            let (t1, ty) = this.parse_ty();
            let kind = if let Some(t2) = this.eat_punct(Punct::Semi) {
                let (t3, len) = this.parse_expr();
                let tend = this.eat_eof().unwrap();
                let arrayty = TyArray { t1, elem: Box::new(ty), t2, semi: Token!(;), t3, len, t4: tend };
                Ty::Array(Brackets(arrayty))
            } else {
                let tlast = this.eat_eof().unwrap();
                Ty::Slice(Brackets(TySlice { t1, ty: Box::new(ty), tlast }))
            };
            (t0, kind)
        }) {
            (t0, ty)
        } else {
            let (t0, path) = self.parse_path();
            (t0, Ty::Path(path))
        }
    }
}
