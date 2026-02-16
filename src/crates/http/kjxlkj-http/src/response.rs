use actix_web::HttpResponse;
use kjxlkj_domain::error::ErrorCode;
use serde::Serialize;

/// Standard API error envelope.
#[derive(Serialize)]
pub struct ErrorEnvelope {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

/// Build an error response.
pub fn error_response(
    code: ErrorCode,
    message: &str,
) -> HttpResponse {
    let status = actix_web::http::StatusCode::from_u16(code.http_status())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

    HttpResponse::build(status).json(ErrorEnvelope {
        code: code.as_str().to_string(),
        message: message.to_string(),
        details: None,
        request_id: super::extract::request_id(),
    })
}

/// Build an error response with details.
pub fn error_response_with_details(
    code: ErrorCode,
    message: &str,
    details: serde_json::Value,
) -> HttpResponse {
    let status = actix_web::http::StatusCode::from_u16(code.http_status())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

    HttpResponse::build(status).json(ErrorEnvelope {
        code: code.as_str().to_string(),
        message: message.to_string(),
        details: Some(details),
        request_id: super::extract::request_id(),
    })
}
