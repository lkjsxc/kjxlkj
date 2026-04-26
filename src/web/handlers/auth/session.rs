//! Session helpers for handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use axum::http::{header, HeaderMap, Uri};
use url::form_urlencoded::byte_serialize;
use uuid::Uuid;

pub async fn session_user(headers: &HeaderMap, pool: &DbPool) -> Result<Option<Uuid>, AppError> {
    let session_id = match cookie_value(headers, "session_id") {
        Some(value) => value,
        None => return Ok(None),
    };
    let session_id = match Uuid::parse_str(session_id) {
        Ok(id) => id,
        Err(_) => return Ok(None),
    };
    db::validate_session(pool, session_id).await
}

pub async fn check_session(headers: &HeaderMap, pool: &DbPool) -> Result<bool, AppError> {
    Ok(session_user(headers, pool).await?.is_some())
}

pub async fn require_session(headers: &HeaderMap, pool: &DbPool) -> Result<Uuid, AppError> {
    session_user(headers, pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Session required".to_string()))
}

pub fn login_url(uri: &Uri) -> String {
    format!("/login?return_to={}", encode(&return_path(uri)))
}

pub fn login_url_for_path(path: &str) -> String {
    format!("/login?return_to={}", encode(path))
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

fn return_path(uri: &Uri) -> String {
    uri.path_and_query()
        .map(|value| value.as_str().to_string())
        .unwrap_or_else(|| "/".to_string())
}

fn encode(value: &str) -> String {
    byte_serialize(value.as_bytes()).collect()
}

pub fn cookie_value<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|part| part.trim().split_once('='))
        .find_map(|(key, value)| (key == name).then_some(value))
}
