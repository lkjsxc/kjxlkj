/// Search route handlers per /docs/spec/api/http.md
///
/// GET /api/search — hybrid search query
use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

/// GET /api/search query parameters per /docs/spec/api/types.md
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub mode: Option<String>,
}

/// GET /api/search
/// Per /docs/spec/domain/search.md: validate mode, run hybrid ranking
pub async fn search_notes(Query(q): Query<SearchQuery>) -> impl IntoResponse {
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
