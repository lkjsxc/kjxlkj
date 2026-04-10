//! Session helpers for handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use actix_web::HttpRequest;
use url::form_urlencoded::byte_serialize;
use uuid::Uuid;

pub async fn session_user(req: &HttpRequest, pool: &DbPool) -> Result<Option<Uuid>, AppError> {
    let cookie = match req.cookie("session_id") {
        Some(cookie) => cookie,
        None => return Ok(None),
    };
    let session_id = match Uuid::parse_str(cookie.value()) {
        Ok(id) => id,
        Err(_) => return Ok(None),
    };
    db::validate_session(pool, session_id).await
}

pub async fn check_session(req: &HttpRequest, pool: &DbPool) -> Result<bool, AppError> {
    Ok(session_user(req, pool).await?.is_some())
}

pub async fn require_session(req: &HttpRequest, pool: &DbPool) -> Result<Uuid, AppError> {
    session_user(req, pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Session required".to_string()))
}

pub fn login_url(req: &HttpRequest) -> String {
    format!("/login?return_to={}", encode(&return_path(req)))
}

pub fn valid_return_to(value: Option<&str>) -> String {
    value
        .filter(|value| value.starts_with('/') && !value.starts_with("//"))
        .filter(|value| !value.contains('\\'))
        .filter(|value| !invalid_return_path(value))
        .unwrap_or("/admin")
        .to_string()
}

fn invalid_return_path(value: &str) -> bool {
    matches!(
        value.split('?').next().unwrap_or(value),
        "/login" | "/logout" | "/setup" | "/reset-password" | "/healthz"
    ) || value.starts_with("/resources/")
        || value.starts_with("/admin/markdown-preview")
}

fn return_path(req: &HttpRequest) -> String {
    req.uri()
        .path_and_query()
        .map(|value| value.as_str().to_string())
        .unwrap_or_else(|| "/".to_string())
}

fn encode(value: &str) -> String {
    byte_serialize(value.as_bytes()).collect()
}
