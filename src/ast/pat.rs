use sourcery_derive::Walk;

use crate::ast::{Ident, Print};

#[derive(Debug, Print, Walk)]
pub enum Pat {
    Ident(Ident),
}