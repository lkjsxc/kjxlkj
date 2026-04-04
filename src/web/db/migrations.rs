//! Database migrations

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

    Ok(())
}
