use kjxlkj_domain::ids::NoteId;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

/// Backlink row.
#[derive(FromRow)]
pub struct BacklinkResult {
    pub source_note_id: Uuid,
    pub title: String,
}

/// Get backlinks for a note per /docs/spec/domain/search.md.
pub async fn get_backlinks(
    pool: &PgPool,
    target_note_id: NoteId,
) -> Result<Vec<BacklinkResult>, sqlx::Error> {
    let rows = sqlx::query_as::<_, BacklinkResult>(
        "SELECT b.source_note_id, np.title
         FROM backlinks b
         JOIN note_projections np ON np.note_id = b.source_note_id
         JOIN note_streams ns ON ns.id = b.source_note_id
         WHERE b.target_note_id = $1
           AND ns.deleted_at IS NULL
         ORDER BY np.title",
    )
    .bind(target_note_id.0)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
