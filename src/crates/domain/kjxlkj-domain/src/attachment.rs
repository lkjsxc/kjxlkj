/// Attachment domain types per /docs/spec/domain/attachments.md
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Chunk size: 4 MiB per /docs/spec/domain/attachments.md
pub const CHUNK_SIZE: usize = 4 * 1024 * 1024;
/// Per-file max: 500 MiB
pub const MAX_FILE_SIZE: usize = 500 * 1024 * 1024;

/// Attachment metadata row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentMeta {
    pub id: Uuid,
    pub note_id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub chunk_count: i32,
    pub created_at: NaiveDateTime,
}

/// Attachment chunk row
#[derive(Debug, Clone)]
pub struct AttachmentChunk {
    pub attachment_id: Uuid,
    pub chunk_index: i32,
    pub data: Vec<u8>,
    pub sha256: String,
}
