use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::core::{normalize_tags, validate_id, validate_input, Record, RecordInput};
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct FsStore {
    root: PathBuf,
}

impl FsStore {
    pub fn new(data_root: PathBuf) -> Self {
        Self {
            root: data_root.join("records"),
        }
    }

    pub async fn ensure_ready(&self) -> Result<(), AppError> {
        tokio::fs::create_dir_all(&self.root).await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Record>, AppError> {
        let mut entries = tokio::fs::read_dir(&self.root).await?;
        let mut output = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if !is_json_file(&path) {
                continue;
            }
            let text = tokio::fs::read_to_string(path).await?;
            output.push(serde_json::from_str::<Record>(&text)?);
        }
        output.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(output)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Record>, AppError> {
        validate_id(id)?;
        let path = self.path_for(id);
        match tokio::fs::read_to_string(path).await {
            Ok(text) => Ok(Some(serde_json::from_str::<Record>(&text)?)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(AppError::Io(error)),
        }
    }

    pub async fn upsert(&self, id: &str, input: RecordInput) -> Result<(Record, bool), AppError> {
        validate_id(id)?;
        validate_input(&input)?;
        let existing = self.get(id).await?;
        let created = existing.is_none();
        let revision = existing.as_ref().map_or(1, |record| record.revision + 1);
        let record = Record {
            id: id.to_owned(),
            title: input.title.trim().to_owned(),
            body: input.body,
            tags: normalize_tags(&input.tags),
            revision,
            updated_at: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        };
        self.write_record(&record).await?;
        Ok((record, created))
    }

    pub async fn delete(&self, id: &str) -> Result<bool, AppError> {
        validate_id(id)?;
        let path = self.path_for(id);
        match tokio::fs::remove_file(path).await {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(AppError::Io(error)),
        }
    }

    async fn write_record(&self, record: &Record) -> Result<(), AppError> {
        let path = self.path_for(&record.id);
        let tmp = self.root.join(format!("{}.tmp", record.id));
        let text = serde_json::to_string_pretty(record)?;
        tokio::fs::write(&tmp, text).await?;
        tokio::fs::rename(tmp, path).await?;
        Ok(())
    }

    fn path_for(&self, id: &str) -> PathBuf {
        self.root.join(format!("{id}.json"))
    }
}

fn is_json_file(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("json")
}
