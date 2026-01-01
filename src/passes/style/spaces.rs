use crate::prelude::*;

pub(crate) struct Spaces;

/// Shape the passed trivia so that it "looks" like a singular space. This means block comments are always delimited by single spaces.
fn shrink_single_space(t: Trivia) -> TriviaN {
    // conservatively bail out if we have something we can't fit on a single line
    if t.iter().any(|x| match x {
        Trivium::LineComment(_) => true,
        Trivium::BlockComment(bc) => bc.contains('\n'),
        Trivium::Whitespace(_) => false,
    }) {
        return TriviaN::new(t);
    }

    let mut had_whitespace = false;
    let mut new = Trivia::default();

    for x in t {
        match x {
            Trivium::BlockComment(_) => {
                if !had_whitespace {
                    new.push(Trivium::single_space());
                }
                had_whitespace = false;
                new.push(x);
            }
            Trivium::Whitespace(_) => {
                had_whitespace = true;
                new.push(Trivium::single_space());
            }
            Trivium::LineComment(_) => unreachable!(),
        }
    }
    if new
        .last()
        .is_none_or(|x| matches!(x, Trivium::BlockComment(_)))
    {
        new.push(Trivium::single_space())
    }
    TriviaN::new(new)
}

/// Shape the passed trivia so it takes "no" space. In practice this means block comments will be separated by whitespace.
fn shrink_no_space(t: Trivia) -> Trivia {
    Trivia::trim_whitespace(shrink_single_space(t).into())
}

#[cfg(test)]
mod tests {
    use crate::Print;
    use crate::ast::Trivia;
    use crate::passes::style::spaces::{shrink_no_space, shrink_single_space};

    fn test_glue<X: Into<Trivia>>(f: fn(Trivia) -> X) -> impl Fn(&str) -> String {
        move |s| {
            let mut out = String::new();
            f(crate::parse::parse_trivia(s)).into().print(&mut out);
            out
        }
    }

    #[test]
    fn test_shrink_single_space() {
        let sss = test_glue(shrink_single_space);
        assert_eq!(" ", sss(""));
        assert_eq!(" /* w */ ", sss("/* w */"));
        assert_eq!(" ", sss("\n \n \n\n \n \n \n\n    "));
        assert_eq!(
            " /**/ /**/ ",
            sss("
         /**//**/
          
        ")
        );
    }

    #[test]
    fn test_shrink_no_space() {
        let sss = test_glue(shrink_no_space);
        assert_eq!("", sss("    \n\n \n \n \n \n"));
        assert_eq!("/* w */", sss("/* w */"));
        assert_eq!("", sss("\n \n \n\n \n \n \n\n    "));
        assert_eq!(
            "/**/ /**/",
            sss("
         /**//**/
          
        ")
        );
    }
}

fn fixup_path(x: &mut Path) {
    if let Some((_, trivia)) = &mut x.leading_colon {
        *trivia = shrink_no_space(trivia.take()).into()
    }
    for (t1, _, t2, _) in &mut x.rest {
        *t1 = shrink_no_space(t1.take()).into();
        *t2 = shrink_no_space(t2.take()).into();
    }
}

fn fixup_visibility_pair(x: &mut (Visibility, Trivia)) {
    if let Visibility::Restricted {
        pub_: _,
        t1,
        parens,
    } = &mut x.0
    {
        *t1 = shrink_no_space(t1.take());
        parens.0.t2 = shrink_no_space(parens.0.t2.take());
        if let Some((_, x)) = &mut parens.0.in_ {
            *x = shrink_single_space(x.take().into());
        }
        parens.0.t3 = shrink_no_space(parens.0.t3.take());
        fixup_path(&mut parens.0.path);
    }
    x.1 = shrink_single_space(x.1.take()).into();
}

pub trait AnyTrivia: From<TriviaN> {
    fn take_t(&mut self) -> Trivia;
    fn set(&mut self, t: TriviaN) { *self = t.into() }
}
 
impl AnyTrivia for Trivia {
    fn take_t(&mut self) -> Trivia {
        self.take()
    }
}

impl AnyTrivia for TriviaN {
    fn take_t(&mut self) -> Trivia {
        self.take().into()
    }
}

pub fn s1(x: &mut impl AnyTrivia) {
    let t = x.take_t();
    x.set(shrink_single_space(t));
}

pub fn s0(x: &mut Trivia) {
    *x = shrink_no_space(x.take());
}

pub(crate) trait Respace {
    fn respace(&mut self, v: &mut Spaces);
}

impl Respace for Literal {
    fn respace(&mut self, _: &mut Spaces) {}
}

impl Respace for Ident {
    fn respace(&mut self, _: &mut Spaces) {}
}

impl Respace for Option<L<Ident>> {
    fn respace(&mut self, _: &mut Spaces) {
        if let Some(L(t, _)) = self {
            s1(t)
        }
    }
}

impl Respace for Option<L<Box<Expr>>> {
    fn respace(&mut self, _: &mut Spaces) {
        if let Some(L(t, _)) = self {
            s1(t)
        }
    }
}


impl Respace for Option<(Visibility, Trivia)> {
    fn respace(&mut self, _: &mut Spaces) {
        self.as_mut().map(fixup_visibility_pair);
    }
}

impl<T: Respace> Respace for Option<T> {
    fn respace(&mut self, v: &mut Spaces) {
        if let Some(x) = self.as_mut() {
            x.respace(v);
        }
    }
}

impl<T: Respace> Respace for Brackets<T> {
    fn respace(&mut self, v: &mut Spaces) {
        self.0.respace(v)
    }
}

impl<T: Respace> Respace for Box<T> {
    fn respace(&mut self, v: &mut Spaces) {
        T::respace(self, v)
    }
}

impl Pass for Spaces {
    fn visit_const(&mut self, c: &mut Const) {
        c.respace(self);
    }
    fn visit_fn(&mut self, f: &mut Fn) {
        f.vis.as_mut().map(fixup_visibility_pair);
    }
}
