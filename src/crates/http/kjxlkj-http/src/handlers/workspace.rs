//! Workspace handlers

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use kjxlkj_domain::Workspace;
use crate::state::{HttpResult, HttpError};
use crate::routes::HttpState;

/// Create workspace request
#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
}

/// List workspaces response
#[derive(Debug, Serialize)]
pub struct ListWorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

/// List all workspaces
pub async fn list_workspaces(
    State(state): State<HttpState>,
    Extension(user_id): Extension<Uuid>,
) -> HttpResult<Json<ListWorkspacesResponse>> {
    // Stub implementation
    Ok(Json(ListWorkspacesResponse {
        workspaces: vec![],
    }))
}

/// Create workspace
pub async fn create_workspace(
    State(state): State<HttpState>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> HttpResult<(StatusCode, Json<Workspace>)> {
    // Stub implementation
    let workspace = Workspace::new(req.name, user_id);
    Ok((StatusCode::CREATED, Json(workspace)))
}
