use std::{collections::HashMap, rc::Rc};

use crate::{
    partition::{PartitionSpec, PartitionSpecRef},
    rollback::Rollback,
    scan::TableScan,
    snapshot::{ExpireSnapshots, SnapshotRef},
    updates::{RewriteFiles, UpdateProperties, UpdateSchema},
};

#[derive(Debug)]
pub struct TableMetadata {
    pub properties: HashMap<String, String>,
}

impl TableMetadata {
    pub fn new(properties: HashMap<String, String>) -> Self {
        Self { properties }
    }

    pub fn replace_properties(&self, properties: HashMap<String, String>) -> TableMetadata {
        TableMetadata { properties }
    }
}

pub trait TableOperations: std::fmt::Debug {
    fn current(&self) -> TableMetadata;
    fn refresh(&self) -> TableMetadata;
    fn commit(&self, base: &TableMetadata, updated: &TableMetadata);
}

pub struct Table {}

impl Table {
    pub fn new() -> Self {
        Self {}
    }

    pub fn refresh(&mut self) {
        todo!()
    }

    pub fn new_scan(&self) -> Box<dyn TableScan> {
        todo!()
    }

    pub fn schema(&self) -> SchemaRef {
        todo!()
    }

    pub fn partition_spec(&self) -> PartitionSpecRef {
        todo!()
    }

    pub fn properties(&self) -> HashMap<String, String> {
        todo!()
    }

    pub fn location(&self) -> String {
        todo!()
    }

    pub fn current_snapshot(&self) -> SnapshotRef {
        todo!()
    }

    pub fn snapshots(&self) -> Vec<SnapshotRef> {
        todo!()
    }

    pub fn new_update_schema(&self) -> UpdateSchema {
        todo!()
    }

    pub fn new_update_properties(&self) -> UpdateProperties {
        todo!()
    }

    pub fn new_append(&self) -> AppendFiles {
        todo!()
    }

    pub fn new_rewrite(&self) -> RewriteFiles {
        todo!()
    }

    pub fn new_delete(&self) -> DeleteFiles {
        todo!()
    }

    pub fn new_expire_snapshots(&self) -> ExpireSnapshots {
        todo!()
    }

    pub fn new_rollback(&self) -> Rollback {
        todo!()
    }
}

pub type TableRef = Rc<Table>;

pub struct Schema {}

pub type SchemaRef = Rc<Schema>;

pub trait Expression {}

pub struct DeleteFiles {}

pub struct AppendFiles {}

pub trait TableFactory {
    fn create(&self, identifier: &str, partition_spec: PartitionSpec, schema: Schema) -> Table;
    fn load(&self, identifier: &str) -> Table;
}
