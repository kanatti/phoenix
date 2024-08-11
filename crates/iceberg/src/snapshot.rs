use std::rc::Rc;

/// A snapshot of the data in a table at a point in time.
#[derive(Debug)]
pub struct Snapshot {
    snapshot_id: u64,
    timestamp_ms: u64,
    manifests: Vec<String>,
    added_files: Vec<DataFile>,
    deleted_files: Vec<DataFile>,
}

impl Snapshot {
    pub fn new(
        snapshot_id: u64,
        timestamp_ms: u64,
        manifests: Vec<String>
    ) -> Self {
        Snapshot {
            snapshot_id,
            timestamp_ms,
            manifests,
            added_files: Vec::new(),
            deleted_files: Vec::new(),
        }
    }
}

pub type SnapshotRef = Rc<Snapshot>;

pub struct ExpireSnapshots {}

#[derive(Debug)]
pub struct DataFile {}
