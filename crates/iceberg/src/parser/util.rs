use serde_json::Value;

use super::ParserError;

pub fn _get_string(value: &Value, field: &str, path: Option<&str>) -> Result<String, ParserError> {
    match value.get(field) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(ParserError::MissingRequiredField(
            path.unwrap_or(field).to_owned(),
        )),
    }
}

pub fn _get_u32(value: &Value, field: &str, path: Option<&str>) -> Result<u32, ParserError> {
    match value.get(field) {
        Some(Value::Number(n)) => Ok(n.as_u64().unwrap() as u32),
        _ => Err(ParserError::MissingRequiredField(
            path.unwrap_or(field).to_owned(),
        )),
    }
}

pub fn _get_u64(value: &Value, field: &str, path: Option<&str>) -> Result<u64, ParserError> {
    match value.get(field) {
        Some(Value::Number(n)) => Ok(n.as_u64().unwrap()),
        _ => Err(ParserError::MissingRequiredField(
            path.unwrap_or(field).to_owned(),
        )),
    }
}

pub fn _get_bool(value: &Value, field: &str, path: Option<&str>) -> Result<bool, ParserError> {
    match value.get(field) {
        Some(Value::Bool(b)) => Ok(b.to_owned()),
        _ => Err(ParserError::MissingRequiredField(
            path.unwrap_or(field).to_owned(),
        )),
    }
}

pub fn _get_string_array(
    value: &Value,
    field: &str,
    path: Option<&str>,
) -> Result<Vec<String>, ParserError> {
    match value.get(field) {
        Some(Value::Array(arr)) => Ok(arr.iter().map(|v| v.as_str().unwrap().to_owned()).collect()),
        _ => Err(ParserError::MissingRequiredField(
            path.unwrap_or(field).to_owned(),
        )),
    }
}

macro_rules! generate_getter_macro {
    ($name:ident, $func_name:ident) => {
        macro_rules! $name {
            ($value:expr, $field:expr) => {
                crate::parser::util::$func_name($value, $field, None)
            };
            ($value:expr, $field:expr, $path:expr) => {
                crate::parser::util::$func_name($value, $field, Some($path))
            };
        }

        pub(crate) use $name;
    };
}

generate_getter_macro!(get_string, _get_string);
generate_getter_macro!(get_u32, _get_u32);
generate_getter_macro!(get_u64, _get_u64);
generate_getter_macro!(get_bool, _get_bool);
generate_getter_macro!(get_string_array, _get_string_array);
