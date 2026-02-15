use kjxlkj_server::config::AppConfig;
use kjxlkj_server::startup;

/// Application entrypoint per /docs/spec/architecture/runtime.md.
///
/// Startup sequence:
/// 1. Load .env (secrets)
/// 2. Load and validate non-secret runtime config from data/config.json
/// 3. Initialize tracing and error handling
/// 4. Initialize PostgreSQL pool
/// 5. Run pending SQL migrations
/// 6. Start Actix server with HTTP + WS routes
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Step 1: load .env secrets
    dotenvy::dotenv().ok();

    // Step 2: load and validate config
    let config_path = std::env::var("KJXLKJ_CONFIG_PATH")
        .unwrap_or_else(|_| "data/config.json".to_string());

    let config = AppConfig::load(&config_path)?;

    // Steps 3-6 handled in startup::run
    startup::run(config).await
}
