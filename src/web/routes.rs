//! Route definitions and server startup

use crate::config::Config;
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db;
use crate::web::handlers::{
    admin, assets, discoverability, favorites, health, history, home, live, login, logout, media,
    media_attachments, password_reset, popular_sections, preview, resource, resource_api,
    resource_file, resource_history, resources, search, settings, setup, site_icon,
};
use crate::web::live::LiveHub;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post, put};
use axum::Router;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tracing::{info, warn};

#[derive(Clone)]
pub struct AppState {
    pub pool: db::DbPool,
    pub storage: Storage,
    pub setup_code: setup::SetupCode,
    pub live_hub: LiveHub,
    pub media_upload_max_bytes: usize,
    pub site_icon_upload_max_bytes: usize,
}

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
    let live_hub = LiveHub::new(&config.live_ice_addr(), config.live_ice_public_ips.clone())
        .await
        .map_err(AppError::StorageError)?;
    let state = AppState {
        pool,
        storage,
        setup_code,
        live_hub,
        media_upload_max_bytes: config.media_upload_max_bytes,
        site_icon_upload_max_bytes: config.site_icon_upload_max_bytes,
    };

    info!("Starting HTTP server on {}", bind_addr);

    let listener = TcpListener::bind(&bind_addr)
        .await
        .map_err(|e| AppError::StorageError(format!("Failed to bind: {e}")))?;
    axum::serve(listener, router(state))
        .await
        .map_err(|e| AppError::StorageError(format!("Server error: {e}")))?;

    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/setup", get(setup::setup_page).post(setup::setup_submit))
        .route("/login", get(login::login_page).post(login::login_submit))
        .route(
            "/reset-password",
            get(password_reset::reset_page).post(password_reset::reset_submit),
        )
        .route(
            "/reset-password/request",
            post(password_reset::reset_request),
        )
        .route("/logout", post(logout::logout))
        .route("/favicon.ico", get(assets::favicon))
        .route("/assets/icon.svg", get(assets::icon_svg))
        .route("/assets/site-icon", get(assets::site_icon))
        .route("/robots.txt", get(discoverability::robots_txt))
        .route("/sitemap.xml", get(discoverability::sitemap_xml))
        .route("/.well-known/nostr.json", get(discoverability::nostr_json))
        .route("/", get(home::home_page))
        .route("/admin", get(admin::admin_page))
        .route("/admin/", get(admin::admin_page))
        .route(
            "/_/popular-resources/{surface}/{window}",
            get(popular_sections::popular_resources_section),
        )
        .route(
            "/admin/settings",
            get(settings::settings_page).post(settings::settings_submit),
        )
        .route("/admin/password", post(settings::password_submit))
        .route(
            "/admin/site-icon",
            post(site_icon::upload).layer(DefaultBodyLimit::max(state.site_icon_upload_max_bytes)),
        )
        .route("/admin/site-icon/reset", post(site_icon::reset))
        .route(
            "/admin/markdown-preview",
            post(preview::render_markdown_preview),
        )
        .route("/search", get(search::search_page))
        .route("/api/resources/search", get(resource_api::search))
        .route(
            "/api/resources/preview-markdown",
            post(preview::render_markdown_preview),
        )
        .route("/live", get(live::live_page))
        .route("/live/ws", get(live::live_ws))
        .route("/{reference}/file", get(resource_file::current_file))
        .route("/{id}/history", get(history::history_page))
        .route(
            "/api/resources/media",
            post(media::create).layer(DefaultBodyLimit::max(state.media_upload_max_bytes)),
        )
        .route("/api/resources/notes", post(resources::create))
        .route(
            "/api/resources/{reference}/history",
            get(resource_history::api_history),
        )
        .route(
            "/api/resources/{reference}",
            get(resource_api::fetch).put(resources::api_update),
        )
        .route(
            "/resources/media",
            post(media::create).layer(DefaultBodyLimit::max(state.media_upload_max_bytes)),
        )
        .route(
            "/resources/{id}/media-attachments",
            post(media_attachments::attach_media)
                .layer(DefaultBodyLimit::max(state.media_upload_max_bytes)),
        )
        .route("/resources/notes", post(resources::create))
        .route(
            "/resources/{id}",
            put(resources::update).delete(resources::remove),
        )
        .route("/resources/favorites/order", put(favorites::reorder))
        .route("/resources/{id}/history", get(resource_history::history))
        .route("/resources/{id}/prev", get(resource_history::previous))
        .route("/resources/{id}/next", get(resource_history::next))
        .route("/{reference}", get(resource::resource_page))
        .layer(CompressionLayer::new())
        .with_state(state)
}
