//! Static asset handlers

use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, DbPool};
use actix_web::{get, web, HttpResponse};
use tracing::warn;

const CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");
const ICON_SVG: &str = include_str!("../assets/icon.svg");

#[get("/favicon.ico")]
pub async fn favicon(
    pool: web::Data<DbPool>,
    storage: web::Data<Storage>,
) -> Result<HttpResponse, AppError> {
    Ok(match uploaded_icon(&pool, &storage).await? {
        Some(response) => response,
        None => bytes("image/x-icon", FAVICON_ICO),
    })
}

#[get("/assets/icon.svg")]
pub async fn icon_svg() -> HttpResponse {
    asset("image/svg+xml; charset=utf-8", ICON_SVG)
}

#[get("/assets/site-icon")]
pub async fn site_icon(
    pool: web::Data<DbPool>,
    storage: web::Data<Storage>,
) -> Result<HttpResponse, AppError> {
    Ok(match uploaded_icon(&pool, &storage).await? {
        Some(response) => response,
        None => asset_no_cache("image/svg+xml; charset=utf-8", ICON_SVG),
    })
}

async fn uploaded_icon(pool: &DbPool, storage: &Storage) -> Result<Option<HttpResponse>, AppError> {
    let settings = db::get_settings(pool).await?;
    let Some(key) = settings.site_icon_key.as_deref() else {
        return Ok(None);
    };
    match storage.get_object(key, None).await {
        Ok(object) => Ok(Some(
            HttpResponse::Ok()
                .append_header((
                    "Content-Type",
                    settings
                        .site_icon_content_type
                        .as_deref()
                        .unwrap_or("application/octet-stream"),
                ))
                .append_header(("Cache-Control", "no-cache"))
                .append_header(("Content-Length", object.body.len().to_string()))
                .body(object.body),
        )),
        Err(error) => {
            warn!(key = %key, error = %error, "uploaded site icon unavailable");
            Ok(None)
        }
    }
}

fn asset(content_type: &str, body: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", content_type))
        .append_header(("Cache-Control", CACHE_CONTROL))
        .body(body)
}

fn asset_no_cache(content_type: &str, body: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", content_type))
        .append_header(("Cache-Control", "no-cache"))
        .body(body)
}

fn bytes(content_type: &str, body: &'static [u8]) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", content_type))
        .append_header(("Cache-Control", CACHE_CONTROL))
        .body(body)
}
