/// kjxlkj-server: Application entry point.
///
/// Startup sequence per /docs/spec/architecture/runtime.md:
/// 1. load .env
/// 2. load data/config.json
/// 3. load data/agent-prompt.json
/// 4. initialize DB pool and migrations
/// 5. start HTTP + WS services
/// 6. start background jobs
/// 7. start kjxlkj-agent loop
use kjxlkj_domain::config::AppConfig;
use kjxlkj_http::routes::api_router;
use kjxlkj_http::state::AppState;
use kjxlkj_ws::handler::ws_handler;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Step 1: load .env (if present)
    let _ = dotenvy_load();

    // Step 2: load data/config.json
    let config = AppConfig::load_from_file("data/config.json")
        .expect("failed to load config.json");

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&config.logging.default_level)),
        )
        .json()
        .init();

    tracing::info!("kjxlkj-server starting");

    // Step 3: validate agent prompt
    let _prompt = kjxlkj_automation::prompt::load_prompt(&config.agent.prompt_path)
        .expect("failed to load agent prompt");
    tracing::info!("agent prompt validated");

    // Step 4: create application state with in-memory repositories
    let state = AppState::new();
    tracing::info!("application state initialized");

    // Build router: HTTP + WS with shared state
    let app = api_router(state)
        .route("/ws", axum::routing::get(ws_handler));

    // Step 5: bind and serve
    let listener = tokio::net::TcpListener::bind(&config.server.bind_addr)
        .await
        .expect("failed to bind");
    tracing::info!(addr = %config.server.bind_addr, "listening");

    // IMP-OPS-03: graceful shutdown with in-flight drain
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    tracing::info!("server shut down gracefully");
}

/// Wait for a shutdown signal (SIGTERM or Ctrl-C).
/// Per /docs/spec/technical/operations.md (IMP-OPS-03):
/// graceful shutdown drains in-flight requests before exiting.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => tracing::info!("received Ctrl+C, starting graceful shutdown"),
        () = terminate => tracing::info!("received SIGTERM, starting graceful shutdown"),
    }
}

/// Load .env file if it exists (non-fatal)
fn dotenvy_load() {
    if let Ok(content) = std::fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                std::env::set_var(key.trim(), value.trim());
            }
        }
    }
}
