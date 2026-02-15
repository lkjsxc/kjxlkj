//! Request/response DTOs per /docs/spec/api/types.md and /docs/spec/api/errors.md.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- Error envelope per /docs/spec/api/errors.md ---

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

impl ApiError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            details: None,
            request_id: Uuid::now_v7().to_string(),
        }
    }
    pub fn with_details(mut self, d: serde_json::Value) -> Self {
        self.details = Some(d);
        self
    }
}

// --- Setup & Auth ---

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResp {
    pub user_id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub csrf_token: String,
}

// --- Users ---

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleReq {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserResp {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub status: String,
    pub created_at: String,
}

// --- Workspaces ---

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceReq {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceReq {
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertMemberReq {
    pub role: String,
}

// --- Projects ---

#[derive(Debug, Deserialize)]
pub struct CreateProjectReq {
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectReq {
    pub name: Option<String>,
    pub description: Option<String>,
}

// --- Notes ---

#[derive(Debug, Deserialize)]
pub struct CreateNoteReq {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<String>,
    pub access_scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PatchNoteReq {
    pub base_version: i64,
    pub patch_ops: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTitleReq {
    pub base_version: i64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct RollbackReq {
    pub target_version: i64,
}

// --- Metadata ---

#[derive(Debug, Deserialize)]
pub struct UpsertMetadataReq {
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceTagsReq {
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub workspace_id: Uuid,
}

// --- Views ---

#[derive(Debug, Deserialize)]
pub struct CreateViewReq {
    pub workspace_id: Uuid,
    pub name: String,
    pub query_json: Option<serde_json::Value>,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateViewReq {
    pub name: Option<String>,
    pub query_json: Option<serde_json::Value>,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
}

// --- Automation ---

#[derive(Debug, Deserialize)]
pub struct CreateRuleReq {
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: Option<serde_json::Value>,
    pub action_json: serde_json::Value,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRuleReq {
    pub trigger: Option<String>,
    pub condition_json: Option<serde_json::Value>,
    pub action_json: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewReq {
    pub decisions: serde_json::Value,
}

// --- Dashboard ---

#[derive(Debug, Deserialize)]
pub struct UpsertWidgetReq {
    pub workspace_id: Uuid,
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: Option<serde_json::Value>,
}
