use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::patch::PatchOp;

#[derive(Debug, Deserialize)]
pub struct SetupRegisterRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
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
    pub csrf_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
    #[serde(default = "default_role_viewer")]
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Workspace {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub slug: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceMemberUpsertRequest {
    pub role: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    #[serde(default)]
    pub markdown: String,
    #[serde(default)]
    pub workspace_id: Option<Uuid>,
    #[serde(default)]
    pub project_id: Option<Uuid>,
    #[serde(default = "default_note_kind")]
    pub note_kind: String,
    #[serde(default = "default_access_scope")]
    pub access_scope: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMediaNoteRequest {
    pub workspace_id: Uuid,
    pub title: String,
    pub media_url: String,
    #[serde(default = "default_note_kind_media")]
    pub note_kind: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchTitleRequest {
    pub base_version: i64,
    pub title: String,
    pub idempotency_key: String,
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
    pub workspace_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub current_version: i64,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct NoteProjection {
    pub note_id: Uuid,
    pub workspace_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
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
pub struct SavedView {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub query_json: serde_json::Value,
    pub sort: String,
    pub filters: serde_json::Value,
    pub owner_user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SavedViewUpsertRequest {
    pub workspace_id: Uuid,
    #[serde(default)]
    pub query_json: serde_json::Value,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default)]
    pub filters: serde_json::Value,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DashboardWidgetUpsertRequest {
    pub id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub r#type: String,
    #[serde(default)]
    pub config_json: serde_json::Value,
    #[serde(default)]
    pub layout: serde_json::Value,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AutomationRuleRequest {
    pub workspace_id: Uuid,
    pub trigger: String,
    #[serde(default)]
    pub condition_json: serde_json::Value,
    #[serde(default)]
    pub action_json: serde_json::Value,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub result_json: Option<serde_json::Value>,
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

fn default_note_kind() -> String {
    "markdown".to_string()
}

fn default_note_kind_media() -> String {
    "media_image".to_string()
}

fn default_access_scope() -> String {
    "workspace".to_string()
}

fn default_sort() -> String {
    "updated_desc".to_string()
}

fn default_enabled() -> bool {
    true
}

fn default_role_viewer() -> String {
    "viewer".to_string()
}
