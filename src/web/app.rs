use actix_web::{web, App, HttpServer};

use crate::config::AppConfig;
use crate::error::AppError;
use crate::storage::FsStore;

use super::{configure_routes, AppState};

pub async fn run_http_server(config: AppConfig) -> Result<(), AppError> {
    let store = FsStore::new(config.data_root.clone());
    store.ensure_ready().await?;
    let state = web::Data::new(AppState {
        admin_token: config.admin_token,
        store,
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
