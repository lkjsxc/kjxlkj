// kjxlkj-server — single-container entry point
// Per /docs/spec/architecture/runtime.md and /docs/spec/architecture/deployment.md
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .json()
        .init();

    tracing::info!("kjxlkj-server starting");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/kjxlkj".into());

    // Create connection pool
    let pool = PgPool::connect(&database_url).await?;

    // Run migrations (const-embedded SQL)
    kjxlkj_db::migrate::run_migrations(&pool).await?;
    tracing::info!("migrations applied");

    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    tracing::info!(bind = %bind, "starting HTTP server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(kjxlkj_http::routes::configure)
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}
