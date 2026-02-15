use kjxlkj_db::repo_session;
use kjxlkj_domain::ids::{SessionId, UserId};
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

/// Session TTL default is 7 days per /docs/spec/security/sessions.md.
const SESSION_TTL_DAYS: i64 = 7;

/// Create a new authenticated session, returning session id and CSRF token.
/// Per /docs/spec/security/sessions.md.
pub async fn create_session(
    pool: &PgPool,
    user_id: UserId,
) -> Result<(SessionId, String), sqlx::Error> {
    let session_id = SessionId(Uuid::now_v7());
    let csrf_token = generate_csrf_token();
    let expires_at = OffsetDateTime::now_utc() + Duration::days(SESSION_TTL_DAYS);
    repo_session::create_session(pool, session_id, user_id, &csrf_token, expires_at).await?;
    Ok((session_id, csrf_token))
}

/// Validate a session and return the user_id and csrf_token if valid.
pub async fn validate_session(
    pool: &PgPool,
    session_id: SessionId,
) -> Result<Option<(UserId, String)>, sqlx::Error> {
    let row = repo_session::find_session(pool, session_id).await?;
    Ok(row.map(|r| (UserId(r.user_id), r.csrf_token)))
}

/// Revoke a session per /docs/spec/security/auth.md logout.
pub async fn revoke_session(
    pool: &PgPool,
    session_id: SessionId,
) -> Result<(), sqlx::Error> {
    repo_session::delete_session(pool, session_id).await
}

/// Generate a random CSRF token per /docs/spec/security/csrf.md.
fn generate_csrf_token() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    hex::encode(hasher.finalize())
}
