use std::collections::{HashMap, HashSet};

use crate::{
    schema::Schema,
    snapshot::{DataFile, SnapshotRef},
    table::{TableMetadata, TableOperations},
};

pub enum UpdateError {}

/// An update that can be committed
pub trait PendingUpdate {
    type Changes;

    /// Apply the update without commit to see changes.
    fn apply(&mut self) -> Result<Self::Changes, UpdateError>;

    /// Commit the updates
    fn commit(&mut self) -> Result<(), UpdateError>;
}

/// Update table properties.
#[derive(Debug)]
pub struct UpdateProperties {
    updates: HashMap<String, String>,
    removals: HashSet<String>,
    ops: Box<dyn TableOperations>,
    base: TableMetadata,
}

impl UpdateProperties {
    pub fn new(ops: Box<dyn TableOperations>) -> Self {
        let base = ops.current();
        Self {
            updates: HashMap::new(),
            removals: HashSet::new(),
            ops,
            base,
        }
    }

    /// Add a key/value property to the table.
    pub fn set(&mut self, key: &str, value: &str) {
        self.updates.insert(key.to_string(), value.to_string());
    }

    /// Remove the given property key from the table.
    pub fn remove(&mut self, key: &str) {
        self.removals.insert(key.to_string());
    }
}

impl PendingUpdate for UpdateProperties {
    type Changes = HashMap<String, String>;

    fn apply(&mut self) -> Result<Self::Changes, UpdateError> {
        self.base = self.ops.refresh();

        let mut changes = HashMap::new();

        // Remove fields
        for (key, value) in self.base.properties.iter() {
            if !self.removals.contains(key) {
                changes.insert(key.clone(), value.clone());
            }
        }

        // Add new fields
        for (key, value) in self.updates.iter() {
            changes.insert(key.clone(), value.clone());
        }

        Ok(changes)
    }

    // TODO: Add retry and backoff
    fn commit(&mut self) -> Result<(), UpdateError> {
        let changes = self.apply()?;
        let updated = self.base.replace_properties(changes);
        self.ops.commit(&self.base, &updated);
        Ok(())
    }
}

pub struct UpdateSchema {}

impl UpdateSchema {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_column(&mut self, name: &str, data_type: &str) {
        todo!()
    }

    pub fn add_child_column(&mut self, parent: &str, name: &str, data_type: &str) {
        todo!()
    }

    pub fn rename_column(&mut self, name: &str, new_name: &str) {
        todo!()
    }

    pub fn update_column_type(&mut self, name: &str, new_type: &str) {
        todo!()
    }

    pub fn delete_column(&mut self, name: &str) {
        todo!()
    }
}

impl PendingUpdate for UpdateSchema {
    type Changes = Schema;

    fn apply(&mut self) -> Result<Self::Changes, UpdateError> {
        todo!()
    }

    fn commit(&mut self) -> Result<(), UpdateError> {
        todo!()
    }
}

pub struct RewriteFiles {}

impl RewriteFiles {
    pub fn new(files_to_delete: Vec<DataFile>, files_to_add: Vec<DataFile>) -> Self {
        Self {}
    }
}

impl PendingUpdate for RewriteFiles {
    type Changes = SnapshotRef;

    fn apply(&mut self) -> Result<Self::Changes, UpdateError> {
        todo!()
    }

    fn commit(&mut self) -> Result<(), UpdateError> {
        todo!()
    }
}
