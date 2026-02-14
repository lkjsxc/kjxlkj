// DTO types for API request/response per /docs/spec/api/types.md
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Error response per /docs/spec/api/errors.md
#[derive(Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

/// Setup register request
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

/// Login request
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Create user request
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub role: Option<String>,
}

/// Role update request
#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// Create workspace request
#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub slug: String,
    pub name: String,
}

/// Update workspace request
#[derive(Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: String,
}

/// Upsert member request
#[derive(Deserialize)]
pub struct UpsertMemberRequest {
    pub role: String,
}

/// Create project request
#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

/// Update project request
#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Create note request
#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: Option<String>,
    pub access_scope: Option<String>,
}

/// Update note (patch) request
#[derive(Deserialize)]
pub struct UpdateNoteRequest {
    pub base_version: i64,
    pub patch_ops: Vec<serde_json::Value>,
}

/// Update title request
#[derive(Deserialize)]
pub struct UpdateTitleRequest {
    pub base_version: i64,
    pub title: String,
}

/// Metadata upsert request
#[derive(Deserialize)]
pub struct MetadataRequest {
    pub value: serde_json::Value,
}

/// Create view request
#[derive(Deserialize)]
pub struct CreateViewRequest {
    pub workspace_id: Uuid,
    pub query_json: serde_json::Value,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
}

/// Update view request
#[derive(Deserialize)]
pub struct UpdateViewRequest {
    pub query_json: serde_json::Value,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
}

/// Session info response
#[derive(Serialize)]
pub struct SessionInfo {
    pub user_id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub csrf_token: String,
}

/// Search query params
#[derive(Deserialize)]
pub struct SearchQuery {
    pub workspace_id: Uuid,
    pub q: String,
}
