use crate::ast::{Trivia, TriviaN};
use crate::passes::Pass;

pub struct Minify;

impl Pass for Minify {
    fn visit_trivia(&mut self, t: &mut Trivia) {
        *t = Trivia::default();
    }
    fn visit_trivia_n(&mut self, t: &mut TriviaN) {
        *t = TriviaN::single_space();
    }
}
