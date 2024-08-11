use serde_json::Value;

use crate::{
    parser::util,
    schema::{NestedField, Schema},
};

use super::ParserError;

static TYPE: &'static str = "type";
static FIELDS: &'static str = "fields";
static NAME: &'static str = "name";
static ID: &'static str = "id";
static REQUIRED: &'static str = "required";


pub fn from_json(json: &str) -> Result<Schema, ParserError> {
    let value: Value = serde_json::from_str(json)?;
    from_json_value(&value)
}

pub fn from_json_value(value: &Value) -> Result<Schema, ParserError> {
    let fields = value
        .get(FIELDS)
        .ok_or_else(|| ParserError::MissingRequiredField("schema.fields".to_owned()))?;

    if !fields.is_array() {
        return Err(ParserError::InvalidFieldType(
            "schema.fields".to_owned(),
            "array".to_owned(),
        ));
    }

    let fields = fields.as_array().unwrap();
    let mut nested_fields: Vec<NestedField> = Vec::with_capacity(fields.len());

    for field in fields {
        nested_fields.push(as_field(field)?);
    }   


    Ok(Schema::new(nested_fields))
}

fn as_field(value: &Value) -> Result<NestedField, ParserError> {
    let name = util::get_string!(value, NAME, "schema.fields.name")?;
    let field_type = util::get_string!(value, TYPE, "schema.fields.type")?;
    let id = util::get_u32!(value, ID, "schema.fields.id")?;
    let required = util::get_bool!(value, REQUIRED, "schema.fields.required")?;

    Ok(NestedField {
        id,
        name,
        field_type,
        required
    })
}
