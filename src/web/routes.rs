//! Route definitions and server startup

use crate::config::Config;
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db;
use crate::web::handlers::{
    admin, assets, discoverability, favorites, health, history, home, login, logout, media,
    media_attachments, password_reset, popular_sections, preview, resource, resource_file,
    resource_history, resources, search, settings, setup, site_icon,
};
use actix_web::{middleware::Compress, web, App, HttpServer};
use tracing::{info, warn};

pub async fn run_server(config: Config) -> Result<(), AppError> {
    let pool = db::create_pool(&config.database_url).await?;
    let storage = Storage::from_config(&config).await?;
    let setup_code = setup::SetupCode::new(config.setup_code.clone());
    if !db::is_setup(&pool).await? {
        warn!(
            setup_code = %setup_code.reveal(),
            "initial setup code issued; enter this once on /setup"
        );
    }
    info!("Database connected and migrations applied");

    let bind_addr = config.bind_addr();
    let pool = web::Data::new(pool);
    let storage = web::Data::new(storage);
    let setup_code = web::Data::new(setup_code);

    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(pool.clone())
            .app_data(storage.clone())
            .app_data(setup_code.clone())
            .service(health::healthz)
            .service(setup::setup_page)
            .service(setup::setup_submit)
            .service(login::login_page)
            .service(login::login_submit)
            .service(password_reset::reset_page)
            .service(password_reset::reset_request)
            .service(password_reset::reset_submit)
            .service(logout::logout)
            .service(assets::favicon)
            .service(assets::icon_svg)
            .service(assets::site_icon)
            .service(discoverability::robots_txt)
            .service(discoverability::sitemap_xml)
            .service(home::home_page)
            .service(admin::admin_page)
            .service(admin::admin_page_slash)
            .service(popular_sections::popular_resources_section)
            .service(settings::settings_page)
            .service(settings::settings_submit)
            .service(settings::password_submit)
            .service(site_icon::upload)
            .service(preview::render_markdown_preview)
            .service(search::search_page)
            .service(resource_file::current_file)
            .service(history::history_page)
            .service(media::create)
            .service(media_attachments::attach_media)
            .service(resources::create)
            .service(resources::update)
            .service(resources::remove)
            .service(favorites::reorder)
            .service(resource_history::history)
            .service(resource_history::previous)
            .service(resource_history::next)
            .service(resource::resource_page)
    })
    .bind(&bind_addr)
    .map_err(|e| AppError::StorageError(format!("Failed to bind: {e}")))?
    .run()
    .await
    .map_err(|e| AppError::StorageError(format!("Server error: {e}")))?;

    Ok(())
}
