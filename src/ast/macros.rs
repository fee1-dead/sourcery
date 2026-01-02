use crate::prelude::*;

/// A macro invocation.
#[derive(Debug, Print, Walk)]
pub struct MacroCall {
    pub path: Path,
    pub t1: Trivia,
    pub bang: Token![!],
    pub t2: Trivia,
    pub inner: Delimited<TokenStream>,
}

impl Respace for MacroCall {
    fn respace(&mut self, pass: &mut Spaces) {
        let MacroCall { path, t1, bang: _, t2, inner: _ } = self;
        path.respace(pass);
        s0(t1);
        s1(t2);
    }
}
