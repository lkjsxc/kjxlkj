//! Route definitions and server startup

use crate::config::Config;
use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::{
    admin, assets, discoverability, favorites, health, history, home, login, logout, note,
    popular_sections, preview, record_history, records, search, settings, setup,
};
use actix_web::{web, App, HttpServer};
use tracing::info;

pub async fn run_server(config: Config) -> Result<(), AppError> {
    let pool = db::create_pool(&config.database_url).await?;
    info!("Database connected and migrations applied");

    let bind_addr = config.bind_addr();
    let pool = web::Data::new(pool);

    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(health::healthz)
            .service(setup::setup_page)
            .service(setup::setup_submit)
            .service(login::login_page)
            .service(login::login_submit)
            .service(logout::logout)
            .service(assets::favicon)
            .service(assets::icon_svg)
            .service(discoverability::robots_txt)
            .service(discoverability::sitemap_xml)
            .service(home::home_page)
            .service(admin::admin_page)
            .service(admin::admin_page_slash)
            .service(popular_sections::popular_notes_section)
            .service(settings::settings_page)
            .service(settings::settings_submit)
            .service(preview::render_markdown_preview)
            .service(search::search_page)
            .service(history::history_page)
            .service(records::create)
            .service(records::update)
            .service(records::remove)
            .service(favorites::reorder)
            .service(record_history::history)
            .service(record_history::previous)
            .service(record_history::next)
            .service(note::note_page)
    })
    .bind(&bind_addr)
    .map_err(|e| AppError::StorageError(format!("Failed to bind: {e}")))?
    .run()
    .await
    .map_err(|e| AppError::StorageError(format!("Server error: {e}")))?;

    Ok(())
}
