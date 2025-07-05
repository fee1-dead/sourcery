mod ast;
mod lex;
pub(crate) use lex::Lexer;
mod token;
mod parse;
pub(crate) use token::*;

use crate::parse::parse;

fn main() {
    println!("{:#?}", parse(" /* w */ mod foo {
        mod barrr ; // a
    }"));
}
