use actix_web::{http::header, HttpResponse};

use crate::web::handlers::app_css::APP_CSS;

const APP_SHELL_JS: &str = include_str!("../static/app_shell.js");

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
