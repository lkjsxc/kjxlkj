//! Static asset handlers

use actix_web::{get, HttpResponse};

const CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");
const ICON_SVG: &str = include_str!("../assets/icon.svg");

#[get("/assets/favicon.ico")]
pub async fn favicon_ico() -> HttpResponse {
    asset_bytes("image/x-icon", FAVICON_ICO)
}

#[get("/assets/icon.svg")]
pub async fn icon_svg() -> HttpResponse {
    asset_text("image/svg+xml; charset=utf-8", ICON_SVG)
}

fn asset_text(content_type: &str, body: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", content_type))
        .append_header(("Cache-Control", CACHE_CONTROL))
        .body(body)
}

fn asset_bytes(content_type: &str, body: &'static [u8]) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", content_type))
        .append_header(("Cache-Control", CACHE_CONTROL))
        .body(body)
}
