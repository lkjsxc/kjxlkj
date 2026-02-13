use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbWorkspace {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub state: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbWorkspaceMemberView {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: OffsetDateTime,
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbSessionWithUser {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub csrf_token: String,
    pub expires_at: OffsetDateTime,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub status: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbNoteStream {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub current_version: i32,
    pub deleted_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbNoteProjection {
    pub note_id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub version: i32,
    pub markdown: String,
    pub rendered_html: String,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbNoteEvent {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub seq: i32,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbWorkspaceEvent {
    pub workspace_id: Uuid,
    pub seq: i32,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbSavedView {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub owner_user_id: Uuid,
    pub query_json: serde_json::Value,
    pub sort: String,
    pub filters: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbBacklink {
    pub note_id: Uuid,
    pub target_title: String,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbSearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub note_kind: String,
    pub version: i32,
    pub markdown: String,
    pub rank: f32,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbAutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbAutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub triggering_event_id: String,
    pub status: String,
    pub provider_kind: Option<String>,
    pub model: Option<String>,
    pub result_json: serde_json::Value,
    pub error_code: Option<String>,
    pub error_detail: Option<String>,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbAdminJob {
    pub id: Uuid,
    pub requested_by: Uuid,
    pub workspace_id: Option<Uuid>,
    pub job_type: String,
    pub status: String,
    pub artifact_path: Option<String>,
    pub error_code: Option<String>,
    pub error_detail: Option<String>,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}
