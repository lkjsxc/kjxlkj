mod config;

use actix_web::{web, App, HttpServer};
use kjxlkj_ws::hub::WsHub;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

/// Startup sequence per runtime.md:
/// 1. load and validate configuration
/// 2. initialize tracing and error handling
/// 3. initialize PostgreSQL pool
/// 4. run pending SQL migrations
/// 5. start Actix server with HTTP + WS routes
/// 6-7. background workers deferred to later waves
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Load configuration.
    let cfg = config::AppConfig::from_env();

    // 2. Initialize tracing.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new("info")
        }))
        .init();
    tracing::info!("kjxlkj-server starting");

    // 3-4. Initialize DB pool and run migrations.
    let pool = kjxlkj_db::init_pool(&cfg.database_url)
        .await
        .expect("failed to initialize database pool");
    tracing::info!("database pool initialized");

    // WebSocket broadcast hub.
    let hub = Arc::new(WsHub::new());

    // 5. Start Actix HTTP server with routes and WS.
    let bind_addr = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!(bind = %bind_addr, "starting HTTP server");

    let static_dir = cfg.static_dir.clone();
    let csrf_secret = cfg.csrf_secret.clone();

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(hub.clone()))
            .app_data(web::Data::new(csrf_secret.clone()))
            .configure(kjxlkj_http::routes::configure)
            .route("/ws", web::get().to(kjxlkj_ws::handler::ws_handler));

        // Serve static frontend files from /static/ and / root.
        let sd = static_dir.clone();
        let app = if std::path::Path::new(&static_dir).exists() {
            app.service(
                actix_web::web::resource("/").to(move || {
                    let sd = sd.clone();
                    async move {
                        let index = format!("{sd}/index.html");
                        match std::fs::read_to_string(&index) {
                            Ok(html) => actix_web::HttpResponse::Ok()
                                .content_type("text/html; charset=utf-8")
                                .body(html),
                            Err(_) => actix_web::HttpResponse::NotFound().finish(),
                        }
                    }
                }),
            )
            .service(
                actix_web::web::scope("/static")
                    .service(actix_web::web::resource("/{filename:.*}").to(serve_static)),
            )
        } else {
            app
        };
        app
    })
    .bind(&bind_addr)?
    .run()
    .await
}

/// Simple static file handler (serves from compiled-in or filesystem).
async fn serve_static(
    path: actix_web::web::Path<String>,
) -> actix_web::HttpResponse {
    let filename = path.into_inner();
    let file_path = format!("static/{filename}");
    match std::fs::read_to_string(&file_path) {
        Ok(contents) => {
            let ct = if filename.ends_with(".js") {
                "application/javascript; charset=utf-8"
            } else if filename.ends_with(".css") {
                "text/css; charset=utf-8"
            } else {
                "text/plain; charset=utf-8"
            };
            actix_web::HttpResponse::Ok().content_type(ct).body(contents)
        }
        Err(_) => actix_web::HttpResponse::NotFound().finish(),
    }
}

