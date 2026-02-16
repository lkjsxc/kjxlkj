use actix_web::{web, App, HttpServer};
use actix_files::Files;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::path::Path;

mod config_loader;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Load .env
    let _ = dotenvy::dotenv();

    // 2. Load config
    let config: serde_json::Value = config_loader::load_config(
        Path::new("./data/config.json"),
    ).expect("failed to load data/config.json");

    // 3. Initialize tracing
    let log_level = config["logging"]["default_level"]
        .as_str()
        .unwrap_or("info")
        .to_string();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&log_level))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    info!("starting kjxlkj server");

    // 4. Load agent prompt
    let prompt_path_str = config["agent"]["prompt_path"]
        .as_str()
        .unwrap_or("./data/agent-prompt.json")
        .to_string();
    let prompt_path = Path::new(&prompt_path_str);

    let agent_prompt = kjxlkj_automation::prompt::load_prompt(prompt_path)
        .expect("failed to load agent prompt");
    let prompt_hash = kjxlkj_automation::prompt::prompt_hash(prompt_path)
        .unwrap_or_else(|_| "unknown".to_string());

    info!(
        "agent prompt loaded: name={}, version={}, hash={}",
        agent_prompt.agent_name, agent_prompt.version, prompt_hash
    );

    // 5. Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://kjxlkj:kjxlkj@localhost:5432/kjxlkj".to_string());
    let max_conn = config["database"]["max_connections"].as_u64().unwrap_or(20) as u32;
    let min_conn = config["database"]["min_connections"].as_u64().unwrap_or(2) as u32;
    let connect_timeout = config["database"]["connect_timeout_ms"].as_u64().unwrap_or(5000);
    let idle_timeout = config["database"]["idle_timeout_ms"].as_u64().unwrap_or(30000);

    let pool = kjxlkj_db::pool::create_pool(
        &database_url,
        max_conn,
        min_conn,
        connect_timeout,
        idle_timeout,
    )
    .await
    .expect("failed to connect to database");

    // 6. Run migrations
    kjxlkj_db::migrate::run_migrations(&pool)
        .await
        .expect("migrations failed");

    // 7. Build shared state — extract owned strings before moving into closure
    let bind_addr = config["server"]["bind_addr"]
        .as_str()
        .unwrap_or("0.0.0.0:8080")
        .to_string();
    let static_dir = config["server"]["static_dir"]
        .as_str()
        .unwrap_or("./static")
        .to_string();

    let app_state = web::Data::new(kjxlkj_http::config::AppState {
        pool: pool.clone(),
        config: kjxlkj_http::config::AppConfig {
            search_embedding_base_url: config["automation"]["base_url"]
                .as_str()
                .unwrap_or("http://127.0.0.1:1234/v1")
                .to_string(),
            search_embedding_model: config["search"]["embedding_model"]
                .as_str()
                .unwrap_or("text-embedding-nomic-embed-text-v1.5")
                .to_string(),
            search_semantic_enabled: config["search"]["semantic_enabled"]
                .as_bool()
                .unwrap_or(true),
            agent_prompt_hash: prompt_hash.clone(),
        },
    });

    // 8. Start HTTP server
    info!("binding to {bind_addr}");
    HttpServer::new(move || {
        let pool_data = web::Data::new(pool.clone());

        App::new()
            .app_data(pool_data.clone())
            .app_data(app_state.clone())
            // Health endpoints
            .route("/api/healthz", web::get().to(kjxlkj_http::routes_health::healthz))
            .route("/api/readyz", web::get().to(kjxlkj_http::routes_health::readyz))
            // Auth endpoints
            .route("/api/setup/register", web::post().to(kjxlkj_http::routes_auth::register))
            .route("/api/auth/login", web::post().to(kjxlkj_http::routes_auth::login))
            .route("/api/auth/logout", web::post().to(kjxlkj_http::routes_auth::logout))
            .route("/api/auth/session", web::get().to(kjxlkj_http::routes_auth::get_session))
            // Workspace endpoints
            .route("/api/workspaces", web::get().to(kjxlkj_http::routes_workspace::list_workspaces))
            .route("/api/workspaces", web::post().to(kjxlkj_http::routes_workspace::create_workspace))
            // Note endpoints
            .route("/api/notes", web::get().to(kjxlkj_http::routes_note::list_notes))
            .route("/api/notes", web::post().to(kjxlkj_http::routes_note::create_note))
            .route("/api/notes/{id}", web::get().to(kjxlkj_http::routes_note::get_note))
            .route("/api/notes/{id}", web::patch().to(kjxlkj_http::routes_note::patch_note))
            .route("/api/notes/{id}", web::delete().to(kjxlkj_http::routes_note::delete_note))
            .route("/api/notes/{id}/title", web::patch().to(kjxlkj_http::routes_note::update_title))
            .route("/api/notes/{id}/history", web::get().to(kjxlkj_http::routes_note::note_history))
            .route("/api/notes/{id}/backlinks", web::get().to(kjxlkj_http::routes_search::backlinks))
            // Search endpoint
            .route("/api/search", web::get().to(kjxlkj_http::routes_search::search))
            // Automation endpoints
            .route("/api/automation/rules", web::get().to(kjxlkj_http::routes_automation::list_rules))
            .route("/api/automation/rules", web::post().to(kjxlkj_http::routes_automation::create_rule))
            .route("/api/automation/rules/{id}/launch", web::post().to(kjxlkj_http::routes_automation::launch_run))
            .route("/api/automation/runs/{id}", web::get().to(kjxlkj_http::routes_automation::get_run))
            // WebSocket endpoint
            .route("/ws", web::get().to(kjxlkj_ws::handler::ws_handler))
            // Static files (frontend)
            .service(Files::new("/", &static_dir).index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
