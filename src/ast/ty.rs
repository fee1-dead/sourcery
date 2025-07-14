use super::Path;
use crate::TrivialPrint;

#[derive(Debug, TrivialPrint!)]
pub enum Ty {
    Path(Path),
}
