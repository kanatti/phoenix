use thiserror::Error;

pub mod metadata;
pub mod schema;
mod util;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),
    #[error("Invalid field type for {0}. Type should be {1}")]
    InvalidFieldType(String, String),
    #[error("Unsupported format version: {0}")]
    UnsupportedFormatVersion(u32),
}
