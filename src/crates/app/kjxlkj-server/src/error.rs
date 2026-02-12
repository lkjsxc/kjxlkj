use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("auth required")]
    AuthRequired,
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("version conflict: expected {expected_version}, current {current_version}")]
    VersionConflict {
        expected_version: i64,
        current_version: i64,
    },
    #[error("payload too large")]
    PayloadTooLarge,
    #[error("rate limited")]
    RateLimited,
    #[error("internal error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorEnvelope<'a> {
    code: &'a str,
    message: String,
    details: Option<serde_json::Value>,
    request_id: &'a str,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::AuthRequired => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::VersionConflict { .. } => StatusCode::CONFLICT,
            Self::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message, details) = match self {
            Self::BadRequest(msg) => ("BAD_REQUEST", msg.clone(), None),
            Self::AuthRequired => ("AUTH_REQUIRED", self.to_string(), None),
            Self::Forbidden(msg) => ("FORBIDDEN", msg.clone(), None),
            Self::NotFound(msg) => ("NOT_FOUND", msg.clone(), None),
            Self::Conflict(msg) => ("VERSION_CONFLICT", msg.clone(), None),
            Self::VersionConflict {
                expected_version,
                current_version,
            } => (
                "VERSION_CONFLICT",
                self.to_string(),
                Some(serde_json::json!({
                    "expected_version": expected_version,
                    "current_version": current_version,
                })),
            ),
            Self::PayloadTooLarge => ("ATTACHMENT_TOO_LARGE", self.to_string(), None),
            Self::RateLimited => ("RATE_LIMITED", self.to_string(), None),
            Self::Internal => ("INTERNAL_ERROR", self.to_string(), None),
        };
        let request_id = Uuid::now_v7().to_string();
        HttpResponse::build(self.status_code()).json(ErrorEnvelope {
            code,
            message,
            details,
            request_id: &request_id,
        })
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        Self::Internal
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        Self::Internal
    }
}
