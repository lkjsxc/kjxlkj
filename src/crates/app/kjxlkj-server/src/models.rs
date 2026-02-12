use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::patch::PatchOp;

#[derive(Debug, Deserialize)]
pub struct SetupRegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub user_id: Uuid,
    pub email: String,
    pub csrf_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    #[serde(default)]
    pub markdown: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchNoteRequest {
    pub base_version: i64,
    pub patch_ops: Vec<PatchOp>,
    pub idempotency_key: String,
}

#[derive(Debug, Deserialize)]
pub struct RollbackRequest {
    pub target_version: i64,
}

#[derive(Debug, Deserialize)]
pub struct MetadataValueRequest {
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceTagsRequest {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct NoteSummary {
    pub id: Uuid,
    pub title: String,
    pub current_version: i64,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct NoteProjection {
    pub note_id: Uuid,
    pub title: String,
    pub version: i64,
    pub markdown: String,
    pub rendered_html: Option<String>,
    pub metadata_json: serde_json::Value,
    pub tags: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct NoteEvent {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AttachmentMeta {
    pub id: Uuid,
    pub note_id: Uuid,
    pub filename: String,
    pub mime: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub chunk_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct JobStatus {
    pub id: Uuid,
    pub kind: String,
    pub status: String,
    pub artifact_path: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
