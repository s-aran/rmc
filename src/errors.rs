use thiserror::Error;

use crate::meta_models::{CharacterNumber, LineNumber};

#[derive(Error, Debug)]
pub enum Pass1Error {
    #[error("")]
    ParseError(LineNumber, CharacterNumber),
}
