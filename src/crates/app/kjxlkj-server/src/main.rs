//! kjxlkj-server application
//! 
//! Main entry point for the kjxlkj platform.

mod config;

use anyhow::Result;
use axum::{
    Router,
    routing::get,
    serve,
};
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use kjxlkj_db::{DbPool, DatabaseConfig, NoteRepo, WorkspaceRepo};
use kjxlkj_auth::SessionStore;
use kjxlkj_http::{create_router as create_http_router, HttpState};
use kjxlkj_ws::{ws_handler, WsState};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,kjxlkj=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting kjxlkj-server...");

    // Load configuration
    let config = config::load_config()?;
    info!("Configuration loaded");

    // Initialize database pool
    let db_pool = DbPool::new(DatabaseConfig::default());
    info!("Database pool initialized");

    // Initialize repositories
    let note_repo = NoteRepo::new();
    let workspace_repo = WorkspaceRepo::new();
    info!("Repositories initialized");

    // Initialize session store
    let session_store = SessionStore::new();
    info!("Session store initialized");

    // Initialize WebSocket state
    let ws_state = WsState::new();
    info!("WebSocket state initialized");

    // Create HTTP state
    let http_state = HttpState {
        db_pool: db_pool.clone(),
        session_store: session_store.clone(),
    };

    // Create routers
    let api_router = create_http_router(http_state);
    
    // WebSocket route
    let ws_router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(ws_state);

    // Static file serving
    let static_router = tower_http::services::ServeDir::new(&config.server.static_dir)
        .not_found_service(axum::routing::get_serve_dir_index_file(
            &config.server.static_dir,
            axum::http::StatusCode::NOT_FOUND,
        ));

    // Combine all routers
    let app = Router::new()
        .merge(api_router)
        .merge(ws_router)
        .fallback_service(static_router);

    // Bind address
    let addr: SocketAddr = config.server.bind_addr.parse()?;
    info!("Listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}
