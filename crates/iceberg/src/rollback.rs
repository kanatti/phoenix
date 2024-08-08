pub struct Rollback {}

impl Rollback {
    pub fn new() -> Self {
        Rollback {}
    }

    pub fn to_snapshot_id(&self, snapshot_id: u64) -> Rollback {
        todo!()
    }

    pub fn to_snapshot_at_time(&self, timestamp_millis: u64) -> Rollback {
        todo!()
    }
}
