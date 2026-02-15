use actix_web::HttpResponse;
use kjxlkj_domain::error::DomainError;
use uuid::Uuid;

use crate::dto::ErrorResponse;

/// Convert DomainError to HTTP response per /docs/spec/api/errors.md.
pub fn domain_error_response(err: DomainError, request_id: &str) -> HttpResponse {
    let status = actix_web::http::StatusCode::from_u16(err.status_code())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

    let details = match &err {
        DomainError::VersionConflict { expected, found } => {
            Some(serde_json::json!({
                "expected_version": expected,
                "current_version": found
            }))
        }
        DomainError::StaleCursor {
            stream_id,
            attempted,
            current,
        } => Some(serde_json::json!({
            "stream_id": stream_id,
            "attempted_seq": attempted,
            "current_cursor": current
        })),
        _ => None,
    };

    let body = ErrorResponse {
        code: err.code().to_string(),
        message: err.to_string(),
        details,
        request_id: request_id.to_string(),
    };

    HttpResponse::build(status).json(body)
}

/// Generate a request ID for correlation.
pub fn new_request_id() -> String {
    Uuid::now_v7().to_string()
}
