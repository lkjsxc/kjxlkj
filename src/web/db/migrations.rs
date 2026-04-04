//! Database migrations

use super::resource_ids;
use super::DbPool;
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
    backfill_revision_ids(&client).await?;
    ensure_revision_primary_key(&client).await?;
    drop_legacy_revision_id(&client).await?;

    Ok(())
}

async fn backfill_revision_ids(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    if !has_legacy_revision_id(client).await? {
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

async fn ensure_revision_primary_key(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    client
        .batch_execute("ALTER TABLE record_revisions ALTER COLUMN id SET NOT NULL")
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    if has_revision_primary_key(client).await? {
        return Ok(());
    }
    client
        .batch_execute(
            "ALTER TABLE record_revisions ADD CONSTRAINT record_revisions_pkey PRIMARY KEY (id)",
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn drop_legacy_revision_id(client: &deadpool_postgres::Object) -> Result<(), AppError> {
    if !has_legacy_revision_id(client).await? {
        return Ok(());
    }
    client
        .batch_execute("ALTER TABLE record_revisions DROP COLUMN legacy_id")
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn has_legacy_revision_id(client: &deadpool_postgres::Object) -> Result<bool, AppError> {
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

async fn has_revision_primary_key(client: &deadpool_postgres::Object) -> Result<bool, AppError> {
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
