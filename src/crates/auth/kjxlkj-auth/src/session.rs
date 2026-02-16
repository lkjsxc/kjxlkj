use rand::Rng;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use uuid::Uuid;
use sqlx::PgPool;
use kjxlkj_db::repo_session;

/// Default session TTL: 7 days.
const SESSION_TTL_DAYS: i64 = 7;

/// Generate a random session token (hex-encoded).
pub fn generate_token() -> String {
    let bytes: [u8; 32] = rand::thread_rng().gen();
    hex::encode(bytes)
}

/// Hash a session token for storage.
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Generate a CSRF token.
pub fn generate_csrf_token() -> String {
    let bytes: [u8; 32] = rand::thread_rng().gen();
    hex::encode(bytes)
}

/// Create a new session for a user.
pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(String, Uuid, String), sqlx::Error> {
    let session_id = Uuid::now_v7();
    let token = generate_token();
    let token_hash = hash_token(&token);
    let csrf = generate_csrf_token();
    let expires_at = OffsetDateTime::now_utc()
        + time::Duration::days(SESSION_TTL_DAYS);

    repo_session::create_session(
        pool,
        session_id,
        user_id,
        &token_hash,
        &csrf,
        expires_at,
    )
    .await?;

    Ok((token, session_id, csrf))
}

/// Validate a session token and return session+user info.
pub async fn validate(
    pool: &PgPool,
    token: &str,
) -> Result<Option<repo_session::SessionUserRow>, sqlx::Error> {
    let token_hash = hash_token(token);
    repo_session::validate_session(pool, &token_hash).await
}

/// Revoke a session.
pub async fn revoke(pool: &PgPool, session_id: Uuid) -> Result<(), sqlx::Error> {
    repo_session::delete_session(pool, session_id).await
}
