use std::sync::Arc;

use arrow::array::{ArrayRef, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;
use serde_json::Value;

type ArrowResult<T> = arrow::error::Result<T>;

pub fn read_record_batch_from_vec(bytes: Vec<u8>) -> ArrowResult<RecordBatch> {
    let mut stream_reader = StreamReader::try_new(bytes.as_slice(), None)?;
    let record_batch = stream_reader.next().unwrap()?;

    Ok(record_batch)
}

pub fn read_record_batch_from_json(
    json_data: &[Value],
) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    if json_data.is_empty() {
        return Err("Empty JSON data".into());
    }

    // Extract field names from the first JSON object
    let fields: Vec<Field> = json_data[0]
        .as_object()
        .ok_or("First item is not a JSON object")?
        .keys()
        .map(|k| Field::new(k, DataType::Utf8, true))
        .collect();

    let schema = Schema::new(fields);

    // Convert JSON data to columns
    let mut columns: Vec<ArrayRef> = Vec::new();
    for field in schema.fields() {
        let column: Vec<Option<String>> = json_data
            .iter()
            .map(|obj| {
                obj.get(field.name())
                    .and_then(|v| v.as_str().map(String::from))
            })
            .collect();
        columns.push(Arc::new(StringArray::from(column)) as ArrayRef);
    }

    RecordBatch::try_new(Arc::new(schema), columns).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_read_record_batch_from_vec() {
        let json_data = vec![
            json!({"name": "Alice", "age": "30", "city": "New York"}),
            json!({"name": "Bob", "age": "25", "city": "San Francisco"}),
        ];

        let record_batch = read_record_batch_from_json(&json_data).unwrap();

        assert_eq!(record_batch.num_columns(), 3);
        assert_eq!(record_batch.num_rows(), 2);

        let schema = record_batch.schema();
        let mut field_names = schema
            .fields()
            .iter()
            .map(|f| f.name())
            .collect::<Vec<&String>>();

        field_names.sort();

        assert_eq!(field_names, &["age", "city", "name"]);

        let age_array = record_batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        assert_eq!(age_array.value(0), "30");
        assert_eq!(age_array.value(1), "25");

        let city_array = record_batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        assert_eq!(city_array.value(0), "New York");
        assert_eq!(city_array.value(1), "San Francisco");

        let name_array = record_batch
            .column(2)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        assert_eq!(name_array.value(0), "Alice");
        assert_eq!(name_array.value(1), "Bob");
    }
}
