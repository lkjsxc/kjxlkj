//! kjxlkj-search: Full-text search and backlink logic.
//! Per /docs/spec/domain/search.md.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Search result row.
#[derive(Debug, FromRow)]
pub struct SearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub note_kind: String,
    pub workspace_id: Uuid,
    pub rank: f32,
}

/// Full-text search per /docs/spec/domain/search.md.
pub async fn search_notes(
    pool: &PgPool,
    ws_id: Uuid,
    query: &str,
) -> Result<Vec<SearchResult>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SearchResult>(
        "SELECT np.note_id, np.title, np.note_kind, \
         np.workspace_id, \
         ts_rank(np.search_vector, plainto_tsquery('english', $1)) as rank \
         FROM note_projections np \
         JOIN note_streams ns ON np.note_id = ns.id \
         WHERE np.workspace_id = $2 \
           AND ns.deleted_at IS NULL \
           AND np.search_vector @@ plainto_tsquery('english', $1) \
         ORDER BY rank DESC \
         LIMIT 100"
    ).bind(query).bind(ws_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
