use serde::{Deserialize, Serialize};

/// Error response envelope per /docs/spec/api/errors.md.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

/// Setup/register request DTO.
#[derive(Debug, Deserialize)]
pub struct SetupRegisterRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

/// Login request DTO.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Session response DTO.
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub csrf_token: String,
}

/// Create user request DTO.
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub role: String,
}

/// User response DTO.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub status: String,
    pub created_at: String,
}

/// Create workspace request.
#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub slug: String,
    pub name: String,
}

/// Workspace response.
#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: uuid::Uuid,
    pub created_at: String,
}

/// Create note request.
#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: uuid::Uuid,
    pub project_id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<String>,
    pub access_scope: Option<String>,
}

/// Note stream response.
#[derive(Debug, Serialize)]
pub struct NoteStreamResponse {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub project_id: Option<uuid::Uuid>,
    pub title: String,
    pub note_kind: String,
    pub current_version: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Note projection response.
#[derive(Debug, Serialize)]
pub struct NoteProjectionResponse {
    pub note_id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub title: String,
    pub note_kind: String,
    pub version: i64,
    pub markdown: String,
    pub rendered_html: String,
    pub metadata_json: serde_json::Value,
}

/// Patch note request.
#[derive(Debug, Deserialize)]
pub struct PatchNoteRequest {
    pub base_version: i64,
    pub ops: Vec<serde_json::Value>,
}

/// Patch title request.
#[derive(Debug, Deserialize)]
pub struct PatchTitleRequest {
    pub base_version: i64,
    pub title: String,
}

/// Rollback note request per /docs/spec/api/http.md.
#[derive(Debug, Deserialize)]
pub struct RollbackNoteRequest {
    pub target_version: i64,
}

/// Upsert metadata request per /docs/spec/domain/metadata.md.
#[derive(Debug, Deserialize)]
pub struct UpsertMetadataRequest {
    pub value: serde_json::Value,
}

/// Replace tags request per /docs/spec/api/http.md.
#[derive(Debug, Deserialize)]
pub struct ReplaceTagsRequest {
    pub tags: Vec<String>,
}

/// Tag response.
#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub name: String,
}

/// Search result response.
#[derive(Debug, Serialize)]
pub struct SearchResultResponse {
    pub note_id: uuid::Uuid,
    pub title: String,
    pub rank: f32,
}

/// Backlink response.
#[derive(Debug, Serialize)]
pub struct BacklinkResponse {
    pub source_note_id: uuid::Uuid,
    pub title: String,
}

/// Attachment response.
#[derive(Debug, Serialize)]
pub struct AttachmentResponse {
    pub id: uuid::Uuid,
    pub note_id: uuid::Uuid,
    pub filename: String,
    pub mime: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub chunk_count: i32,
    pub created_at: String,
}

/// Upsert member request.
#[derive(Debug, Deserialize)]
pub struct UpsertMemberRequest {
    pub role: String,
}

/// Member response.
#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub workspace_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub role: String,
    pub joined_at: String,
}

// Re-export automation/export DTOs from split module.
pub use crate::dto_automation::*;
