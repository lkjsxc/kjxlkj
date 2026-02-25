//! Connection pool management

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::DatabaseConfig;
use crate::error::Result;

/// In-memory database pool (for testing and docs-only baseline)
#[derive(Debug, Clone)]
pub struct DbPool {
    pub config: DatabaseConfig,
}

impl DbPool {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }

    pub async fn connect(&self) -> Result<DbConnection> {
        Ok(DbConnection::new())
    }
}

/// Database connection wrapper
#[derive(Debug, Clone)]
pub struct DbConnection;

impl DbConnection {
    pub fn new() -> Self {
        Self
    }
}

/// PostgreSQL pool (for production)
#[cfg(feature = "postgres")]
pub struct PgPool {
    inner: sqlx::Pool<sqlx::Postgres>,
}

#[cfg(feature = "postgres")]
impl PgPool {
    pub async fn new(config: &DatabaseConfig, database_url: &str) -> Result<Self> {
        let pool = sqlx::PgPool::connect(database_url)
            .await
            .map_err(|e| crate::error::DbError::ConnectionFailed(e.to_string()))?;
        Ok(Self { inner: pool })
    }

    pub fn inner(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.inner
    }
}
