/// Error response envelope per /docs/spec/api/errors.md
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use kjxlkj_domain::DomainError;
use serde::Serialize;
use uuid::Uuid;

/// Standard error envelope per /docs/spec/api/errors.md
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

impl ErrorResponse {
    pub fn from_domain_error(err: &DomainError, request_id: &str) -> Self {
        Self {
            code: err.code().to_string(),
            message: err.to_string(),
            details: match err {
                DomainError::VersionConflict { expected, actual } => {
                    Some(serde_json::json!({
                        "expected_version": expected,
                        "current_version": actual,
                    }))
                }
                _ => None,
            },
            request_id: request_id.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let body = serde_json::to_string(&self).unwrap_or_default();
        (status, body).into_response()
    }
}

/// Convert DomainError into axum response
pub fn domain_error_response(err: DomainError) -> Response {
    let request_id = Uuid::new_v4().to_string();
    let status = StatusCode::from_u16(err.status_code())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let body = ErrorResponse::from_domain_error(&err, &request_id);
    let json = serde_json::to_string(&body).unwrap_or_default();
    (status, json).into_response()
}
