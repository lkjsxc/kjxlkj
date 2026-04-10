//! Static asset handlers

use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, DbPool};
use crate::web::handlers::http;
use crate::web::routes::AppState;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::Response;
use tracing::warn;

const CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");
const ICON_SVG: &str = include_str!("../assets/icon.svg");

pub async fn favicon(State(state): State<AppState>) -> Result<Response, AppError> {
    Ok(match uploaded_icon(&state.pool, &state.storage).await? {
        Some(response) => response,
        None => bytes("image/x-icon", FAVICON_ICO),
    })
}

pub async fn icon_svg() -> Response {
    asset("image/svg+xml; charset=utf-8", ICON_SVG)
}

pub async fn site_icon(State(state): State<AppState>) -> Result<Response, AppError> {
    Ok(match uploaded_icon(&state.pool, &state.storage).await? {
        Some(response) => response,
        None => asset_no_cache("image/svg+xml; charset=utf-8", ICON_SVG),
    })
}

async fn uploaded_icon(pool: &DbPool, storage: &Storage) -> Result<Option<Response>, AppError> {
    let settings = db::get_settings(pool).await?;
    let Some(key) = settings.site_icon_key.as_deref() else {
        return Ok(None);
    };
    match storage.get_object(key, None).await {
        Ok(object) => {
            let content_type = settings
                .site_icon_content_type
                .as_deref()
                .unwrap_or("application/octet-stream");
            let mut response = http::bytes_with_type(StatusCode::OK, content_type, object.body);
            http::set_header(&mut response, header::CACHE_CONTROL, "no-cache");
            Ok(Some(response))
        }
        Err(error) => {
            warn!(key = %key, error = %error, "uploaded site icon unavailable");
            Ok(None)
        }
    }
}

fn asset(content_type: &str, body: &'static str) -> Response {
    let mut response = http::text_with_type(StatusCode::OK, content_type, body.to_string());
    http::set_header(&mut response, header::CACHE_CONTROL, CACHE_CONTROL);
    response
}

fn asset_no_cache(content_type: &str, body: &'static str) -> Response {
    let mut response = http::text_with_type(StatusCode::OK, content_type, body.to_string());
    http::set_header(&mut response, header::CACHE_CONTROL, "no-cache");
    response
}

fn bytes(content_type: &str, body: &'static [u8]) -> Response {
    let mut response = http::bytes_with_type(StatusCode::OK, content_type, body.to_vec());
    http::set_header(&mut response, header::CACHE_CONTROL, CACHE_CONTROL);
    response
}
