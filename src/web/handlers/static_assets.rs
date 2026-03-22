use actix_web::{http::header, HttpResponse};

use crate::web::handlers::app_css::APP_CSS;

const ADMIN_RUNTIME_CORE_JS: &str = include_str!("../static/admin_runtime_core.js");
const ADMIN_RUNTIME_AUTOSAVE_JS: &str = include_str!("../static/admin_runtime_autosave.js");
const ADMIN_RUNTIME_SHORTCUTS_JS: &str = include_str!("../static/admin_runtime_shortcuts.js");
const APP_SHELL_JS: &str = include_str!("../static/app_shell.js");

pub async fn handle_get_admin_runtime_core_js() -> HttpResponse {
    js_response(ADMIN_RUNTIME_CORE_JS)
}

pub async fn handle_get_admin_runtime_autosave_js() -> HttpResponse {
    js_response(ADMIN_RUNTIME_AUTOSAVE_JS)
}

pub async fn handle_get_admin_runtime_shortcuts_js() -> HttpResponse {
    js_response(ADMIN_RUNTIME_SHORTCUTS_JS)
}

pub async fn handle_get_app_shell_js() -> HttpResponse {
    js_response(APP_SHELL_JS)
}

pub async fn handle_get_app_css() -> HttpResponse {
    HttpResponse::Ok()
        .append_header((header::CACHE_CONTROL, "no-store"))
        .content_type("text/css; charset=utf-8")
        .body(APP_CSS)
}

fn js_response(body: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .append_header((header::CACHE_CONTROL, "no-store"))
        .content_type("application/javascript; charset=utf-8")
        .body(body)
}
