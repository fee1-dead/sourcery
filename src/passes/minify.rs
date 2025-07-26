use crate::ast::File;
use crate::passes::Pass;

pub struct Minify;

impl Pass for Minify {
    fn visit_file(&mut self, file: &mut File) {
        
    }
}
