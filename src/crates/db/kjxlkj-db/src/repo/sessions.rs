// Session repository per /docs/spec/security/sessions.md
use kjxlkj_domain::types::Session;
use sqlx::PgPool;
use uuid::Uuid;

/// Insert a new session.
pub async fn insert_session(pool: &PgPool, session: &Session, csrf: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (id, user_id, token, csrf_token, expires_at, created_at)
         VALUES ($1, $2, $3, $4, now() + interval '7 days', now())",
    )
    .bind(session.id)
    .bind(session.user_id)
    .bind(&session.token)
    .bind(csrf)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find session by token (validates not expired).
pub async fn find_by_token(
    pool: &PgPool,
    token: &str,
) -> Result<Option<(Uuid, Uuid, String)>, sqlx::Error> {
    let row: Option<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT id, user_id, csrf_token FROM sessions
         WHERE token = $1 AND expires_at > now()",
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Revoke session by ID.
pub async fn revoke_session(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Revoke all sessions for user.
pub async fn revoke_all_for_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Rolling renewal: extend session TTL by 7 days from now.
/// Per /docs/spec/security/sessions.md: Session TTL default is 7 days
/// with rolling renewal.
pub async fn renew_session(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE sessions SET expires_at = now() + interval '7 days' WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Clean up expired sessions.
pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM sessions WHERE expires_at <= now()")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
