use kjxlkj_domain::ids::WorkspaceId;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

/// FTS search result row.
#[derive(FromRow)]
pub struct SearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub rank: f32,
}

/// Full-text search over note projections per /docs/spec/domain/search.md.
pub async fn search_notes(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    query: &str,
) -> Result<Vec<SearchResult>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SearchResult>(
        "SELECT np.note_id, np.title,
                ts_rank(np.search_vector,
                        plainto_tsquery('english', $2)) as rank
         FROM note_projections np
         JOIN note_streams ns ON ns.id = np.note_id
         WHERE ns.workspace_id = $1
           AND ns.deleted_at IS NULL
           AND np.search_vector @@ plainto_tsquery('english', $2)
         ORDER BY rank DESC
         LIMIT 50",
    )
    .bind(workspace_id.0)
    .bind(query)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
