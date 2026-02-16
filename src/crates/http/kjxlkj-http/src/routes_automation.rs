/// Automation route handlers per /docs/spec/api/http.md
///
/// GET  /api/automation/rules          — list rules
/// POST /api/automation/rules          — create rule
/// PATCH /api/automation/rules/{id}    — update rule
/// POST /api/automation/rules/{id}/launch — launch run
/// GET  /api/automation/runs           — list runs
/// GET  /api/automation/runs/{id}      — get run
/// POST /api/automation/runs/{id}/review — review run
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

/// GET /api/automation/rules
pub async fn list_rules() -> impl IntoResponse {
    Json(serde_json::json!([]))
}

/// POST /api/automation/rules
pub async fn create_rule(
    Json(_input): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": Uuid::new_v4(),
    })))
}

/// PATCH /api/automation/rules/{id}
pub async fn update_rule(
    Path(_id): Path<Uuid>,
    Json(_input): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({"updated": true}))
}

/// POST /api/automation/rules/{id}/launch
/// Per /docs/spec/domain/automation.md: creates a run in "queued" state
pub async fn launch_rule(Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::ACCEPTED, Json(serde_json::json!({
        "run_id": Uuid::new_v4(),
        "status": "queued",
    })))
}

/// GET /api/automation/runs
pub async fn list_runs() -> impl IntoResponse {
    Json(serde_json::json!([]))
}

/// GET /api/automation/runs/{id}
pub async fn get_run(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": id,
        "status": "queued",
    }))
}

/// POST /api/automation/runs/{id}/review
/// Per /docs/spec/domain/automation.md: accept/reject operations
pub async fn review_run(
    Path(_id): Path<Uuid>,
    Json(_input): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({"reviewed": true}))
}
