use thiserror::Error;

use crate::meta_models::{CharacterNumber, LineNumber};

#[derive(Error, Debug)]
pub enum Pass1Error {
    #[error("")]
    ParseError(LineNumber, CharacterNumber),
}

#[derive(Error, Debug)]
pub enum Pass2Error {
    #[error("")]
    ParseError(LineNumber, CharacterNumber),
}