use crate::prelude::*;

impl<'src> super::Parser<'src> {
    pub fn parse_ty(&mut self) -> L<Ty> {
        if let Some(ty) = self.eat_delim(Delimiter::Brackets, |t0, mut this| {
            let L(t1, ty) = this.parse_ty();
            let kind = if let Some(t2) = this.eat_punct(Punct::Semi) {
                let L(t3, len) = this.parse_expr();
                let tend = this.eat_eof().unwrap();
                let arrayty = TyArray { t1, elem: Box::new(ty), t2, semi: Token!(;), t3, len, t4: tend };
                Ty::Array(Brackets(arrayty))
            } else {
                let tlast = this.eat_eof().unwrap();
                Ty::Slice(Brackets(TySlice { t1, ty: Box::new(ty), tlast }))
            };
            t0 << kind
        }) {
            ty
        } else {
            self.parse_qpath().map(Ty::Path)
        }
    }
}
