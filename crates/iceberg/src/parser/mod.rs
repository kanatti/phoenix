use thiserror::Error;

pub mod metadata;
pub mod schema;
pub mod partition_spec;
pub mod snapshot;
mod util;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),
    #[error("{0}")]
    InvalidFieldType(String),
    #[error("{0}")]
    InvalidPartitionTransform(String),
    #[error("Unsupported format version: {0}")]
    UnsupportedFormatVersion(u32),
}
