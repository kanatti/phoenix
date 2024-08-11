use super::Transform;

pub struct PartitionField<T, S> {
    pub source_id: u32,
    pub name: String,
    pub transform: Box<dyn Transform<T, S>>,
}

impl<T, S> PartitionField<T, S> {
    pub fn new(source_id: u32, name: String, transform: Box<dyn Transform<T, S>>) -> Self {
        Self {
            source_id,
            name,
            transform,
        }
    }
}
