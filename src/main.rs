mod ast;
mod lex;
pub(crate) use lex::Lexer;
mod token;
pub(crate) use token::*;
mod parse;
mod print;

use crate::{parse::parse, print::Print};

fn main() {
    let src = " /* w */ mod foo {
        mod barrr ; // a
    }";
    let f = parse(src);
    println!("{f:#?}");
    let mut s = String::new();
    f.print(src, &mut s);
    assert_eq!(src, s);
}
