//! Filesystem-backed storage implementation

use crate::core::Record;
use crate::storage::traits::{Storage, StorageError};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

/// Filesystem storage for records
pub struct FilesystemStorage {
    root: PathBuf,
}

impl FilesystemStorage {
    /// Create a new filesystem storage with the given root directory
    pub async fn new(root: PathBuf) -> Result<Self, StorageError> {
        fs::create_dir_all(&root).await?;
        let records_dir = root.join("records");
        fs::create_dir_all(&records_dir).await?;
        Ok(Self { root })
    }

    fn records_dir(&self) -> PathBuf {
        self.root.join("records")
    }

    fn record_path(&self, id: &str) -> PathBuf {
        self.records_dir().join(format!("{}.json", id))
    }
}

#[async_trait]
impl Storage for FilesystemStorage {
    async fn list(&self) -> Result<Vec<Record>, StorageError> {
        let mut records = Vec::new();
        let mut entries = fs::read_dir(self.records_dir()).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                let content = fs::read_to_string(&path).await?;
                let record: Record = serde_json::from_str(&content)?;
                records.push(record);
            }
        }

        records.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(records)
    }

    async fn get(&self, id: &str) -> Result<Option<Record>, StorageError> {
        let path = self.record_path(id);
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path).await?;
        let record: Record = serde_json::from_str(&content)?;
        Ok(Some(record))
    }

    async fn upsert(&self, id: &str, record: Record) -> Result<(Record, bool), StorageError> {
        let path = self.record_path(id);
        let created = !path.exists();

        let json = serde_json::to_string_pretty(&record)?;
        let temp_path = self.records_dir().join(format!(".{}.tmp", id));
        fs::write(&temp_path, &json).await?;
        fs::rename(&temp_path, &path).await?;

        Ok((record, created))
    }

    async fn delete(&self, id: &str) -> Result<bool, StorageError> {
        let path = self.record_path(id);
        if !path.exists() {
            return Ok(false);
        }
        fs::remove_file(&path).await?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn setup() -> (FilesystemStorage, TempDir) {
        let dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(dir.path().to_path_buf())
            .await
            .unwrap();
        (storage, dir)
    }

    #[tokio::test]
    async fn empty_list() {
        let (storage, _dir) = setup().await;
        let records = storage.list().await.unwrap();
        assert!(records.is_empty());
    }

    #[tokio::test]
    async fn upsert_and_get() {
        let (storage, _dir) = setup().await;
        let record = Record::new(
            "test-id".to_string(),
            "Title".to_string(),
            "Body".to_string(),
            vec![],
        );
        let (saved, created) = storage.upsert("test-id", record.clone()).await.unwrap();
        assert!(created);
        assert_eq!(saved.id, "test-id");

        let fetched = storage.get("test-id").await.unwrap().unwrap();
        assert_eq!(fetched.title, "Title");
    }

    #[tokio::test]
    async fn delete_existing() {
        let (storage, _dir) = setup().await;
        let record = Record::new(
            "del-id".to_string(),
            "Title".to_string(),
            "Body".to_string(),
            vec![],
        );
        storage.upsert("del-id", record).await.unwrap();
        assert!(storage.delete("del-id").await.unwrap());
        assert!(storage.get("del-id").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_nonexistent() {
        let (storage, _dir) = setup().await;
        assert!(!storage.delete("nope").await.unwrap());
    }

    #[tokio::test]
    async fn list_sorted() {
        let (storage, _dir) = setup().await;
        for id in ["zzz", "aaa", "mmm"] {
            let record = Record::new(id.to_string(), "T".to_string(), "B".to_string(), vec![]);
            storage.upsert(id, record).await.unwrap();
        }
        let records = storage.list().await.unwrap();
        let ids: Vec<_> = records.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, vec!["aaa", "mmm", "zzz"]);
    }
}
