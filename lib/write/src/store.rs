use std::{fs::{self, File}, path::Path};

use async_trait::async_trait;
use arrow::record_batch::RecordBatch;
use parquet::file::properties::WriterProperties;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Failed to create directory: {0}")]
    DirectoryCreationError(#[from] std::io::Error),
    #[error("Failed to create file: {0}")]
    FileCreationError(std::io::Error),
    #[error("Failed to create Parquet writer: {0}")]
    ParquetWriterCreationError(#[from] parquet::errors::ParquetError),
}

// Trait for storage operations
#[async_trait]
pub trait Store: Send + Sync {
    async fn write(&self, data: RecordBatch, path: &str) -> Result<(), StoreError>;
    async fn notify_catalog(&self, path: &str) -> Result<(), StoreError>;
}

// Remote store implementation
pub struct RemoteStore {
    // Add fields for remote storage configuration
}

#[async_trait]
impl Store for RemoteStore {
    async fn write(&self, data: RecordBatch, path: &str) -> Result<(), StoreError> {
        // Implement remote write logic
        todo!()
    }

    async fn notify_catalog(&self, path: &str) -> Result<(), StoreError> {
        // Implement catalog notification logic
        todo!()
    }
}

// Local store wrapper for development
pub struct LocalStore {
    pub base_path: String,
}


#[async_trait]
impl Store for LocalStore {
    async fn write(&self, data: RecordBatch, path: &str) -> Result<(), StoreError> {
        let full_path = Path::new(&self.base_path).join(path);
    
        // Ensure the directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).map_err(StoreError::DirectoryCreationError)?;
        }
    
        let file = File::create(&full_path).map_err(StoreError::FileCreationError)?;

        let mut writer = parquet::arrow::ArrowWriter::try_new(
            file,
            data.schema(),
            Some(WriterProperties::builder().build()),
        )?;
        writer.write(&data)?;
        writer.close()?;
        Ok(())
    }

    async fn notify_catalog(&self, path: &str) -> Result<(), StoreError> {
        println!("Notifying catalog about: {}", path);
        Ok(())
    }
}