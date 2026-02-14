//! Kjxlkj server entrypoint.
//!
//! This is the main application that wires together all crates.

use actix_web::{web, App, HttpServer};
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use kjxlkj_db::DbPool;
use kjxlkj_http::handlers::configure_routes;

/// Application configuration.
pub struct AppConfig {
    pub database_url: String,
    pub bind_address: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl AppConfig {
    /// Load configuration from environment.
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:kjxlkj.db?mode=rwc".to_string());
        let bind_address = env::var("BIND_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev-secret-change-in-production".to_string());

        Self {
            database_url,
            bind_address,
            port,
            jwt_secret,
        }
    }
}

/// Application state.
pub struct AppState {
    pub pool: DbPool,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set tracing subscriber");

    // Load configuration
    let config = AppConfig::from_env();
    info!("Starting kjxlkj server on {}:{}", config.bind_address, config.port);

    // Connect to database
    let pool = DbPool::new(&config.database_url, 5)
        .await
        .expect("failed to connect to database");

    // Run migrations
    pool.run_migrations()
        .await
        .expect("failed to run migrations");
    info!("Database migrations completed");

    // Create app state
    let state = web::Data::new(AppState {
        pool,
        jwt_secret: config.jwt_secret,
    });
    let db_pool = web::Data::new(state.pool.clone());

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(db_pool.clone())
            .configure(configure_routes)
    })
    .bind((config.bind_address.as_str(), config.port))?
    .run()
    .await
}
