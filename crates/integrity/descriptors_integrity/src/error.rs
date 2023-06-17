use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DescriptorsIntegrityError {
    #[error("{0:?}, field is missing")]
    EmptyField(String),
    // #[error("Element missing its Entry")]
    // ValidationError,

    // #[error("Element missing its Entry")]
    // EntryMissing,

    // #[error("Wasm Error {0}")]
    // Wasm(WasmError),
}

impl From<DescriptorsIntegrityError> for ValidateCallbackResult {
    fn from(e: DescriptorsIntegrityError) -> Self {
        ValidateCallbackResult::Invalid(e.to_string())
    }
}

impl From<DescriptorsIntegrityError> for ExternResult<ValidateCallbackResult> {
    fn from(e: DescriptorsIntegrityError) -> Self {
        Ok(e.into())
    }
}
