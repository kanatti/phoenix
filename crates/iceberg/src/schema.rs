use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct NestedField {
    pub id: u32,
    pub name: String,
    pub field_type: String,
    pub required: bool,
}

#[derive(Debug)]
pub struct Schema {
    pub fields: Vec<NestedField>,
}

impl Schema {
    pub fn new(fields: Vec<NestedField>) -> Self {
        Schema { fields }
    }
}

pub type SchemaRef = Rc<Schema>;
