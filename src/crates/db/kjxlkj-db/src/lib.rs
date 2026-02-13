pub mod models;
pub mod repos;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Create connection pool and run migrations.
pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("database migrations applied");
    Ok(pool)
}
