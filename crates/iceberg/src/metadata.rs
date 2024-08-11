use crate::schema::Schema;

#[derive(Debug)]
pub struct TableMetadata {
    pub location: String,
    pub last_updated_millis: u64,
    pub last_column_id: u32,
    pub current_snapshot_id: u32,
    pub schema: Schema,
}
