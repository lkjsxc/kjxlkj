use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct SessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub csrf_token: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

#[derive(FromRow, serde::Serialize)]
pub struct SessionUserRow {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub is_owner: bool,
    pub csrf_token: String,
    pub expires_at: OffsetDateTime,
}

/// Create a new session.
pub async fn create_session(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    token_hash: &str,
    csrf_token: &str,
    expires_at: OffsetDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (id, user_id, token_hash, csrf_token, expires_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(user_id)
    .bind(token_hash)
    .bind(csrf_token)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// Validate session by token hash and return user info.
pub async fn validate_session(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<SessionUserRow>, sqlx::Error> {
    let row = sqlx::query_as::<_, SessionUserRow>(
        r#"SELECT s.id as session_id, s.user_id, u.username,
                  u.display_name, u.is_owner, s.csrf_token, s.expires_at
         FROM sessions s
         JOIN users u ON s.user_id = u.id
         WHERE s.token_hash = $1
           AND s.expires_at > now()
           AND u.is_disabled = false"#
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Delete/revoke a session by id.
pub async fn delete_session(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Clean up expired sessions (housekeeping).
pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM sessions WHERE expires_at < now()")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
