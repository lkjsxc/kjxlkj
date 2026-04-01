//! Vendored asset handlers

use actix_web::{get, HttpResponse};

const CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
const ICON_SVG: &str = include_str!("../assets/icon.svg");
const TOAST_UI_CSS: &str = include_str!("../assets/vendor/toastui/3.2.2/toastui-editor.min.css");
const TOAST_UI_DARK_CSS: &str =
    include_str!("../assets/vendor/toastui/3.2.2/toastui-editor-dark.min.css");
const TOAST_UI_JS: &str = include_str!("../assets/vendor/toastui/3.2.2/toastui-editor-all.min.js");

#[get("/assets/icon.svg")]
pub async fn icon_svg() -> HttpResponse {
    asset("image/svg+xml; charset=utf-8", ICON_SVG)
}

#[get("/assets/vendor/toastui/3.2.2/toastui-editor.min.css")]
pub async fn toastui_css() -> HttpResponse {
    asset("text/css; charset=utf-8", TOAST_UI_CSS)
}

#[get("/assets/vendor/toastui/3.2.2/toastui-editor-dark.min.css")]
pub async fn toastui_dark_css() -> HttpResponse {
    asset("text/css; charset=utf-8", TOAST_UI_DARK_CSS)
}

#[get("/assets/vendor/toastui/3.2.2/toastui-editor-all.min.js")]
pub async fn toastui_js() -> HttpResponse {
    asset("text/javascript; charset=utf-8", TOAST_UI_JS)
}

fn asset(content_type: &str, body: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", content_type))
        .append_header(("Cache-Control", CACHE_CONTROL))
        .body(body)
}
