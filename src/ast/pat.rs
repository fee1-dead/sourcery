use sourcery_derive::{Respace, Walk};

use crate::ast::{Ident, Print};

#[derive(Debug, Print, Walk, Respace)]
pub enum Pat {
    Ident(Ident),
}