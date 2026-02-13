//! Data transfer objects for API requests and responses.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

// === Auth DTOs ===

#[derive(Debug, Deserialize)]
pub struct SetupRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
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
    pub display_name: Option<String>,
    pub global_role: String,
}

// === User DTOs ===

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub global_role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub global_role: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
}

// === Workspace DTOs ===

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UpsertMemberRequest {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub role: String,
}

// === Project DTOs ===

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
}

// === Note DTOs ===

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub note_kind: Option<String>,
    pub access_scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub body: String,
    pub base_version: u64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTitleRequest {
    pub title: String,
    pub base_version: u64,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub note_kind: String,
    pub access_scope: String,
    pub state: String,
    pub version: u64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct NoteHistoryResponse {
    pub id: Uuid,
    pub note_id: Uuid,
    pub event_type: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub version: u64,
    pub actor_id: Option<Uuid>,
    pub created_at: OffsetDateTime,
}

// === Automation DTOs ===

#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub workspace_id: Uuid,
    pub name: String,
    pub trigger: serde_json::Value,
    pub condition: Option<serde_json::Value>,
    pub action: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct RuleResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub trigger: serde_json::Value,
    pub condition: Option<serde_json::Value>,
    pub action: serde_json::Value,
    pub state: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct RunResponse {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub state: String,
    pub operations: Vec<serde_json::Value>,
    pub raw_model_output: Option<String>,
    pub parse_diagnostics: Option<String>,
    pub started_at: Option<OffsetDateTime>,
    pub completed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ReviewRunRequest {
    pub decisions: Vec<OperationDecision>,
}

#[derive(Debug, Deserialize)]
pub struct OperationDecision {
    pub operation_id: Uuid,
    pub decision: String, // "accept" or "reject"
}

// === Health DTOs ===

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub status: String,
    pub database: bool,
}
