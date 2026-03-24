//! Health endpoint handler

use actix_web::{get, HttpResponse};

/// Health check endpoint
#[get("/healthz")]
pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().content_type("text/plain").body("ok")
}
