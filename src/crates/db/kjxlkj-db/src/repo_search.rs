use sqlx::{PgPool, FromRow};
use uuid::Uuid;

#[derive(FromRow, serde::Serialize)]
pub struct SearchRow {
    pub note_id: Uuid,
    pub title: String,
    pub snippet: String,
    pub rank: f32,
}

/// Lexical search using tsvector.
pub async fn lexical_search(
    pool: &PgPool,
    workspace_id: Uuid,
    query: &str,
    limit: i64,
) -> Result<Vec<SearchRow>, sqlx::Error> {
    sqlx::query_as::<_, SearchRow>(
        r#"SELECT ns.id as note_id, ns.title,
                  ts_headline('english', np.markdown, plainto_tsquery($2),
                      'MaxWords=30, MinWords=10') as snippet,
                  ts_rank(si.tsv, plainto_tsquery($2)) as rank
           FROM note_search_index si
           JOIN note_streams ns ON si.note_id = ns.id
           JOIN note_projections np ON si.note_id = np.note_id
           WHERE ns.workspace_id = $1
             AND NOT ns.is_deleted
             AND si.tsv @@ plainto_tsquery($2)
           ORDER BY ts_rank(si.tsv, plainto_tsquery($2)) DESC
           LIMIT $3"#
    )
    .bind(workspace_id)
    .bind(query)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Upsert search index entry for a note.
pub async fn upsert_search_index(
    pool: &PgPool,
    note_id: Uuid,
    title: &str,
    markdown: &str,
) -> Result<(), sqlx::Error> {
    let combined = format!("{} {}", title, markdown);
    sqlx::query(
        "INSERT INTO note_search_index (note_id, tsv, updated_at)
         VALUES ($1, to_tsvector('english', $2), now())
         ON CONFLICT (note_id) DO UPDATE
         SET tsv = to_tsvector('english', $2), updated_at = now()"
    )
    .bind(note_id)
    .bind(&combined)
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove note from search index.
pub async fn remove_from_index(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM note_search_index WHERE note_id = $1")
        .bind(note_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(FromRow, serde::Serialize)]
pub struct BacklinkRow {
    pub source_note_id: Uuid,
    pub source_title: String,
}

/// Get backlinks for a note.
pub async fn get_backlinks(
    pool: &PgPool,
    target_note_id: Uuid,
) -> Result<Vec<BacklinkRow>, sqlx::Error> {
    sqlx::query_as::<_, BacklinkRow>(
        "SELECT bl.source_note_id, ns.title as source_title
         FROM backlinks bl
         JOIN note_streams ns ON bl.source_note_id = ns.id
         WHERE bl.target_note_id = $1
           AND NOT ns.is_deleted
         ORDER BY ns.updated_at DESC"
    )
    .bind(target_note_id)
    .fetch_all(pool)
    .await
}

/// Update backlinks for a note based on wiki-link parsing.
pub async fn update_backlinks(
    pool: &PgPool,
    source_note_id: Uuid,
    target_note_ids: &[Uuid],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Remove existing outgoing links
    sqlx::query("DELETE FROM backlinks WHERE source_note_id = $1")
        .bind(source_note_id)
        .execute(&mut *tx)
        .await?;

    // Insert new links
    for target_id in target_note_ids {
        sqlx::query(
            "INSERT INTO backlinks (source_note_id, target_note_id)
             VALUES ($1, $2)
             ON CONFLICT DO NOTHING"
        )
        .bind(source_note_id)
        .bind(target_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
