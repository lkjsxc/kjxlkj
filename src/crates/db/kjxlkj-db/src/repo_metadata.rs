use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use kjxlkj_domain::ids::NoteId;

/// Metadata row per /docs/spec/domain/metadata.md.
#[derive(FromRow)]
pub struct MetadataRow {
    pub note_id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: OffsetDateTime,
}

/// Upsert a metadata key-value pair.
pub async fn upsert_metadata(
    pool: &PgPool,
    note_id: NoteId,
    key: &str,
    value: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_metadata (note_id, key, value, updated_at)
         VALUES ($1, $2, $3, now())
         ON CONFLICT (note_id, key)
         DO UPDATE SET value = $3, updated_at = now()",
    )
    .bind(note_id.0)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a metadata key.
pub async fn delete_metadata(
    pool: &PgPool,
    note_id: NoteId,
    key: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM note_metadata WHERE note_id = $1 AND key = $2",
    )
    .bind(note_id.0)
    .bind(key)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// List all metadata for a note.
pub async fn list_metadata(
    pool: &PgPool,
    note_id: NoteId,
) -> Result<Vec<MetadataRow>, sqlx::Error> {
    sqlx::query_as::<_, MetadataRow>(
        "SELECT note_id, key, value, updated_at
         FROM note_metadata
         WHERE note_id = $1
         ORDER BY key",
    )
    .bind(note_id.0)
    .fetch_all(pool)
    .await
}

/// Build metadata JSON object from rows.
pub fn metadata_to_json(rows: &[MetadataRow]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for row in rows {
        map.insert(row.key.clone(), row.value.clone());
    }
    serde_json::Value::Object(map)
}
