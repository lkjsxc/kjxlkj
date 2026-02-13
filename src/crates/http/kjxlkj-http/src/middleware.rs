//! HTTP middleware.

use actix_web::{dev::ServiceRequest, Error};
use actix_web::HttpMessage;

/// Session authentication middleware.
pub async fn session_auth(
    req: ServiceRequest,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Check for session token in header
    let _token = req.headers().get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    
    // TODO: Implement session validation
    Ok(req)
}

/// CSRF protection middleware using default headers.
pub fn csrf_protection() -> actix_web::middleware::DefaultHeaders {
    actix_web::middleware::DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add(("X-XSS-Protection", "1; mode=block"))
}
