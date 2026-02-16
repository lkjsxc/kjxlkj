/// In-memory attachment repository per /docs/spec/domain/attachments.md
///
/// Stores attachment metadata and chunk data in memory.
/// Used for testing and development.
use crate::repo::AttachmentRepo;
use kjxlkj_domain::attachment::{AttachmentChunk, AttachmentMeta};
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// In-memory store for attachments.
pub struct InMemoryAttachmentRepo {
    metas: RwLock<HashMap<Uuid, AttachmentMeta>>,
    chunks: RwLock<HashMap<Uuid, Vec<AttachmentChunk>>>,
}

impl InMemoryAttachmentRepo {
    pub fn new() -> Self {
        Self {
            metas: RwLock::new(HashMap::new()),
            chunks: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryAttachmentRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl AttachmentRepo for InMemoryAttachmentRepo {
    fn create_attachment(&self, meta: &AttachmentMeta) -> Result<(), DomainError> {
        let mut metas = self.metas.write().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        if metas.contains_key(&meta.id) {
            return Err(DomainError::BadRequest("attachment already exists".into()));
        }
        metas.insert(meta.id, meta.clone());
        Ok(())
    }

    fn store_chunk(&self, chunk: &AttachmentChunk) -> Result<(), DomainError> {
        let mut chunks = self.chunks.write().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        chunks.entry(chunk.attachment_id).or_default().push(chunk.clone());
        Ok(())
    }

    fn get_attachment(&self, id: Uuid) -> Result<Option<AttachmentMeta>, DomainError> {
        let metas = self.metas.read().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        Ok(metas.get(&id).cloned())
    }

    fn list_attachments(&self, note_id: Uuid) -> Result<Vec<AttachmentMeta>, DomainError> {
        let metas = self.metas.read().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        Ok(metas.values().filter(|m| m.note_id == note_id).cloned().collect())
    }

    fn get_chunks(&self, attachment_id: Uuid) -> Result<Vec<AttachmentChunk>, DomainError> {
        let chunks = self.chunks.read().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        let mut result = chunks.get(&attachment_id).cloned().unwrap_or_default();
        result.sort_by_key(|c| c.chunk_index);
        Ok(result)
    }

    fn delete_attachment(&self, id: Uuid) -> Result<(), DomainError> {
        let mut metas = self.metas.write().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        let mut chunks = self.chunks.write().map_err(|_| DomainError::Internal("lock poisoned".into()))?;
        metas.remove(&id);
        chunks.remove(&id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn test_meta(id: Uuid, note_id: Uuid) -> AttachmentMeta {
        AttachmentMeta {
            id,
            note_id,
            filename: "test.png".into(),
            content_type: "image/png".into(),
            size_bytes: 1024,
            sha256: "abc123".into(),
            chunk_count: 1,
            created_at: NaiveDateTime::default(),
        }
    }

    fn test_chunk(att_id: Uuid, index: i32) -> AttachmentChunk {
        AttachmentChunk {
            attachment_id: att_id,
            chunk_index: index,
            data: vec![0u8; 16],
            sha256: format!("chunk_{}", index),
        }
    }

    #[test]
    fn create_and_get_attachment() {
        let repo = InMemoryAttachmentRepo::new();
        let id = Uuid::new_v4();
        let note_id = Uuid::new_v4();
        repo.create_attachment(&test_meta(id, note_id)).unwrap();
        let fetched = repo.get_attachment(id).unwrap().unwrap();
        assert_eq!(fetched.filename, "test.png");
    }

    #[test]
    fn list_attachments_by_note() {
        let repo = InMemoryAttachmentRepo::new();
        let note_id = Uuid::new_v4();
        let other_note = Uuid::new_v4();
        repo.create_attachment(&test_meta(Uuid::new_v4(), note_id)).unwrap();
        repo.create_attachment(&test_meta(Uuid::new_v4(), note_id)).unwrap();
        repo.create_attachment(&test_meta(Uuid::new_v4(), other_note)).unwrap();
        assert_eq!(repo.list_attachments(note_id).unwrap().len(), 2);
    }

    #[test]
    fn store_and_get_chunks_ordered() {
        let repo = InMemoryAttachmentRepo::new();
        let att_id = Uuid::new_v4();
        let note_id = Uuid::new_v4();
        repo.create_attachment(&test_meta(att_id, note_id)).unwrap();
        repo.store_chunk(&test_chunk(att_id, 2)).unwrap();
        repo.store_chunk(&test_chunk(att_id, 0)).unwrap();
        repo.store_chunk(&test_chunk(att_id, 1)).unwrap();
        let chunks = repo.get_chunks(att_id).unwrap();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].chunk_index, 0);
        assert_eq!(chunks[1].chunk_index, 1);
        assert_eq!(chunks[2].chunk_index, 2);
    }

    #[test]
    fn delete_removes_meta_and_chunks() {
        let repo = InMemoryAttachmentRepo::new();
        let att_id = Uuid::new_v4();
        let note_id = Uuid::new_v4();
        repo.create_attachment(&test_meta(att_id, note_id)).unwrap();
        repo.store_chunk(&test_chunk(att_id, 0)).unwrap();
        repo.delete_attachment(att_id).unwrap();
        assert!(repo.get_attachment(att_id).unwrap().is_none());
        assert!(repo.get_chunks(att_id).unwrap().is_empty());
    }

    #[test]
    fn duplicate_create_fails() {
        let repo = InMemoryAttachmentRepo::new();
        let id = Uuid::new_v4();
        let note_id = Uuid::new_v4();
        repo.create_attachment(&test_meta(id, note_id)).unwrap();
        assert!(repo.create_attachment(&test_meta(id, note_id)).is_err());
    }
}
