use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum DescriptorsError {
    #[error("{0} field is missing")]
    EmptyField(String),
    // #[error("Element missing its Entry")]
    // ValidationError,

    // #[error("Element missing its Entry")]
    // EntryMissing,

    // #[error("Wasm Error {0}")]
    // Wasm(WasmError),
}

// impl From<DescriptorsError> for ValidateCallbackResult {
//     fn from(e: DescriptorsError) -> Self {
//         ValidateCallbackResult::Invalid(e.to_string())
//     }
// }

// impl From<DescriptorsError> for ExternResult<ValidateCallbackResult> {
//     fn from(e: DescriptorsError) -> Self {
//         Ok(e.into())
//     }
// }
