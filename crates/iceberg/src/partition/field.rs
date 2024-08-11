use super::Transform;

#[derive(Debug)]
pub struct PartitionField {
    pub source_id: u32,
    pub name: String,
    pub transform: Box<dyn Transform>,
}

impl PartitionField {
    pub fn new(source_id: u32, name: String, transform: Box<dyn Transform>) -> Self {
        Self {
            source_id,
            name,
            transform,
        }
    }
}
