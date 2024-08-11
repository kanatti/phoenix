use std::collections::HashMap;

use serde_json::{value, Value};

use crate::{metadata::TableMetadata, partition::PartitionSpec, schema::Schema};

use super::{partition_spec, schema, util, ParserError};

static TABLE_FORMAT_VERSION: u32 = 1;

static FORMAT_VERSION: &'static str = "format-version";
static LOCATION: &'static str = "location";
static LAST_UPDATED_MILLIS: &'static str = "last-updated-ms";
static LAST_COLUMN_ID: &'static str = "last-column-id";
static SCHEMA: &'static str = "schema";
static PARTITION_SPEC: &'static str = "partition-spec";
static PROPERTIES: &'static str = "properties";
static CURRENT_SNAPSHOT_ID: &'static str = "current-snapshot-id";
static SNAPSHOTS: &'static str = "snapshots";

pub fn from_json(json: &str) -> Result<TableMetadata, ParserError> {
    let value: Value = serde_json::from_str(json)?;
    from_json_value(&value)
}

pub fn from_json_value(value: &Value) -> Result<TableMetadata, ParserError> {
    let format_version = util::get_u32!(&value, FORMAT_VERSION)?;

    if format_version != TABLE_FORMAT_VERSION {
        return Err(ParserError::UnsupportedFormatVersion(format_version));
    }

    let location = util::get_string!(&value, LOCATION)?;
    let last_column_id = util::get_u32!(&value, LAST_COLUMN_ID)?;
    // Parse PROPERTIES
    let current_snapshot_id = util::get_u32!(&value, CURRENT_SNAPSHOT_ID)?;
    let last_updated_millis = util::get_u64!(&value, LAST_UPDATED_MILLIS)?;
    // Parse SNAPSHOTS

    Ok(TableMetadata {
        location,
        last_column_id,
        current_snapshot_id,
        last_updated_millis,
        schema: get_schema(value)?,
        partition_spec: get_partition_spec(value)?,
        properties: get_properties(value)?,
    })
}

fn get_schema(value: &Value) -> Result<Schema, ParserError> {
    let value = value
        .get(SCHEMA)
        .ok_or_else(|| ParserError::MissingRequiredField(SCHEMA.to_owned()))?;

    schema::from_json_value(&value)
}

fn get_partition_spec(value: &Value) -> Result<PartitionSpec, ParserError> {
    let value = value
        .get(PARTITION_SPEC)
        .ok_or_else(|| ParserError::MissingRequiredField(PARTITION_SPEC.to_owned()))?;

    partition_spec::from_json_value(&value)
}

fn get_properties(value: &Value) -> Result<HashMap<String, String>, ParserError> {
    let value = value
        .get(PROPERTIES)
        .ok_or_else(|| ParserError::MissingRequiredField(PROPERTIES.to_owned()))?;

    let value = value.as_object().unwrap();

    let mut properties = HashMap::with_capacity(value.len());

    for (property, property_value) in value.iter() {
        let property_value = property_value
            .as_str()
            .ok_or_else(|| ParserError::InvalidFieldType(property.clone()))?;
        properties.insert(property.clone(), property_value.to_owned());
    }

    Ok(properties)
}

#[cfg(test)]
mod tests {
    use crate::schema::NestedField;

    use super::*;

    #[test]
    fn test_from_json() {
        let json = r#"{
            "format-version": 1,
            "location": "s3://test-location/metadata.json",
            "last-column-id": 100,
            "last-updated-ms": 1723320520000,
            "current-snapshot-id": 1,
            "schema": {
                "fields": [
                    {"id": 1, "name": "id", "type": "integer", "required": false},
                    {"id": 2, "name": "name", "type": "string", "required": true},
                    {"id": 3, "name": "age", "type": "integer", "required": true}
                ]
            },
            "partition-spec": [
                {
                    "source-id": 1,
                    "transform": "bucket",
                    "name": "id_bucket"
                }
            ],
            "properties": {
                "property1": "value1",
                "property2": "value2"
            }
        }"#;

        let result = from_json(json);
        assert!(result.is_ok(), "Result is not Ok, Error - {:?}", result);

        let metadata = result.unwrap();
        assert_eq!(metadata.location, "s3://test-location/metadata.json");
        assert_eq!(metadata.last_column_id, 100);
        assert_eq!(metadata.current_snapshot_id, 1);
        assert_eq!(metadata.last_updated_millis, 1723320520000);

        assert_eq!(metadata.schema.fields.len(), 3);

        assert_eq!(
            metadata.schema.fields[0],
            NestedField {
                id: 1,
                name: "id".to_string(),
                field_type: "integer".to_string(),
                required: false
            }
        );
        assert_eq!(
            metadata.schema.fields[1],
            NestedField {
                id: 2,
                name: "name".to_string(),
                field_type: "string".to_string(),
                required: true
            }
        );
        assert_eq!(
            metadata.schema.fields[2],
            NestedField {
                id: 3,
                name: "age".to_string(),
                field_type: "integer".to_string(),
                required: true
            }
        );
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
            "location": "s3://test-location/metadata.json",
            "last-column-id": 100,
            "last-updated-ms": 1723320520000,
            "current-snapshot-id": 1
        }"#;

        let result = from_json(json);
        assert!(result.is_err());

        if let Err(ParserError::MissingRequiredField(field)) = result {
            assert_eq!(field, FORMAT_VERSION);
        }
    }

    #[test]
    fn test_from_json_unsupported_format_version() {
        let json = r#"{
            "format-version": 2,
            "location": "s3://test-location/metadata.json",
            "last-column-id": 100,
            "last-updated-ms": 1723320520000,
            "current-snapshot-id": 1
        }"#;

        let result = from_json(json);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ParserError::UnsupportedFormatVersion(2))
        ));
    }
}
