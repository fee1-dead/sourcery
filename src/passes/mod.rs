use crate::ast::File;

mod minify;

pub trait Pass {
    fn visit_file(&mut self, file: &mut File);
}


