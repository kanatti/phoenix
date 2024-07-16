use async_trait::async_trait;
use arrow::record_batch::RecordBatch;
use parquet::file::properties::WriterProperties;

// Trait for storage operations
#[async_trait]
pub trait Store: Send + Sync {
    async fn write(&self, data: RecordBatch, path: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn notify_catalog(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

// Remote store implementation
pub struct RemoteStore {
    // Add fields for remote storage configuration
}

#[async_trait]
impl Store for RemoteStore {
    async fn write(&self, data: RecordBatch, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implement remote write logic
        todo!()
    }

    async fn notify_catalog(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    async fn write(&self, data: RecordBatch, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = format!("{}/{}", self.base_path, path);
        let file = std::fs::File::create(full_path)?;
        let mut writer = parquet::arrow::ArrowWriter::try_new(
            file,
            data.schema(),
            Some(WriterProperties::builder().build()),
        )?;
        writer.write(&data)?;
        writer.close()?;
        Ok(())
    }

    async fn notify_catalog(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Notifying catalog about: {}", path);
        Ok(())
    }
}