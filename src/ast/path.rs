use crate::prelude::*;

#[derive(Debug, Print, Walk, Respace)]
pub struct PathSegment {
    pub ident: Ident,
}

#[derive(Debug, Print)]
pub struct Path {
    pub leading_colon: Option<(Token![::], Trivia)>,
    pub seg1: PathSegment,
    // Not `List` because this one is self-contained (no trailing trivia)
    pub rest: Vec<(Trivia, Token![::], Trivia, PathSegment)>,
}

impl Respace for Path {
    fn respace(&mut self, v: &mut crate::passes::style::spaces::Spaces) {
        let Path { leading_colon, seg1, rest } = self;
        if let Some((_, t)) = leading_colon {
            s0(t);
        }
        seg1.respace(v);
        for (t, _, tt, s) in rest {
            s0(t);
            s0(tt);
            s.respace(v);
        }
    }
}

pub struct QSelf {
    pub left: Token![<],
    pub t1: Trivia,
    pub ty: Ty,
    pub as_: Option<(Trivia, Token![as], Path)>,
    pub tlast: Trivia,
    pub right: Token![>],
}
