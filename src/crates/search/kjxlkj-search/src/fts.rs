use kjxlkj_db::models::search::SearchResultRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

/// Execute a full-text search scoped to a workspace.
pub async fn search_notes(
    pool: &PgPool,
    workspace_id: Uuid,
    query: &str,
) -> Result<Vec<SearchResultRow>, DomainError> {
    if query.trim().is_empty() {
        return Err(DomainError::BadRequest {
            reason: "search query must not be empty".into(),
        });
    }
    repos::search::full_text_search(pool, workspace_id, query)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}
