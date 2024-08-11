use serde_json::Value;

use crate::{
    parser::util,
    partition::{transform, PartitionField, PartitionSpec},
};

use super::ParserError;

static SOURCE_ID: &'static str = "source-id";
static TRANSFORM: &'static str = "transform";
static NAME: &'static str = "name";

pub fn from_json(json: &str) -> Result<PartitionSpec, ParserError> {
    let value: Value = serde_json::from_str(json)?;
    from_json_value(&value)
}

pub fn from_json_value(value: &Value) -> Result<PartitionSpec, ParserError> {
    if !value.is_array() {
        return Err(ParserError::InvalidFieldType(
            "partition-spec must be an array".to_string(),
        ));
    }

    let field_values = value.as_array().unwrap();
    let mut partition_fields = Vec::with_capacity(field_values.len());

    for field_value in field_values {
        let source_id = util::get_u32!(field_value, SOURCE_ID)?;
        let transform_name = util::get_string!(field_value, TRANSFORM)?;
        let name = util::get_string!(field_value, NAME)?;

        let transform = transform::get_transform(&transform_name)
            .ok_or_else(|| ParserError::InvalidPartitionTransform(transform_name))?;

        partition_fields.push(PartitionField::new(source_id, name, transform));
    }

    Ok(PartitionSpec::new(partition_fields))
}
