//! Route definitions and server startup

use crate::config::Config;
use crate::error::AppError;
use crate::storage::FilesystemStorage;
use crate::web::db;
use crate::web::handlers::{admin, health, login, logout, records, setup};
use actix_web::{web, App, HttpServer};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

/// Run the HTTP server
pub async fn run_server(config: Config) -> Result<(), AppError> {
    let pool = db::create_pool(&config.database_url).await?;
    info!("Database connected and migrations applied");

    let storage = FilesystemStorage::new(PathBuf::from(&config.data_root)).await?;
    let storage = Arc::new(storage);
    info!("Storage initialized at {}", config.data_root);

    let bind_addr = config.bind_addr();
    let config = web::Data::new(config);
    let pool = web::Data::new(pool);
    let storage = web::Data::new(storage.clone());

    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(pool.clone())
            .app_data(storage.clone())
            .service(health::healthz)
            .service(setup::setup_page)
            .service(setup::setup_submit)
            .service(login::login_page)
            .service(login::login_submit)
            .service(logout::logout)
            .service(admin::home)
            .service(admin::admin_page)
            .service(admin::admin_page_slash)
            .service(records::list)
            .service(records::fetch)
            .service(records::upsert)
            .service(records::remove)
    })
    .bind(&bind_addr)
    .map_err(|e| AppError::StorageError(format!("Failed to bind: {}", e)))?
    .run()
    .await
    .map_err(|e| AppError::StorageError(format!("Server error: {}", e)))?;

    Ok(())
}
