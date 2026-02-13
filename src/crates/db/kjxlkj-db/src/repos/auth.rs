use crate::models::{DbSessionWithUser, DbUser, DbWorkspace};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn owner_exists(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM users WHERE role = 'owner' AND status = 'active')",
    )
    .fetch_one(pool)
    .await?;
    Ok(exists)
}

pub async fn create_owner_with_workspace(
    pool: &PgPool,
    email: &str,
    display_name: &str,
    password_hash: &str,
    workspace_slug: &str,
    workspace_name: &str,
) -> Result<(DbUser, DbWorkspace), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as::<_, DbUser>(
        "INSERT INTO users (id, email, display_name, password_hash, role, status)
         VALUES ($1, $2, $3, $4, 'owner', 'active')
         RETURNING id, email, display_name, password_hash, role, status, created_at",
    )
    .bind(Uuid::now_v7())
    .bind(email)
    .bind(display_name)
    .bind(password_hash)
    .fetch_one(&mut *tx)
    .await?;

    let workspace = sqlx::query_as::<_, DbWorkspace>(
        "INSERT INTO workspaces (id, slug, name, owner_user_id, state)
         VALUES ($1, $2, $3, $4, 'active')
         RETURNING id, slug, name, owner_user_id, state, created_at, updated_at",
    )
    .bind(Uuid::now_v7())
    .bind(workspace_slug)
    .bind(workspace_name)
    .bind(user.id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO workspace_memberships (workspace_id, user_id, role)
         VALUES ($1, $2, 'owner')",
    )
    .bind(workspace.id)
    .bind(user.id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok((user, workspace))
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<DbUser>, sqlx::Error> {
    sqlx::query_as::<_, DbUser>(
        "SELECT id, email, display_name, password_hash, role, status, created_at
         FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn create_session(
    pool: &PgPool,
    session_id: Uuid,
    user_id: Uuid,
    csrf_token: &str,
    expires_at: OffsetDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (id, user_id, csrf_token, expires_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(session_id)
    .bind(user_id)
    .bind(csrf_token)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_session_with_user(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Option<DbSessionWithUser>, sqlx::Error> {
    sqlx::query_as::<_, DbSessionWithUser>(
        "SELECT
            s.id AS session_id,
            s.user_id,
            s.csrf_token,
            s.expires_at,
            u.email,
            u.display_name,
            u.role,
            u.status
         FROM sessions s
         INNER JOIN users u ON u.id = s.user_id
         WHERE s.id = $1
           AND s.revoked_at IS NULL
           AND s.expires_at > NOW()",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
}

pub async fn revoke_session(pool: &PgPool, session_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET revoked_at = NOW() WHERE id = $1")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn touch_session(pool: &PgPool, session_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET last_seen_at = NOW() WHERE id = $1")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}
