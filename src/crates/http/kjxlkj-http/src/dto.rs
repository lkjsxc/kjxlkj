use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Auth DTOs ──

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub user_id: Uuid,
    pub username: String,
    pub global_role: String,
}

// ── User DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub global_role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

// ── Workspace DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertMemberRequest {
    pub role: String,
}

// ── Project DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: String,
}

// ── Note DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub body: Option<String>,
    pub note_kind: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PatchNoteRequest {
    pub body: String,
    pub version: i64,
}

#[derive(Debug, Deserialize)]
pub struct PatchNoteTitleRequest {
    pub title: String,
    pub version: i64,
}

#[derive(Debug, Deserialize)]
pub struct RollbackRequest {
    pub target_version: i64,
}

// ── Metadata DTOs ──

#[derive(Debug, Deserialize)]
pub struct UpsertMetadataRequest {
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceTagsRequest {
    pub tags: Vec<String>,
}

// ── View DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateViewRequest {
    pub workspace_id: Uuid,
    pub name: String,
    pub filter: Option<serde_json::Value>,
    pub sort: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateViewRequest {
    pub name: Option<String>,
    pub filter: Option<serde_json::Value>,
    pub sort: Option<serde_json::Value>,
}

// ── Automation DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub workspace_id: Uuid,
    pub name: String,
    pub trigger: String,
    pub condition: Option<serde_json::Value>,
    pub action: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRuleRequest {
    pub name: Option<String>,
    pub trigger: Option<String>,
    pub condition: Option<serde_json::Value>,
    pub action: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LaunchRunRequest {
    pub trigger_event_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewRunRequest {
    pub decision: String, // "apply" or "reject"
    pub summary: Option<String>,
}

// ── Search DTOs ──

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub workspace_id: Uuid,
    pub q: String,
}
