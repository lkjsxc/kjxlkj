//! Authentication database operations

use super::DbPool;
use crate::error::AppError;
use uuid::Uuid;

/// Check if admin setup is complete
pub async fn is_setup(pool: &DbPool) -> Result<bool, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_one("SELECT EXISTS(SELECT 1 FROM admin_user) AS setup", &[])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get::<_, bool>("setup"))
}

/// Create admin user
pub async fn create_admin(pool: &DbPool, username: &str, password: &str) -> Result<Uuid, AppError> {
    let hash = bcrypt::hash(password, 12)
        .map_err(|e| AppError::StorageError(format!("Password hash failed: {e}")))?;

    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_one(
            "INSERT INTO admin_user (username, password_hash) VALUES ($1, $2) RETURNING id",
            &[&username, &hash],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get("id"))
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
            "SELECT id, password_hash FROM admin_user WHERE username = $1",
            &[&username],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    match row {
        Some(r) => {
            let hash: String = r.get("password_hash");
            let valid = bcrypt::verify(password, &hash).unwrap_or(false);
            if valid {
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

    let row = client
        .query_one(
            "INSERT INTO sessions (user_id, expires_at) \
             VALUES ($1, NOW() + make_interval(mins => $2)) \
             RETURNING id",
            &[&user_id, &minutes],
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

    let row = client
        .query_opt(
            "SELECT user_id FROM sessions WHERE id = $1 AND expires_at > NOW()",
            &[&session_id],
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

    client
        .execute("DELETE FROM sessions WHERE id = $1", &[&session_id])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}
