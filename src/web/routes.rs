//! Route definitions and server startup

use crate::config::Config;
use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::{admin, assets, health, history, login, logout, records, search, setup};
use actix_web::{web, App, HttpServer};
use tracing::info;

/// Run the HTTP server
pub async fn run_server(config: Config) -> Result<(), AppError> {
    let pool = db::create_pool(&config.database_url).await?;
    info!("Database connected and migrations applied");

    let bind_addr = config.bind_addr();
    let config = web::Data::new(config);
    let pool = web::Data::new(pool);

    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(pool.clone())
            .service(health::healthz)
            .service(setup::setup_page)
            .service(setup::setup_submit)
            .service(login::login_page)
            .service(login::login_submit)
            .service(logout::logout)
            .service(assets::toastui_css)
            .service(assets::toastui_dark_css)
            .service(assets::toastui_js)
            .service(admin::admin_page)
            .service(admin::admin_page_slash)
            .service(search::search_page)
            .service(history::history_page)
            .service(history::revision_page)
            .service(records::create)
            .service(records::update)
            .service(records::remove)
            .service(records::history)
            .service(records::previous)
            .service(records::next)
            .service(admin::home)
            .service(admin::note_page)
    })
    .bind(&bind_addr)
    .map_err(|e| AppError::StorageError(format!("Failed to bind: {e}")))?
    .run()
    .await
    .map_err(|e| AppError::StorageError(format!("Server error: {e}")))?;

    Ok(())
}
