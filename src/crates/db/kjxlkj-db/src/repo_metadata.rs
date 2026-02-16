use sqlx::PgPool;
use uuid::Uuid;

/// Upsert note metadata key.
pub async fn upsert_metadata(
    pool: &PgPool,
    note_id: Uuid,
    key: &str,
    value: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_metadata (note_id, key, value, updated_at)
         VALUES ($1, $2, $3, now())
         ON CONFLICT (note_id, key) DO UPDATE
         SET value = $3, updated_at = now()"
    )
    .bind(note_id)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete note metadata key.
pub async fn delete_metadata(
    pool: &PgPool,
    note_id: Uuid,
    key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM note_metadata WHERE note_id = $1 AND key = $2"
    )
    .bind(note_id)
    .bind(key)
    .execute(pool)
    .await?;
    Ok(())
}

/// Get all metadata for a note as a JSON object.
pub async fn get_metadata_map(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<serde_json::Value, sqlx::Error> {
    let rows: Vec<(String, serde_json::Value)> = sqlx::query_as(
        "SELECT key, value FROM note_metadata WHERE note_id = $1"
    )
    .bind(note_id)
    .fetch_all(pool)
    .await?;

    let mut map = serde_json::Map::new();
    for (key, value) in rows {
        map.insert(key, value);
    }
    Ok(serde_json::Value::Object(map))
}
