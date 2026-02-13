use actix_web::HttpRequest;
use kjxlkj_auth::csrf;
use kjxlkj_domain::errors::DomainError;

const CSRF_HEADER: &str = "X-CSRF-Token";

/// Validate the CSRF token on mutating requests.
pub fn validate_csrf(
    req: &HttpRequest,
    session_id: &str,
    csrf_secret: &str,
) -> Result<(), DomainError> {
    // GET and HEAD are exempt per csrf.md.
    let method = req.method();
    if method == actix_web::http::Method::GET || method == actix_web::http::Method::HEAD {
        return Ok(());
    }
    let token = req
        .headers()
        .get(CSRF_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or(DomainError::BadRequest {
            reason: "missing CSRF token".into(),
        })?;
    if !csrf::validate_csrf_token(token, session_id, csrf_secret) {
        return Err(DomainError::BadRequest {
            reason: "invalid CSRF token".into(),
        });
    }
    Ok(())
}
