use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{auth::request_id, state::AppState};

pub async fn healthz() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.db_pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "ready" }))).into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "not-ready" })),
        )
            .into_response(),
    }
}

pub async fn not_implemented(headers: HeaderMap) -> impl IntoResponse {
    let rid = request_id(&headers);
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "code": "NOT_IMPLEMENTED",
            "message": "endpoint is reachable but not fully implemented yet",
            "details": null,
            "request_id": rid
        })),
    )
}
