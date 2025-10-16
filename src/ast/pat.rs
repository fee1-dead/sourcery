use sourcery_derive::Walk;

use crate::ast::{Ident, TrivialPrint};

#[derive(Debug, TrivialPrint!, Walk)]
pub enum Pat {
    Ident(Ident),
}