use crate::models::search::SearchResultRow;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn full_text_search(
    pool: &PgPool,
    workspace_id: Uuid,
    query: &str,
) -> Result<Vec<SearchResultRow>, sqlx::Error> {
    sqlx::query_as::<_, SearchResultRow>(
        "SELECT n.id AS note_id, n.title,
         ts_headline('english', n.body, plainto_tsquery('english', $2)) AS snippet,
         ts_rank(n.search_vec, plainto_tsquery('english', $2)) AS rank
         FROM notes n
         WHERE n.workspace_id = $1 AND n.is_deleted = false
         AND n.search_vec @@ plainto_tsquery('english', $2)
         ORDER BY rank DESC LIMIT 50",
    )
    .bind(workspace_id)
    .bind(query)
    .fetch_all(pool)
    .await
}
