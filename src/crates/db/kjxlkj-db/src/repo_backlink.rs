use kjxlkj_domain::ids::NoteId;
use sqlx::PgPool;
use uuid::Uuid;

/// Sync backlinks for a source note based on extracted wiki-link targets.
/// Per /docs/spec/domain/search.md: wiki links parsed from markdown
/// MUST update backlink projection table.
pub async fn sync_backlinks(
    pool: &PgPool,
    source_note_id: NoteId,
    target_titles: &[String],
) -> Result<(), sqlx::Error> {
    // Delete existing backlinks from this source
    sqlx::query("DELETE FROM backlinks WHERE source_note_id = $1")
        .bind(source_note_id.0)
        .execute(pool)
        .await?;

    // For each target title, find note streams by title and insert
    for title in target_titles {
        // Look up note by exact title in same workspace
        let target_ids: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT ns.id FROM note_streams ns
             WHERE ns.title = $1
               AND ns.deleted_at IS NULL
               AND ns.id != $2
             LIMIT 10",
        )
        .bind(title)
        .bind(source_note_id.0)
        .fetch_all(pool)
        .await?;

        for (target_id,) in target_ids {
            sqlx::query(
                "INSERT INTO backlinks (source_note_id, target_note_id)
                 VALUES ($1, $2)
                 ON CONFLICT DO NOTHING",
            )
            .bind(source_note_id.0)
            .bind(target_id)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}
