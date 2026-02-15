//! Session repository per /docs/spec/security/sessions.md.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct SessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub csrf_token: String,
    pub expires_at: String,
    pub created_at: String,
}

/// Create session.
pub async fn create_session(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    csrf_token: &str,
    expires_at: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (id, user_id, csrf_token, expires_at) \
         VALUES ($1, $2, $3, $4::timestamptz)",
    )
    .bind(id)
    .bind(user_id)
    .bind(csrf_token)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find active session by id.
pub async fn find_session(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<SessionRow>, sqlx::Error> {
    sqlx::query_as::<_, SessionRow>(
        "SELECT id, user_id, csrf_token, \
         expires_at::text as expires_at, \
         created_at::text as created_at \
         FROM sessions \
         WHERE id = $1 AND expires_at > NOW()",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Delete session (logout).
pub async fn delete_session(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Clean expired sessions.
pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result =
        sqlx::query("DELETE FROM sessions WHERE expires_at <= NOW()")
            .execute(pool)
            .await?;
    Ok(result.rows_affected())
}
