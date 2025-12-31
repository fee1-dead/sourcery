use crate::prelude::*;

pub mod spaces;

struct SizeEstimator(usize);

impl Pass for SizeEstimator {
    fn visit_token(&mut self, size: usize) {
        self.0 += size;
    }
    fn visit_trivia(&mut self, t: &mut Trivia) {
        // TODO write one that considers line breaks
        self.0 += t.iter().map(|x| x.snippet().len()).sum::<usize>();
    }
}


pub fn format_with_style_guide(f: &mut File) {
    spaces::Spaces.visit_file(f);
}

