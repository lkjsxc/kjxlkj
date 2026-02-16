use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// Build a PostgreSQL connection pool from configuration values.
pub async fn create_pool(
    database_url: &str,
    max_connections: u32,
    min_connections: u32,
    connect_timeout_ms: u64,
    idle_timeout_ms: u64,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_millis(connect_timeout_ms))
        .idle_timeout(Duration::from_millis(idle_timeout_ms))
        .connect(database_url)
        .await
}
