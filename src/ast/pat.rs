use crate::ast::{Ident, TrivialPrint};

#[derive(Debug, TrivialPrint!)]
pub enum Pat {
    Ident(Ident),
}