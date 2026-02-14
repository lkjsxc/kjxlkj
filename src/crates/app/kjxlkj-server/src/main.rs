// kjxlkj-server — single-container entry point
// Per /docs/spec/architecture/runtime.md and /docs/spec/architecture/deployment.md
use actix_web::{web, App, HttpServer};
use actix_files::Files;
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

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./static".into());
    tracing::info!(static_dir = %static_dir, "serving SPA from static dir");

    // Shared WebSocket session manager
    let ws_mgr = kjxlkj_ws::session_mgr::SessionManager::new();

    HttpServer::new(move || {
        let static_dir = static_dir.clone();
        App::new()
            .wrap(kjxlkj_auth::headers::SecurityHeaders)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(ws_mgr.clone()))
            .configure(kjxlkj_http::routes::configure)
            // WebSocket endpoint
            .route("/ws", web::get().to(kjxlkj_ws::handler::ws_handler))
            // SPA: serve static assets, fallback to index.html for client routing
            .service(
                Files::new("/", &static_dir)
                    .index_file("index.html")
                    .default_handler(
                        actix_web::dev::fn_service(move |req: actix_web::dev::ServiceRequest| {
                            let static_dir = static_dir.clone();
                            async move {
                                let (req, _) = req.into_parts();
                                let index = std::path::Path::new(&static_dir).join("index.html");
                                let file = actix_files::NamedFile::open(index)?;
                                let resp = file.into_response(&req);
                                Ok(actix_web::dev::ServiceResponse::new(req, resp))
                            }
                        })
                    )
            )
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}
