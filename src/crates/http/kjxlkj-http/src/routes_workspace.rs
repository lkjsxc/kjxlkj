/// Workspace route handlers per /docs/spec/api/http.md
///
/// GET  /api/workspaces — list all visible workspaces
/// POST /api/workspaces — create a new workspace
use crate::error_response::domain_error_response;
use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use kjxlkj_db::repo::WorkspaceRepo;
use kjxlkj_domain::workspace::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateWorkspaceInput {
    pub slug: String,
    pub name: String,
}

pub async fn list_workspaces(State(state): State<AppState>) -> Response {
    match state.workspace_repo.list_workspaces(Uuid::nil()) {
        Ok(list) => Json(serde_json::to_value(&list).unwrap()).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn create_workspace(
    State(state): State<AppState>,
    Json(input): Json<CreateWorkspaceInput>,
) -> Response {
    let now = chrono::Utc::now().naive_utc();
    let ws = Workspace {
        id: Uuid::new_v4(),
        slug: input.slug.clone(),
        name: input.name.clone(),
        owner_user_id: Uuid::nil(),
        state: WorkspaceState::Active,
        created_at: now,
        updated_at: now,
    };
    if let Err(e) = state.workspace_repo.create_workspace(&ws) {
        return domain_error_response(e);
    }
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": ws.id,
        "slug": ws.slug,
        "name": ws.name,
    }))).into_response()
}
