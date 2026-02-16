use sqlx::PgPool;
use tracing::info;

/// Run all database migrations. Must be called at startup.
/// Migrations are embedded at compile time from the migrations/ directory.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("running database migrations");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("migrations complete");
    Ok(())
}
