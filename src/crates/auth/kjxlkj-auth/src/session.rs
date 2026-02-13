use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

/// Generate a cryptographic session token (64 hex chars).
pub fn generate_token() -> String {
    let bytes: [u8; 32] = rand::random();
    hex::encode(bytes)
}

/// Hash a session token for storage (SHA-256).
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Create a new session, returning the raw token for the cookie.
pub async fn create_session(pool: &PgPool, user_id: Uuid) -> Result<String, DomainError> {
    let token = generate_token();
    let token_hash = hash_token(&token);
    let session_id = Uuid::new_v4();
    let ttl_days = 7i64; // 7 days per sessions.md
    repos::sessions::create(pool, session_id, user_id, &token_hash, ttl_days)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(token)
}

/// Validate a session token, returning (session_id, user_id) if valid.
pub async fn validate_session(
    pool: &PgPool,
    token: &str,
) -> Result<Option<(Uuid, Uuid)>, DomainError> {
    let token_hash = hash_token(token);
    let row = repos::sessions::find_by_token_hash(pool, &token_hash)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(row.map(|s| (s.id, s.user_id)))
}

/// Revoke a specific session.
pub async fn revoke_session(pool: &PgPool, session_id: Uuid) -> Result<(), DomainError> {
    repos::sessions::delete(pool, session_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}

/// Revoke all sessions for a user.
pub async fn revoke_all_sessions(pool: &PgPool, user_id: Uuid) -> Result<(), DomainError> {
    repos::sessions::delete_by_user(pool, user_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}
