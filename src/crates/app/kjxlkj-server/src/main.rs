//! kjxlkj-server: Application entrypoint per /docs/spec/architecture/runtime.md.
//! Startup sequence:
//! 1. load .env
//! 2. load data/config.json
//! 3. init tracing
//! 4. init PG pool
//! 5. run migrations
//! 6. start Actix server with HTTP + WS routes
//! 7. start background workers

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load .env
    let _ = dotenvy::dotenv();

    // 2. Load config
    let config_path = std::env::var("KJXLKJ_CONFIG_PATH")
        .unwrap_or_else(|_| "data/config.json".to_string());
    let config = kjxlkj_db::config::AppConfig::load(&config_path)
        .map_err(|e| anyhow::anyhow!(e))?;

    // 3. Init tracing
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new(&config.logging.default_level)
        });
    if config.logging.json {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .init();
    }

    // 4. Validate secrets and init PG pool
    let db_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL not set in .env"))?;
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_millis(
            config.database.connect_timeout_ms,
        ))
        .idle_timeout(Duration::from_millis(config.database.idle_timeout_ms))
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET application_name = 'kjxlkj'")
                    .execute(&mut *conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&db_url)
        .await?;

    // 5. Run migrations
    tracing::info!("Running migrations...");
    kjxlkj_db::migrate::run_migrations(&pool).await?;
    tracing::info!("Migrations complete");

    // 6. Start server
    let bind_addr = config.server.bind_addr.clone();
    let static_dir = config.server.static_dir.clone();
    let config_data = web::Data::new(config);
    let pool_data = web::Data::new(pool.clone());

    tracing::info!("Starting server on {bind_addr}");

    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(pool_data.clone())
            .app_data(config_data.clone())
            .configure(kjxlkj_http::configure)
            // WebSocket upgrade route
            .route("/ws", web::get().to(ws_upgrade));
        // Serve SPA static files
        app = app.service(
            actix_files::Files::new("/", &static_dir)
                .index_file("index.html")
                .default_handler(
                    web::to(spa_fallback),
                ),
        );
        app
    })
    .bind(&bind_addr)?
    .run()
    .await?;

    // 7. Shutdown: drain and close
    tracing::info!("Shutting down");
    pool.close().await;
    Ok(())
}

/// WebSocket upgrade handler.
async fn ws_upgrade(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<sqlx::PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Verify session before upgrade per /docs/spec/security/csrf.md
    let ctx = kjxlkj_http::middleware::extract_session(&req, pool.get_ref())
        .await;
    let ctx = match ctx {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };
    let session = kjxlkj_ws::actor::WsSession::new(
        ctx.user_id,
        pool.get_ref().clone(),
    );
    actix_web_actors::ws::start(session, &req, stream)
}

/// SPA fallback: serve index.html for client-side routing.
async fn spa_fallback() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/index.html")?)
}
