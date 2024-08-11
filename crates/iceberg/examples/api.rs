use std::fs;

use iceberg::parser;

fn main() {
    let examples_path = std::env::current_dir().unwrap().join("examples");
    let metadata = fs::read_to_string(examples_path.join("metadata.json")).unwrap();
    println!("{}", metadata);
    let table_metadata = parser::metadata::from_json(&metadata);
    match table_metadata {
        Ok(table_metadata) => {
            println!("{:#?}", table_metadata);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
