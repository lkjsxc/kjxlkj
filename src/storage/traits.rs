//! Storage trait definition

use crate::core::Record;
use async_trait::async_trait;
use thiserror::Error;

/// Error type for storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Record not found: {0}")]
    NotFound(String),
}

/// Generic storage trait for record persistence
#[async_trait]
pub trait Storage: Send + Sync {
    /// List all records sorted by ID
    async fn list(&self) -> Result<Vec<Record>, StorageError>;

    /// Get a single record by ID
    async fn get(&self, id: &str) -> Result<Option<Record>, StorageError>;

    /// Upsert a record, returns (record, created)
    async fn upsert(&self, id: &str, record: Record) -> Result<(Record, bool), StorageError>;

    /// Delete a record by ID, returns true if existed
    async fn delete(&self, id: &str) -> Result<bool, StorageError>;
}
