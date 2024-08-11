use std::rc::Rc;

use super::PartitionField;

#[derive(Debug)]
pub struct PartitionSpec {
    partition_fields: Vec<PartitionField>,
}

impl PartitionSpec {
    pub fn new(partition_fields: Vec<PartitionField>) -> PartitionSpec {
        PartitionSpec { partition_fields }
    }
}

pub type PartitionSpecRef = Rc<PartitionSpec>;
