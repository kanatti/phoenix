use crate::types::{self, FieldType};

pub trait Transform<T, S> {
    fn apply(&self, input: T) -> S;
    fn can_transform(&self, field_type: FieldType) -> bool;
    fn get_result_type(&self) -> FieldType;
}

pub struct TransformFactory {}

pub struct Bucket {
    pub n: i32,
}

impl Bucket {
    pub fn new(n: i32) -> Bucket {
        Bucket { n }
    }
}

impl Transform<types::Integer, u32> for Bucket {
    fn apply(&self, input: types::Integer) -> u32 {
        return input as u32 / self.n as u32;
    }

    fn can_transform(&self, field_type: FieldType) -> bool {
        field_type == FieldType::Integer
    }

    fn get_result_type(&self) -> FieldType {
        FieldType::Integer
    }
}