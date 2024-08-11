use serde_json::Value;

use crate::snapshot::Snapshot;

use super::{util, ParserError};

static SNAPSHOT_ID: &'static str = "snapshot-id";
static TIMESTAMP_MS: &'static str = "timestamp-ms";
static MANIFESTS: &'static str = "manifests";

pub fn from_json(json: &str) -> Result<Snapshot, ParserError> {
    let value: Value = serde_json::from_str(json)?;
    from_json_value(&value)
}

pub fn from_json_value(value: &Value) -> Result<Snapshot, ParserError> {
    if !value.is_object() {
        return Err(ParserError::InvalidFieldType(
            "snapshot must be an object".to_string(),
        ));
    }

    let snapshot_id = util::get_u64!(value, SNAPSHOT_ID)?;
    let timestamp_ms = util::get_u64!(value, TIMESTAMP_MS)?;
    let manifests = util::get_string_array!(value, MANIFESTS)?;

    return Ok(Snapshot::new(snapshot_id, timestamp_ms, manifests));
}