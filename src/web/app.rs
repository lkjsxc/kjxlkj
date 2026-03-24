use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

use crate::config::AppConfig;
use crate::core::auth::{normalize_session_timeout_minutes, DEFAULT_SESSION_TIMEOUT_MINUTES};
use crate::error::AppError;
use crate::storage::FsStore;

use super::auth_store::AuthStore;
use super::{configure_routes, AppState};

pub async fn run_http_server(config: AppConfig) -> Result<(), AppError> {
    let store = FsStore::new(config.data_root.clone());
    store.ensure_ready().await?;
    let auth_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&config.database_url)
        .await?;
    let auth_store = AuthStore::new(auth_pool);
    auth_store.ensure_ready().await?;
    let state = web::Data::new(AppState {
        admin_token: config.admin_token,
        store,
        auth_store,
        session_timeout_minutes: normalize_session_timeout_minutes(
            config
                .session_timeout_minutes
                .unwrap_or(DEFAULT_SESSION_TIMEOUT_MINUTES),
        ),
    });
    let bind_host = config.bind_host.clone();
    let bind_port = config.bind_port;
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind((bind_host, bind_port))
    .map_err(AppError::Io)?
    .run()
    .await
    .map_err(AppError::Io)
}
