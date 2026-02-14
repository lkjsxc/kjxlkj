use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: Cow<'static, str>,
    pub message: Cow<'static, str>,
    pub details: Option<Value>,
    pub request_id: String,
}

#[derive(Debug, Serialize)]
struct ErrorEnvelope {
    code: String,
    message: String,
    details: Option<Value>,
    request_id: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: &'static str) -> Self {
        Self {
            status,
            code: Cow::Borrowed(code),
            message: Cow::Borrowed(message),
            details: None,
            request_id: new_request_id(),
        }
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = request_id;
        self
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorEnvelope {
            code: self.code.to_string(),
            message: self.message.to_string(),
            details: self.details.clone(),
            request_id: self.request_id.clone(),
        };
        HttpResponse::build(self.status).json(body)
    }
}

impl Display for ApiError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

pub fn new_request_id() -> String {
    Uuid::now_v7().to_string()
}
