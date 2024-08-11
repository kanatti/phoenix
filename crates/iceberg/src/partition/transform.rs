use core::fmt;

use crate::types::FieldType;

pub trait TransformInput {
    fn as_integer(&self) -> i32;
    fn as_float(&self) -> f32;
    fn as_string(&self) -> String;
    fn as_boolean(&self) -> bool;
}

pub trait TransformOutput {
    fn as_integer(&self) -> i32;
    fn as_float(&self) -> f32;
    fn as_string(&self) -> String;
    fn as_boolean(&self) -> bool;
}

impl TransformOutput for i32 {
    fn as_integer(&self) -> i32 {
        *self as i32
    }

    fn as_float(&self) -> f32 {
        *self as f32
    }

    fn as_string(&self) -> String {
        format!("{}", *self)
    }

    fn as_boolean(&self) -> bool {
        *self != 0
    }
}

pub trait Transform : fmt::Debug {
    // TODO: Remove Boxing
    fn apply(&self, input: &dyn TransformInput) -> Box<dyn TransformOutput>;
    fn can_transform(&self, field_type: FieldType) -> bool;
    fn get_result_type(&self) -> FieldType;
}

pub struct TransformFactory {}

#[derive(Debug)]
pub struct Bucket {
    pub n: i32,
}

impl Bucket {
    pub fn new(n: i32) -> Bucket {
        Bucket { n }
    }
}

impl Transform for Bucket {
    fn apply(&self, input: &dyn TransformInput) -> Box<dyn TransformOutput> {
        return Box::new(input.as_integer() / self.n);
    }

    fn can_transform(&self, field_type: FieldType) -> bool {
        field_type == FieldType::Integer
    }

    fn get_result_type(&self) -> FieldType {
        FieldType::Integer
    }
}


pub fn get_transform(transform_name: &str) -> Option<Box<dyn Transform>> {
    match transform_name {
        "bucket" => Some(Box::new(Bucket::new(5))),
        _ => None,
    }
}