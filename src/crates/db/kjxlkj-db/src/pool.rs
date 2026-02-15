use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Initialize PostgreSQL connection pool per
/// /docs/spec/architecture/configuration.md.
/// Pool sizing follows /docs/spec/technical/performance.md targets.
pub async fn init_pool(
    database_url: &str,
    app_name: &str,
    max_conn: u32,
    min_conn: u32,
    connect_timeout_ms: u64,
    idle_timeout_ms: u64,
) -> Result<PgPool, sqlx::Error> {
    init_pool_with_statement_timeout(
        database_url,
        app_name,
        max_conn,
        min_conn,
        connect_timeout_ms,
        idle_timeout_ms,
        0,
    )
    .await
}

/// Extended pool init with per-connection statement_timeout.
/// timeout_ms=0 disables the statement timeout (PostgreSQL default).
pub async fn init_pool_with_statement_timeout(
    database_url: &str,
    app_name: &str,
    max_conn: u32,
    min_conn: u32,
    connect_timeout_ms: u64,
    idle_timeout_ms: u64,
    statement_timeout_ms: u64,
) -> Result<PgPool, sqlx::Error> {
    let app_name_owned = app_name.to_owned();
    PgPoolOptions::new()
        .max_connections(max_conn)
        .min_connections(min_conn)
        .acquire_timeout(Duration::from_millis(connect_timeout_ms))
        .idle_timeout(Duration::from_millis(idle_timeout_ms))
        .after_connect(move |conn, _meta| {
            let name = app_name_owned.clone();
            let stmt_timeout = statement_timeout_ms;
            Box::pin(async move {
                sqlx::query(&format!("SET application_name = '{}'", name))
                    .execute(&mut *conn)
                    .await?;
                if stmt_timeout > 0 {
                    sqlx::query(&format!(
                        "SET statement_timeout = '{}'",
                        stmt_timeout
                    ))
                    .execute(&mut *conn)
                    .await?;
                }
                Ok(())
            })
        })
        .connect(database_url)
        .await
}

/// Run embedded SQLx migrations at startup per
/// /docs/spec/architecture/runtime.md step 5.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("src/migrations").run(pool).await
}
