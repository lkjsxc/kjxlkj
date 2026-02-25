//! Search handlers

use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use kjxlkj_domain::{SearchQuery, SearchMode, SearchSort};
use crate::state::{HttpResult, HttpError};
use crate::routes::HttpState;

/// Search query params
#[derive(Debug, Deserialize)]
pub struct SearchQueryParams {
    q: String,
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    mode: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    note_kind: Option<String>,
    sort: Option<String>,
}

/// Search response
pub async fn search(
    State(state): State<HttpState>,
    Query(params): Query<SearchQueryParams>,
) -> HttpResult<Json<kjxlkj_domain::SearchResponse>> {
    let mode = match params.mode.as_deref() {
        Some("lexical") => SearchMode::Lexical,
        Some("semantic") => SearchMode::Semantic,
        _ => SearchMode::Hybrid,
    };

    let sort = match params.sort.as_deref() {
        Some("updated_at") => SearchSort::UpdatedAt,
        _ => SearchSort::Relevance,
    };

    let query = SearchQuery {
        q: params.q,
        workspace_id: params.workspace_id,
        project_id: params.project_id,
        mode,
        limit: params.limit.unwrap_or(20),
        offset: params.offset.unwrap_or(0),
        note_kind: params.note_kind,
        sort,
    };

    // Stub implementation - return empty results
    Ok(Json(kjxlkj_domain::SearchResponse {
        results: vec![],
        total: 0,
        mode: format!("{:?}", mode).to_lowercase(),
        degraded: false,
        degraded_reason: None,
        query_normalized: query.q.to_lowercase(),
        query_expanded: vec![query.q.to_lowercase()],
        timing_ms: kjxlkj_domain::SearchTiming::default(),
    }))
}
