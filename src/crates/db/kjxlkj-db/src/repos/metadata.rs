use crate::models::metadata::{BacklinkRow, MetadataRow};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn upsert_metadata(
    pool: &PgPool,
    note_id: Uuid,
    key: &str,
    value: serde_json::Value,
) -> Result<MetadataRow, sqlx::Error> {
    sqlx::query_as::<_, MetadataRow>(
        "INSERT INTO metadata (note_id, key, value)
         VALUES ($1, $2, $3)
         ON CONFLICT (note_id, key) DO UPDATE SET value = $3, updated_at = now()
         RETURNING note_id, key, value, updated_at",
    )
    .bind(note_id)
    .bind(key)
    .bind(&value)
    .fetch_one(pool)
    .await
}

pub async fn list_metadata(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Vec<MetadataRow>, sqlx::Error> {
    sqlx::query_as::<_, MetadataRow>(
        "SELECT note_id, key, value, updated_at FROM metadata WHERE note_id = $1",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
}

pub async fn delete_metadata(
    pool: &PgPool,
    note_id: Uuid,
    key: &str,
) -> Result<bool, sqlx::Error> {
    let res = sqlx::query("DELETE FROM metadata WHERE note_id = $1 AND key = $2")
        .bind(note_id)
        .bind(key)
        .execute(pool)
        .await?;
    Ok(res.rows_affected() > 0)
}

pub async fn upsert_backlinks(
    pool: &PgPool,
    source_id: Uuid,
    targets: &[Uuid],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM backlinks WHERE source_note_id = $1")
        .bind(source_id)
        .execute(&mut *tx)
        .await?;
    for target in targets {
        sqlx::query(
            "INSERT INTO backlinks (source_note_id, target_note_id)
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(source_id)
        .bind(target)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await
}

pub async fn list_backlinks(
    pool: &PgPool,
    target_id: Uuid,
) -> Result<Vec<BacklinkRow>, sqlx::Error> {
    sqlx::query_as::<_, BacklinkRow>(
        "SELECT source_note_id, target_note_id FROM backlinks WHERE target_note_id = $1",
    )
    .bind(target_id)
    .fetch_all(pool)
    .await
}
