pub mod handlers;
mod password;
mod render;
pub mod router;
mod session;
pub mod state;
mod stores;

use actix_web::{web, App, HttpServer};

use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::web::state::WebState;

pub async fn run_http_server(config: AppConfig) -> Result<(), AppError> {
    let bind_addr = config.bind_addr;
    let app_state = AppState::new(config)?;
    let web_state = WebState::from_app_state(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(web_state.clone()))
            .configure(router::configure_routes)
    })
    .bind(bind_addr)
    .map_err(|source| AppError::ServerBind {
        addr: bind_addr.to_string(),
        source,
    })?
    .run()
    .await
    .map_err(AppError::ServerRun)
}
