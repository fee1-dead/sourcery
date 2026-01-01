use crate::prelude::*;

#[derive(Debug, Print, Walk, Respace)]
pub struct PathSegment {
    pub ident: Ident,
}

pub enum PathSegmentArguments {
    AngleBracketed {
        colon2: Option<L<Token![::]>>,
        t1: Trivia,
        lt: Token![<],

    }
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

#[derive(Debug, Print, Walk)]
pub struct QSelf {
    pub left: Token![<],
    pub t1: Trivia,
    pub ty: Box<Ty>,
    pub as_: Option<(Trivia, Token![as], Trivia, Path)>,
    pub tlast: Trivia,
    pub right: Token![>],
}

#[derive(Debug, Print, Walk)]
pub struct QPath {
    pub qself: Option<(QSelf, Trivia)>,
    pub path: Path,
}

impl Respace for QPath {
    fn respace(&mut self, v: &mut Spaces) {
        if self.qself.is_some() {
            todo!()
        }
        self.path.respace(v);
    }
}


