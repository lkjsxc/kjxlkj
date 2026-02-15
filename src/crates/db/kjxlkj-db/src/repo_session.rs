use kjxlkj_domain::ids::{SessionId, UserId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// Session repository per /docs/spec/security/sessions.md.
#[derive(FromRow)]
pub struct SessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub csrf_token: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

pub async fn create_session(
    pool: &PgPool,
    id: SessionId,
    user_id: UserId,
    csrf_token: &str,
    expires_at: OffsetDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (id, user_id, csrf_token, expires_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id.0)
    .bind(user_id.0)
    .bind(csrf_token)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_session(
    pool: &PgPool,
    id: SessionId,
) -> Result<Option<SessionRow>, sqlx::Error> {
    sqlx::query_as::<_, SessionRow>(
        "SELECT id, user_id, csrf_token, expires_at, created_at
         FROM sessions WHERE id = $1 AND expires_at > now()",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

pub async fn delete_session(
    pool: &PgPool,
    id: SessionId,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(id.0)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_expired_sessions(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM sessions WHERE expires_at <= now()")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
