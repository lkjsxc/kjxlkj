//! Database migrations

use super::resource_ids;
use super::DbPool;
use crate::core::{derive_summary, derive_title};
use crate::error::AppError;
const MIGRATIONS_SQL: &str = include_str!("migrations.sql");

pub async fn run_migrations(pool: &DbPool) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Connection failed: {e}")))?;
    client
        .batch_execute(MIGRATIONS_SQL)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Migration failed: {e}")))?;
    backfill_snapshot_ids(&client).await?;
    backfill_snapshot_metadata(&client).await?;
    backfill_current_snapshots(&client).await?;
    ensure_snapshot_columns(&client).await?;
    ensure_snapshot_primary_key(&client).await?;
    drop_legacy_snapshot_id(&client).await
}
async fn backfill_snapshot_ids(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    if !has_legacy_snapshot_id(client).await? {
        return Ok(());
    }
    let rows = client
        .query(
            "SELECT legacy_id FROM record_revisions WHERE id IS NULL ORDER BY legacy_id ASC",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    for row in rows {
        let legacy_id: i32 = row.get("legacy_id");
        let id = resource_ids::next_resource_id(client).await?;
        client
            .execute(
                "UPDATE record_revisions SET id = $1 WHERE legacy_id = $2",
                &[&id, &legacy_id],
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }
    Ok(())
}
async fn backfill_snapshot_metadata(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    let rows = client
        .query(
            "SELECT rr.id, rr.body, r.alias \
             FROM record_revisions rr \
             JOIN records r ON r.id = rr.record_id \
             WHERE rr.title IS NULL OR rr.summary IS NULL OR rr.alias IS NULL \
             ORDER BY rr.created_at ASC, rr.id ASC",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    for row in rows {
        let id: String = row.get("id");
        let body: String = row.get("body");
        let alias: Option<String> = row.get("alias");
        let title = derive_title(&body);
        let summary = derive_summary(&body);
        client
            .execute(
                "UPDATE record_revisions \
                 SET alias = COALESCE(alias, $2), title = COALESCE(title, $3), summary = COALESCE(summary, $4) \
                 WHERE id = $1",
                &[&id, &alias, &title, &summary],
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }
    Ok(())
}
async fn backfill_current_snapshots(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    let rows = client
        .query(
            "SELECT r.id, r.alias, r.title, r.summary, r.body, r.is_private, \
             COALESCE(s.snapshot_number, 0) AS latest_number, s.alias AS snapshot_alias, \
             s.title AS snapshot_title, s.summary AS snapshot_summary, s.body AS snapshot_body, \
             s.is_private AS snapshot_is_private \
             FROM records r \
             LEFT JOIN LATERAL (
                 SELECT snapshot_number, alias, title, summary, body, is_private
                 FROM record_revisions
                 WHERE record_id = r.id
                 ORDER BY snapshot_number DESC
                 LIMIT 1
             ) s ON TRUE \
             WHERE r.deleted_at IS NULL ORDER BY r.created_at ASC, r.id ASC",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    for row in rows {
        let latest_number: i32 = row.get("latest_number");
        let alias: Option<String> = row.get("alias");
        let title: String = row.get("title");
        let summary: String = row.get("summary");
        let body: String = row.get("body");
        let is_private: bool = row.get("is_private");
        let latest_alias: Option<String> = row.get("snapshot_alias");
        let latest_title: Option<String> = row.get("snapshot_title");
        let latest_summary: Option<String> = row.get("snapshot_summary");
        let latest_body: Option<String> = row.get("snapshot_body");
        let latest_private: Option<bool> = row.get("snapshot_is_private");
        if latest_number > 0
            && latest_alias == alias
            && latest_title.as_deref() == Some(title.as_str())
            && latest_summary.as_deref() == Some(summary.as_str())
            && latest_body.as_deref() == Some(body.as_str())
            && latest_private == Some(is_private)
        {
            continue;
        }
        let snapshot_id = resource_ids::next_resource_id(client).await?;
        let record_id: String = row.get("id");
        client
            .execute(
                "INSERT INTO record_revisions \
                 (id, record_id, snapshot_number, alias, title, summary, body, is_private) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[
                    &snapshot_id,
                    &record_id,
                    &(latest_number + 1),
                    &alias,
                    &title,
                    &summary,
                    &body,
                    &is_private,
                ],
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }
    Ok(())
}
async fn ensure_snapshot_columns(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    client
        .batch_execute(
            "ALTER TABLE record_revisions ALTER COLUMN id SET NOT NULL;
             ALTER TABLE record_revisions ALTER COLUMN snapshot_number SET NOT NULL;
             ALTER TABLE record_revisions ALTER COLUMN title SET NOT NULL;
             ALTER TABLE record_revisions ALTER COLUMN summary SET NOT NULL",
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
async fn ensure_snapshot_primary_key(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    if has_snapshot_primary_key(client).await? {
        return Ok(());
    }
    client
        .batch_execute(
            "ALTER TABLE record_revisions ADD CONSTRAINT record_revisions_pkey PRIMARY KEY (id)",
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
async fn drop_legacy_snapshot_id(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    if !has_legacy_snapshot_id(client).await? {
        return Ok(());
    }
    client
        .batch_execute("ALTER TABLE record_revisions DROP COLUMN legacy_id")
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
async fn has_legacy_snapshot_id(client: &deadpool_postgres::Object) -> Result<bool, AppError> {
    client
        .query_one(
            "SELECT EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'record_revisions' AND column_name = 'legacy_id'
            )",
            &[],
        )
        .await
        .map(|row| row.get(0))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
async fn has_snapshot_primary_key(client: &deadpool_postgres::Object) -> Result<bool, AppError> {
    client
        .query_one(
            "SELECT EXISTS (
                SELECT 1 FROM information_schema.table_constraints
                WHERE table_name = 'record_revisions'
                  AND constraint_type = 'PRIMARY KEY'
            )",
            &[],
        )
        .await
        .map(|row| row.get(0))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
