use actix_web::HttpRequest;
use kjxlkj_auth::session;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{SessionId, UserId};
use sqlx::PgPool;
use uuid::Uuid;

/// Authenticated identity extracted from session cookie.
#[derive(Debug, Clone)]
pub struct AuthIdentity {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub csrf_token: String,
}

/// Session cookie name.
pub const SESSION_COOKIE: &str = "kjxlkj_session";

/// Extract session from request cookie and validate against DB.
/// Per /docs/spec/security/sessions.md.
pub async fn extract_session(
    req: &HttpRequest,
    pool: &PgPool,
) -> Result<AuthIdentity, DomainError> {
    let cookie = req
        .cookie(SESSION_COOKIE)
        .ok_or(DomainError::AuthRequired)?;

    let session_id = cookie
        .value()
        .parse::<Uuid>()
        .map(SessionId)
        .map_err(|_| DomainError::AuthRequired)?;

    let (user_id, csrf_token) = session::validate_session(pool, session_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::AuthRequired)?;

    Ok(AuthIdentity {
        user_id,
        session_id,
        csrf_token,
    })
}

/// Validate CSRF token for mutating requests.
/// Per /docs/spec/security/csrf.md.
pub fn validate_csrf(
    req: &HttpRequest,
    identity: &AuthIdentity,
    csrf_header: &str,
) -> Result<(), DomainError> {
    let header_val = req
        .headers()
        .get(csrf_header)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if header_val == identity.csrf_token {
        Ok(())
    } else {
        Err(DomainError::CsrfInvalid)
    }
}
