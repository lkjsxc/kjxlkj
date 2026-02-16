/// Search route handlers per /docs/spec/api/http.md
///
/// GET /api/search — hybrid search query
/// Integrates lexical + semantic search per /docs/spec/domain/search.md:
/// - If mode=hybrid: combine lexical + semantic, fallback to lexical on failure
/// - If mode=semantic: semantic only, degrade to empty on failure
/// - If mode=lexical: lexical only
/// - Semantic failure sets SearchEmbeddingDegraded diagnostics
use crate::error_response::domain_error_response;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use kjxlkj_db::repo::SearchRepo;
use kjxlkj_search::SearchService;
use serde::{Deserialize, Serialize};
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

/// Search response with optional degraded flag
#[derive(Serialize)]
struct SearchResponse {
    results: Vec<kjxlkj_domain::search::SearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    degraded: Option<String>,
}

/// GET /api/search
/// Per /docs/spec/domain/search.md: validate mode, run hybrid ranking
pub async fn search_notes(
    State(state): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> Response {
    let mode = q.mode.as_deref().unwrap_or("hybrid");
    if let Some(ref m) = q.mode {
        if kjxlkj_domain::search::SearchMode::from_str(m).is_none() {
            return domain_error_response(
                kjxlkj_domain::DomainError::SearchModeInvalid(m.clone()),
            );
        }
    }
    let domain_query = kjxlkj_domain::search::SearchQuery {
        q: q.q.clone(),
        workspace_id: q.workspace_id,
        project_id: q.project_id,
        limit: q.limit,
        mode: q.mode.as_deref().and_then(kjxlkj_domain::search::SearchMode::from_str),
    };
    let limit = q.limit.unwrap_or(20) as usize;
    let mut degraded: Option<String> = None;

    // Lexical results (always attempted unless mode=semantic)
    let lexical = if mode != "semantic" {
        match state.search_repo.search_notes(&domain_query) {
            Ok(r) => r,
            Err(e) => return domain_error_response(e),
        }
    } else {
        vec![]
    };

    // Semantic results (attempted for hybrid or semantic mode)
    let semantic = if mode == "hybrid" || mode == "semantic" {
        match SearchService::semantic_search(
            state.embedding_provider.as_ref(),
            &state.embedding_store,
            &q.q,
            limit,
        ) {
            Ok(scored) => scored
                .into_iter()
                .map(|(note_id, score)| kjxlkj_domain::search::SearchResult {
                    note_id,
                    title: String::new(),
                    snippet: String::new(),
                    score_lexical: 0.0,
                    score_semantic: score,
                    score_final: 0.0,
                })
                .collect(),
            Err(_) => {
                degraded = Some("SEARCH_EMBEDDING_DEGRADED".to_string());
                vec![]
            }
        }
    } else {
        vec![]
    };

    let results = SearchService::merge_and_rank(lexical, semantic);
    let body = SearchResponse { results, degraded };
    (StatusCode::OK, Json(body)).into_response()
}
