use super::TableMetadata;
use serde_json::Value;
use thiserror::Error;

static TABLE_FORMAT_VERSION: u32 = 1;

static FORMAT_VERSION: &'static str = "format-version";

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Missing format version")]
    MissingFormatVersion,
    #[error("Unsupported format version: {0}")]
    UnsupportedFormatVersion(u32),
}

pub fn from_json(json: &str) -> Result<TableMetadata, ParserError> {
    let v: Value = serde_json::from_str(json)?;
    let format_version = get_format_version(&v)?;

    if format_version != TABLE_FORMAT_VERSION {
        return Err(ParserError::UnsupportedFormatVersion(format_version));
    }

    Ok(TableMetadata {})
}

fn get_format_version(value: &Value) -> Result<u32, ParserError> {
    match value.get(FORMAT_VERSION) {
        Some(Value::Number(n)) => Ok(n.as_u64().unwrap() as u32),
        _ => Err(ParserError::MissingFormatVersion),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let json = r#"{
            "format-version": 1
        }"#;

        let result = from_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_json_invalid_json() {
        let json = r#"{
            "format-version": 1
        "#;

        let result = from_json(json);
        assert!(result.is_err());
        assert!(matches!(result, Err(ParserError::InvalidJson(_))));
    }

    #[test]
    fn test_from_json_missing_format_version() {
        let json = r#"{
            "foo": "bar"
        }"#;

        let result = from_json(json);
        assert!(result.is_err());
        assert!(matches!(result, Err(ParserError::MissingFormatVersion)));
    }

    #[test]
    fn test_from_json_unsupported_format_version() {
        let json = r#"{
            "format-version": 2
        }"#;

        let result = from_json(json);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ParserError::UnsupportedFormatVersion(2))
        ));
    }
}
