//! DTOs for views, dashboards, projects, and media notes.
//! Split per 200-line policy.
use serde::{Deserialize, Serialize};

/// Create saved view request per /docs/spec/api/http.md.
#[derive(Debug, Deserialize)]
pub struct CreateViewRequest {
    pub workspace_id: uuid::Uuid,
    pub name: String,
    pub query_json: serde_json::Value,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
}

/// Update saved view request.
#[derive(Debug, Deserialize)]
pub struct UpdateViewRequest {
    pub name: Option<String>,
    pub query_json: Option<serde_json::Value>,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
}

/// Saved view response per /docs/spec/api/types.md.
#[derive(Debug, Serialize)]
pub struct SavedViewResponse {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub name: String,
    pub query_json: serde_json::Value,
    pub sort: Option<String>,
    pub filters: serde_json::Value,
    pub owner_user_id: uuid::Uuid,
    pub created_at: String,
}

/// Dashboard widget upsert request per /docs/spec/api/http.md.
#[derive(Debug, Deserialize)]
pub struct UpsertWidgetRequest {
    pub id: Option<uuid::Uuid>,
    pub workspace_id: uuid::Uuid,
    #[serde(rename = "type")]
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: Option<serde_json::Value>,
}

/// Dashboard widget response per /docs/spec/api/types.md.
#[derive(Debug, Serialize)]
pub struct DashboardWidgetResponse {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    #[serde(rename = "type")]
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: Option<serde_json::Value>,
    pub created_at: String,
}

/// Update project request per /docs/spec/api/http.md.
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Project response per /docs/spec/api/types.md.
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

/// Media note creation request per /docs/spec/api/http.md.
#[derive(Debug, Deserialize)]
pub struct CreateMediaNoteRequest {
    pub workspace_id: uuid::Uuid,
    pub project_id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub note_kind: String,
}
