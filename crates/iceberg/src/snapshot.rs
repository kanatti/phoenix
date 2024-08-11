use std::rc::Rc;

/// A snapshot of the data in a table at a point in time.
pub trait Snapshot {
    fn snapshot_id(&self) -> u64;
    fn timestamp_millis(&self) -> u64;
    fn manifests(&self) -> Vec<String>;
    fn added_files(&self) -> Vec<DataFile>;
    fn deleted_files(&self) -> Vec<DataFile>;
}

pub type SnapshotRef = Rc<dyn Snapshot>;

pub struct ExpireSnapshots {}

pub struct DataFile {}
