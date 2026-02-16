use sqlx::{PgPool, FromRow};
use kjxlkj_db::repo_search;
use uuid::Uuid;
use tracing::{info, warn};

#[derive(FromRow)]
struct ReindexRow {
    id: Uuid,
    title: String,
    markdown: String,
}

/// Reindex all notes in the search index. Resumable and idempotent.
pub async fn reindex_all(pool: &PgPool, batch_size: i64) -> Result<u64, sqlx::Error> {
    let mut offset: i64 = 0;
    let mut total: u64 = 0;

    loop {
        let rows = sqlx::query_as::<_, ReindexRow>(
            "SELECT ns.id, ns.title, np.markdown
             FROM note_streams ns
             JOIN note_projections np ON ns.id = np.note_id
             WHERE NOT ns.is_deleted
             ORDER BY ns.id
             OFFSET $1 LIMIT $2"
        )
        .bind(offset)
        .bind(batch_size)
        .fetch_all(pool)
        .await?;

        if rows.is_empty() {
            break;
        }

        for row in &rows {
            if let Err(e) = repo_search::upsert_search_index(
                pool,
                row.id,
                &row.title,
                &row.markdown,
            )
            .await
            {
                warn!("reindex failed for note {}: {e}", row.id);
            } else {
                total += 1;
            }
        }

        offset += batch_size;
        info!("reindexed batch, total so far: {total}");
    }

    info!("reindex complete: {total} notes indexed");
    Ok(total)
}
