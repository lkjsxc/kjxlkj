//! Database migrations

use super::DbPool;
use crate::error::AppError;

const MIGRATIONS_SQL: &[&str] = &[
    include_str!("migrations/001_reset.sql"),
    include_str!("migrations/010_auth.sql"),
    include_str!("migrations/020_spaces.sql"),
    include_str!("migrations/030_resources.sql"),
    include_str!("migrations/040_api_and_indexes.sql"),
];

pub async fn run_migrations(pool: &DbPool) -> Result<(), AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Connection failed: {e}")))?
        .batch_execute(&MIGRATIONS_SQL.join("\n"))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Migration failed: {e}")))
}
