// Auth middleware per /docs/spec/security/csrf.md, sessions.md
use actix_web::{dev, web, Error, FromRequest, HttpRequest, HttpResponse};
use kjxlkj_db::repo::{sessions, users};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::types::{Role, User};
use serde::Serialize;
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

/// Authenticated session info injected into handlers.
#[derive(Debug, Clone)]
pub struct AuthSession {
    pub user: User,
    pub session_id: Uuid,
    pub csrf_token: String,
}

/// Error response envelope per /docs/spec/api/errors.md
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

/// Build error response from DomainError.
pub fn error_response(err: &DomainError) -> HttpResponse {
    let request_id = Uuid::now_v7().to_string();
    let body = ErrorResponse {
        code: err.code().to_string(),
        message: err.to_string(),
        details: None,
        request_id,
    };
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(err.status_code())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .json(body)
}

/// Extract session token from cookie.
pub fn extract_session_token(req: &HttpRequest) -> Option<String> {
    req.cookie("session")
        .map(|c| c.value().to_string())
}

/// Validate CSRF token from header against session CSRF.
pub fn validate_csrf(req: &HttpRequest, expected: &str) -> bool {
    req.headers()
        .get("X-CSRF-Token")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == expected)
        .unwrap_or(false)
}

fn auth_error(code: &str, msg: &str) -> Error {
    let body = ErrorResponse {
        code: code.into(),
        message: msg.into(),
        details: None,
        request_id: Uuid::now_v7().to_string(),
    };
    actix_web::error::InternalError::from_response(
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, msg),
        HttpResponse::Unauthorized().json(body),
    )
    .into()
}

fn forbidden_error(code: &str, msg: &str) -> Error {
    let body = ErrorResponse {
        code: code.into(),
        message: msg.into(),
        details: None,
        request_id: Uuid::now_v7().to_string(),
    };
    actix_web::error::InternalError::from_response(
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, msg),
        HttpResponse::Forbidden().json(body),
    )
    .into()
}

/// Actix FromRequest extractor: validates session cookie + CSRF for
/// mutating methods per /docs/spec/security/csrf.md
impl FromRequest for AuthSession {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let pool = req
                .app_data::<web::Data<PgPool>>()
                .ok_or_else(|| auth_error("INTERNAL_ERROR", "Missing pool"))?;

            let token = extract_session_token(&req)
                .ok_or_else(|| auth_error("AUTH_REQUIRED", "No session cookie"))?;

            let (session_id, user_id, csrf) =
                sessions::find_by_token(pool.get_ref(), &token)
                    .await
                    .map_err(|_| auth_error("INTERNAL_ERROR", "DB error"))?
                    .ok_or_else(|| auth_error("AUTH_REQUIRED", "Invalid or expired session"))?;

            let user = users::find_by_id(pool.get_ref(), user_id)
                .await
                .map_err(|_| auth_error("INTERNAL_ERROR", "DB error"))?
                .ok_or_else(|| auth_error("AUTH_REQUIRED", "User not found"))?;

            // CSRF enforcement per /docs/spec/security/csrf.md
            // State-changing methods MUST include valid CSRF token
            let method = req.method().as_str();
            if matches!(method, "POST" | "PUT" | "PATCH" | "DELETE") {
                if !validate_csrf(&req, &csrf) {
                    return Err(forbidden_error("CSRF_INVALID", "Invalid CSRF token"));
                }
            }

            Ok(AuthSession {
                user,
                session_id,
                csrf_token: csrf,
            })
        })
    }
}

/// Role guard: ensures the authenticated user has the required minimum role.
pub fn require_role(auth: &AuthSession, min_role: Role) -> Result<(), Error> {
    let level = role_level(auth.user.role);
    let required = role_level(min_role);
    if level >= required {
        Ok(())
    } else {
        Err(forbidden_error("FORBIDDEN", "Insufficient permissions"))
    }
}

fn role_level(r: Role) -> u8 {
    match r {
        Role::Viewer => 0,
        Role::Editor => 1,
        Role::Admin => 2,
        Role::Owner => 3,
    }
}
