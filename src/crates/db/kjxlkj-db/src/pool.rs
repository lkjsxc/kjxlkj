//! Database pool and migration management.

use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
use thiserror::Error;

/// Database errors.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("pool error: {0}")]
    PoolError(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    MigrationError(String),
}

/// Database pool wrapper.
#[derive(Clone)]
pub struct DbPool {
    pool: SqlitePool,
}

impl DbPool {
    /// Create a new database pool.
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self, DbError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    /// Get a reference to the underlying pool.
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Run migrations.
    pub async fn run_migrations(&self) -> Result<(), DbError> {
        // raw_sql executes the full migration script (multiple statements).
        sqlx::raw_sql(include_str!("../migrations/001_initial.sql"))
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| DbError::MigrationError(e.to_string()))?;
        Ok(())
    }

    /// Check database connectivity.
    pub async fn is_ready(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }
}

/// Run migrations on a pool.
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), DbError> {
    sqlx::raw_sql(include_str!("../migrations/001_initial.sql"))
        .execute(pool)
        .await
        .map_err(|e: sqlx::Error| DbError::MigrationError(e.to_string()))?;
    Ok(())
}
