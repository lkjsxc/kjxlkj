//! Session helpers for handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use actix_web::HttpRequest;
use uuid::Uuid;

pub async fn check_session(req: &HttpRequest, pool: &DbPool) -> Result<bool, AppError> {
    let cookie = match req.cookie("session_id") {
        Some(cookie) => cookie,
        None => return Ok(false),
    };
    let session_id = match Uuid::parse_str(cookie.value()) {
        Ok(id) => id,
        Err(_) => return Ok(false),
    };
    Ok(db::validate_session(pool, session_id).await?.is_some())
}

pub async fn require_session(req: &HttpRequest, pool: &DbPool) -> Result<(), AppError> {
    if check_session(req, pool).await? {
        Ok(())
    } else {
        Err(AppError::Unauthorized("Session required".to_string()))
    }
}
