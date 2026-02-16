/// HTTP route definitions per /docs/spec/api/http.md
///
/// Base path: /api
use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Build the complete API router per /docs/spec/api/http.md
pub fn api_router() -> Router {
    Router::new()
        // Auth and Session
        .route("/api/setup/register", post(setup_register))
        .route("/api/auth/login", post(auth_login))
        .route("/api/auth/logout", post(auth_logout))
        .route("/api/auth/session", get(auth_session))
        // Workspaces
        .route("/api/workspaces", get(list_workspaces))
        .route("/api/workspaces", post(create_workspace))
        // Notes
        .route("/api/notes", get(list_notes))
        .route("/api/notes", post(create_note))
        .route("/api/notes/{id}", get(get_note))
        .route("/api/notes/{id}", patch(patch_note))
        .route("/api/notes/{id}", delete(delete_note))
        .route("/api/notes/{id}/title", patch(update_title))
        .route("/api/notes/{id}/history", get(note_history))
        .route("/api/notes/{id}/backlinks", get(note_backlinks))
        // Search
        .route("/api/search", get(search_notes))
        // Automation
        .route("/api/automation/rules", get(list_rules))
        .route("/api/automation/rules", post(create_rule))
        .route("/api/automation/rules/{id}", patch(update_rule))
        .route("/api/automation/rules/{id}/launch", post(launch_rule))
        .route("/api/automation/runs", get(list_runs))
        .route("/api/automation/runs/{id}", get(get_run))
        .route("/api/automation/runs/{id}/review", post(review_run))
        // Health
        .route("/api/healthz", get(healthz))
        .route("/api/readyz", get(readyz))
}

// === Stub handlers ===
// These return structure-correct responses for compilation and type-checking.
// Full DB-backed implementations require PostgreSQL runtime.

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn healthz() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}

async fn readyz() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}

#[derive(Deserialize)]
struct RegisterInput {
    username: String,
    password: String,
}

async fn setup_register(Json(input): Json<RegisterInput>) -> impl IntoResponse {
    // Stub: would check user_count == 0, then create owner
    (StatusCode::CREATED, Json(serde_json::json!({
        "message": "owner created",
        "username": input.username,
    })))
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

async fn auth_login(Json(_input): Json<LoginInput>) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "message": "session created"
    })))
}

async fn auth_logout() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

async fn auth_session() -> impl IntoResponse {
    Json(serde_json::json!({
        "authenticated": false
    }))
}

async fn list_workspaces() -> impl IntoResponse {
    Json(serde_json::json!([]))
}

#[derive(Deserialize)]
struct CreateWorkspaceInput {
    slug: String,
    name: String,
}

async fn create_workspace(Json(input): Json<CreateWorkspaceInput>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "slug": input.slug,
        "name": input.name,
    })))
}

#[derive(Deserialize)]
struct ListNotesQuery {
    workspace_id: Option<Uuid>,
    include_deleted: Option<bool>,
}

async fn list_notes(Query(_q): Query<ListNotesQuery>) -> impl IntoResponse {
    Json(serde_json::json!([]))
}

#[derive(Deserialize)]
struct CreateNoteBody {
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    title: Option<String>,
    note_kind: Option<String>,
    markdown: Option<String>,
}

async fn create_note(Json(input): Json<CreateNoteBody>) -> impl IntoResponse {
    // Per /docs/spec/domain/notes.md: default title rule
    let title = input
        .title
        .unwrap_or_else(kjxlkj_domain::note::default_note_title);
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "title": title,
        "workspace_id": input.workspace_id,
        "note_kind": input.note_kind.unwrap_or_else(|| "markdown".into()),
        "current_version": 1,
    })))
}

async fn get_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "title": "stub",
        "version": 1,
        "markdown": "",
        "metadata_json": {},
    }))
}

#[derive(Deserialize)]
struct PatchNoteBody {
    base_version: i64,
    markdown: Option<String>,
}

async fn patch_note(
    Path(id): Path<Uuid>,
    Json(_input): Json<PatchNoteBody>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "version": 2,
    }))
}

async fn delete_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "note_id": id,
        "state": "soft_deleted",
    })))
}

#[derive(Deserialize)]
struct UpdateTitleBody {
    base_version: i64,
    title: String,
}

async fn update_title(
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTitleBody>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "title": input.title,
        "version": input.base_version + 1,
    }))
}

async fn note_history(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "events": [],
    }))
}

async fn note_backlinks(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "backlinks": [],
    }))
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    limit: Option<i64>,
    mode: Option<String>,
}

async fn search_notes(Query(q): Query<SearchQuery>) -> impl IntoResponse {
    // Validate mode per /docs/spec/api/http.md
    if let Some(ref mode) = q.mode {
        if kjxlkj_domain::search::SearchMode::from_str(mode).is_none() {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({
                    "code": "SEARCH_MODE_INVALID",
                    "message": format!("unknown search mode: {}", mode),
                    "details": null,
                    "request_id": Uuid::new_v4().to_string(),
                })),
            );
        }
    }
    (StatusCode::OK, Json(serde_json::json!([])))
}

async fn list_rules() -> impl IntoResponse {
    Json(serde_json::json!([]))
}

async fn create_rule(Json(_input): Json<serde_json::Value>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": Uuid::new_v4(),
    })))
}

async fn update_rule(
    Path(_id): Path<Uuid>,
    Json(_input): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({"updated": true}))
}

async fn launch_rule(Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::ACCEPTED, Json(serde_json::json!({
        "run_id": Uuid::new_v4(),
        "status": "queued",
    })))
}

async fn list_runs() -> impl IntoResponse {
    Json(serde_json::json!([]))
}

async fn get_run(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": id,
        "status": "queued",
    }))
}

async fn review_run(
    Path(_id): Path<Uuid>,
    Json(_input): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({"reviewed": true}))
}
