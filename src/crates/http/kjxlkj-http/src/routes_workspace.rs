/// Workspace route handlers per /docs/spec/api/http.md
///
/// GET  /api/workspaces — list all visible workspaces
/// POST /api/workspaces — create a new workspace
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

/// POST /api/workspaces payload per /docs/spec/api/types.md
#[derive(Deserialize)]
pub struct CreateWorkspaceInput {
    pub slug: String,
    pub name: String,
}

/// GET /api/workspaces
pub async fn list_workspaces() -> impl IntoResponse {
    Json(serde_json::json!([]))
}

/// POST /api/workspaces
/// Per /docs/spec/domain/workspaces.md: slug must be unique.
pub async fn create_workspace(
    Json(input): Json<CreateWorkspaceInput>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "slug": input.slug,
        "name": input.name,
    })))
}
