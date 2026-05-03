//! Authentication database operations

use super::password;
use super::DbPool;
use crate::error::AppError;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Check if admin setup is complete
pub async fn is_setup(pool: &DbPool) -> Result<bool, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_one("SELECT EXISTS(SELECT 1 FROM users) AS setup", &[])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get::<_, bool>("setup"))
}

/// Create the first local user and personal space.
pub async fn create_admin(pool: &DbPool, username: &str, password: &str) -> Result<Uuid, AppError> {
    let hash = password::hash_secret(password)?;

    let mut client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let tx = client
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let email = local_email(username);

    let row = tx
        .query_one(
            "INSERT INTO users (email, username, display_name, status) \
             VALUES ($1, $2, $3, 'active') RETURNING id",
            &[&email, &username, &username],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let user_id: Uuid = row.get("id");
    tx.execute(
        "INSERT INTO user_local_credentials (user_id, password_hash) VALUES ($1, $2)",
        &[&user_id, &hash],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let space = tx
        .query_one(
            "INSERT INTO spaces (slug, name, owner_user_id) VALUES ($1, $2, $3) RETURNING id",
            &[&username, &username, &user_id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let space_id: Uuid = space.get("id");
    tx.execute(
        "INSERT INTO space_memberships (space_id, user_id, role) VALUES ($1, $2, 'owner')",
        &[&space_id, &user_id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    tx.execute(
        "INSERT INTO space_settings (space_id, site_name) VALUES ($1, $2)",
        &[&space_id, &username],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(user_id)
}

/// Verify admin credentials
pub async fn verify_credentials(
    pool: &DbPool,
    username: &str,
    password: &str,
) -> Result<Option<Uuid>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_opt(
            "SELECT users.id, creds.password_hash \
             FROM users JOIN user_local_credentials creds ON creds.user_id = users.id \
             WHERE users.status = 'active' AND (users.username = $1 OR users.email = $1)",
            &[&username],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    match row {
        Some(r) => {
            let hash: String = r.get("password_hash");
            if password::verify_secret(password, &hash) {
                Ok(Some(r.get("id")))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// Create a new session
pub async fn create_session(pool: &DbPool, user_id: Uuid, minutes: i32) -> Result<Uuid, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let session_id = Uuid::new_v4();
    let token_hash = token_hash(&session_id);
    let row = client
        .query_one(
            "INSERT INTO user_sessions (id, user_id, token_hash, csrf_secret_hash, expires_at) \
             VALUES ($1, $2, $3, $3, NOW() + make_interval(mins => $4)) \
             RETURNING id",
            &[&session_id, &user_id, &token_hash, &minutes],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get("id"))
}

/// Validate a session
pub async fn validate_session(pool: &DbPool, session_id: Uuid) -> Result<Option<Uuid>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let token_hash = token_hash(&session_id);
    let row = client
        .query_opt(
            "UPDATE user_sessions SET last_seen_at = NOW() \
             WHERE token_hash = $1 AND expires_at > NOW() AND revoked_at IS NULL \
             RETURNING user_id",
            &[&token_hash],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.map(|r| r.get("user_id")))
}

/// Delete a session
pub async fn delete_session(pool: &DbPool, session_id: Uuid) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let token_hash = token_hash(&session_id);
    client
        .execute(
            "UPDATE user_sessions SET revoked_at = NOW() WHERE token_hash = $1",
            &[&token_hash],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

fn token_hash(session_id: &Uuid) -> String {
    format!("{:x}", Sha256::digest(session_id.to_string().as_bytes()))
}

fn local_email(username: &str) -> String {
    if username.contains('@') {
        username.to_string()
    } else {
        format!("{username}@local.invalid")
    }
}
