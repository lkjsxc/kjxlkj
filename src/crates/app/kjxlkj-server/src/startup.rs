use actix_files::Files;
use actix_web::{web, App, HttpServer};
use kjxlkj_db::pool;
use kjxlkj_http::{
    routes_attachments, routes_auth, routes_health, routes_metadata,
    routes_notes, routes_search, routes_users, routes_workspaces,
};
use kjxlkj_ws::route::{ws_connect, WsConfig};
use tracing::info;

use crate::config::AppConfig;

/// Build and run the server per /docs/spec/architecture/runtime.md.
pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    // Step 3: initialize tracing
    init_tracing(&config.logging.default_level, config.logging.json);

    // Step 1-2: secrets from .env, config already loaded
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("connecting to database");

    // Step 4: initialize PostgreSQL pool
    let db_pool = pool::init_pool(
        &database_url,
        &config.database.app_name,
        config.database.max_connections,
        config.database.min_connections,
        config.database.connect_timeout_ms,
        config.database.idle_timeout_ms,
    )
    .await?;

    // Step 5: run pending migrations
    info!("running migrations");
    pool::run_migrations(&db_pool).await?;

    let ws_config = WsConfig {
        heartbeat_interval_ms: config.websocket.heartbeat_interval_ms,
        client_timeout_ms: config.websocket.client_timeout_ms,
        replay_batch_size: config.websocket.replay_batch_size,
    };

    let bind_addr = config.server.bind_addr.clone();
    let static_dir = config.server.static_dir.clone();
    let max_body = config.server.max_request_body_mb * 1024 * 1024;

    info!(bind = %bind_addr, "starting server");

    // Step 6: start Actix server with HTTP + WS routes
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(ws_config.clone()))
            .app_data(web::JsonConfig::default().limit(max_body))
            // Health routes
            .route("/api/healthz", web::get().to(routes_health::healthz))
            .route("/api/readyz", web::get().to(routes_health::readyz))
            // Auth routes
            .route(
                "/setup/register",
                web::post().to(routes_auth::setup_register),
            )
            .route("/auth/login", web::post().to(routes_auth::login))
            .route("/auth/logout", web::post().to(routes_auth::logout))
            .route("/auth/session", web::get().to(routes_auth::get_session))
            // User routes
            .route("/api/users", web::get().to(routes_users::list_users))
            .route("/api/users", web::post().to(routes_users::create_user))
            .route(
                "/api/users/{id}/role",
                web::patch().to(routes_users::update_role),
            )
            .route(
                "/api/users/{id}",
                web::delete().to(routes_users::delete_user),
            )
            // Workspace routes
            .route(
                "/api/workspaces",
                web::get().to(routes_workspaces::list_workspaces),
            )
            .route(
                "/api/workspaces",
                web::post().to(routes_workspaces::create_workspace),
            )
            .route(
                "/api/workspaces/{id}",
                web::patch().to(routes_workspaces::update_workspace),
            )
            .route(
                "/api/workspaces/{id}",
                web::delete().to(routes_workspaces::delete_workspace),
            )
            .route(
                "/api/workspaces/{id}/members",
                web::get().to(routes_workspaces::list_members),
            )
            .route(
                "/api/workspaces/{id}/members/{user_id}",
                web::put().to(routes_workspaces::upsert_member),
            )
            // Project routes
            .route(
                "/api/projects",
                web::get().to(routes_workspaces::list_projects),
            )
            .route(
                "/api/projects",
                web::post().to(routes_workspaces::create_project),
            )
            // Note routes
            .route("/api/notes", web::post().to(routes_notes::create_note))
            .route("/api/notes", web::get().to(routes_notes::list_notes))
            .route(
                "/api/notes/{id}",
                web::get().to(routes_notes::get_note),
            )
            .route(
                "/api/notes/{id}",
                web::patch().to(routes_notes::patch_note),
            )
            .route(
                "/api/notes/{id}/title",
                web::patch().to(routes_notes::patch_title),
            )
            .route(
                "/api/notes/{id}",
                web::delete().to(routes_notes::delete_note),
            )
            .route(
                "/api/notes/{id}/history",
                web::get().to(routes_notes::note_history),
            )
            .route(
                "/api/notes/{id}/rollback",
                web::post().to(routes_notes::rollback_note),
            )
            // Metadata & tags
            .route(
                "/api/notes/{id}/metadata/{key}",
                web::put().to(routes_metadata::upsert_metadata),
            )
            .route(
                "/api/notes/{id}/metadata/{key}",
                web::delete().to(routes_metadata::delete_metadata),
            )
            .route(
                "/api/notes/{id}/tags",
                web::put().to(routes_metadata::replace_tags),
            )
            .route(
                "/api/tags",
                web::get().to(routes_metadata::list_tags),
            )
            // Search & backlinks
            .route(
                "/api/search",
                web::get().to(routes_search::search),
            )
            .route(
                "/api/notes/{id}/backlinks",
                web::get().to(routes_search::get_backlinks),
            )
            // Attachments
            .route(
                "/api/notes/{id}/attachments",
                web::post().to(routes_attachments::upload_attachment),
            )
            .route(
                "/api/attachments/{id}",
                web::get().to(routes_attachments::download_attachment),
            )
            .route(
                "/api/attachments/{id}",
                web::delete().to(routes_attachments::delete_attachment),
            )
            // WebSocket
            .route("/ws", web::get().to(ws_connect))
            // Static files (SPA)
            .service(Files::new("/", &static_dir).index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run()
    .await?;

    Ok(())
}

fn init_tracing(level: &str, json: bool) {
    use tracing_subscriber::{fmt, EnvFilter};
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));
    if json {
        fmt().with_env_filter(filter).json().init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}
