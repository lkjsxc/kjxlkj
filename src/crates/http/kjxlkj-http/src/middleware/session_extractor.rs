use actix_web::dev::ServiceRequest;
use kjxlkj_auth::session;
use kjxlkj_db::models::user::UserRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

/// Authenticated user context attached to the request.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub user: UserRow,
}

/// Extract the authenticated user from the session cookie.
pub async fn extract_auth_user(
    pool: &PgPool,
    req: &ServiceRequest,
) -> Result<AuthUser, DomainError> {
    let cookie = req
        .cookie("session")
        .ok_or(DomainError::InvalidCredentials)?;
    let token = cookie.value();
    let (session_id, user_id) = session::validate_session(pool, token)
        .await?
        .ok_or(DomainError::InvalidCredentials)?;
    let user = repos::users::find_by_id(pool, user_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::InvalidCredentials)?;
    if user.disabled {
        return Err(DomainError::InvalidCredentials);
    }
    Ok(AuthUser {
        user_id,
        session_id,
        user,
    })
}

/// Helper to get AuthUser from an actix_web::HttpRequest.
pub async fn get_auth_user(
    pool: &PgPool,
    req: &actix_web::HttpRequest,
) -> Result<AuthUser, DomainError> {
    let cookie = req
        .cookie("session")
        .ok_or(DomainError::InvalidCredentials)?;
    let token = cookie.value();
    let (session_id, user_id) = session::validate_session(pool, token)
        .await?
        .ok_or(DomainError::InvalidCredentials)?;
    let user = repos::users::find_by_id(pool, user_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::InvalidCredentials)?;
    if user.disabled {
        return Err(DomainError::InvalidCredentials);
    }
    Ok(AuthUser {
        user_id,
        session_id,
        user,
    })
}
