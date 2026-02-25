//! Workspace handlers

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use kjxlkj_domain::Workspace;
use crate::state::HttpResult;
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
    State(_state): State<HttpState>,
) -> HttpResult<Json<ListWorkspacesResponse>> {
    // Stub implementation
    Ok(Json(ListWorkspacesResponse {
        workspaces: vec![],
    }))
}

/// Create workspace
pub async fn create_workspace(
    State(_state): State<HttpState>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> HttpResult<(StatusCode, Json<Workspace>)> {
    // Stub implementation
    let workspace = Workspace::new(req.name, Uuid::new_v4());
    Ok((StatusCode::CREATED, Json(workspace)))
}
