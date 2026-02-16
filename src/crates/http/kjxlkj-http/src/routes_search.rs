/// Search route handlers per /docs/spec/api/http.md
///
/// GET /api/search — hybrid search query
use crate::error_response::domain_error_response;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use kjxlkj_db::repo::SearchRepo;
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
pub async fn search_notes(
    State(state): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> Response {
    if let Some(ref mode) = q.mode {
        if kjxlkj_domain::search::SearchMode::from_str(mode).is_none() {
            return domain_error_response(
                kjxlkj_domain::DomainError::SearchModeInvalid(mode.clone()),
            );
        }
    }
    let domain_query = kjxlkj_domain::search::SearchQuery {
        q: q.q,
        workspace_id: q.workspace_id,
        project_id: q.project_id,
        limit: q.limit,
        mode: q.mode.as_deref().and_then(kjxlkj_domain::search::SearchMode::from_str),
    };
    match state.search_repo.search_notes(&domain_query) {
        Ok(results) => {
            (StatusCode::OK, Json(serde_json::to_value(&results).unwrap())).into_response()
        }
        Err(e) => domain_error_response(e),
    }
}
