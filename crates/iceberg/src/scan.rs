use crate::table::{Expression, TableRef};

pub enum ScanTask {
    FileScanTask(FileScanTask),
    CombinedScanTask(CombinedScanTask),
}

pub struct FileScanTask {}

pub struct CombinedScanTask {}

pub trait TableScan {
    fn table(&self) -> TableRef;
    fn use_snapshot(&self, snapshot_id: u64) -> Box<dyn TableScan>;
    fn as_of_time(&self, timestamp: u64) -> Box<dyn TableScan>;
    fn select(&self, columns: Vec<String>) -> Box<dyn TableScan>;
    fn filter(&self, predicate: &dyn Expression) -> Box<dyn TableScan>;
    fn plan_files(&self) -> Vec<FileScanTask>;
    fn plan_tasks(&self) -> Vec<CombinedScanTask>;
    fn get_filter(&self) -> &dyn Expression;
}
