/// PostgreSQL user and session repository per /docs/spec/security/auth.md
///
/// Uses sqlx::PgPool for user CRUD and session management.
/// Maps to tables: users, sessions (migration 001).
use crate::pg_rows::pg_err;
use crate::user_repo::{SessionRecord, UserRecord};
use kjxlkj_domain::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

/// PostgreSQL-backed user repository.
pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: &UserRecord) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO users (id, username, password_hash, role, disabled, created_at)
             VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(user.id).bind(&user.username).bind(&user.password_hash)
            .bind(user.role.as_str()).bind(user.disabled).bind(user.created_at)
            .execute(&self.pool).await.map_err(pg_err)?;
        Ok(())
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PgUserRow>, DomainError> {
        let row = sqlx::query_as::<_, PgUserRow>(
            "SELECT id, username, password_hash, role, disabled, created_at
             FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool).await.map_err(pg_err)?;
        Ok(row)
    }

    pub async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PgUserRow>, DomainError> {
        let row = sqlx::query_as::<_, PgUserRow>(
            "SELECT id, username, password_hash, role, disabled, created_at
             FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(pg_err)?;
        Ok(row)
    }

    pub async fn user_count(&self) -> Result<i64, DomainError> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool).await.map_err(pg_err)?;
        Ok(count.0)
    }
}

/// PostgreSQL-backed session repository.
pub struct PgSessionRepo {
    pool: PgPool,
}

impl PgSessionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_session(
        &self,
        session: &SessionRecord,
    ) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO sessions (id, user_id, token, role, expires_at, created_at)
             VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(session.id).bind(session.user_id).bind(&session.token)
            .bind(session.role.as_str()).bind(session.expires_at)
            .bind(session.created_at)
            .execute(&self.pool).await.map_err(pg_err)?;
        Ok(())
    }

    pub async fn get_session_by_token(
        &self,
        token: &str,
    ) -> Result<Option<PgSessionRow>, DomainError> {
        let row = sqlx::query_as::<_, PgSessionRow>(
            "SELECT id, user_id, token, role, expires_at, created_at
             FROM sessions WHERE token = $1 AND expires_at > now()")
            .bind(token)
            .fetch_optional(&self.pool).await.map_err(pg_err)?;
        Ok(row)
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM sessions WHERE token = $1")
            .bind(token)
            .execute(&self.pool).await.map_err(pg_err)?;
        Ok(())
    }

    /// Revoke all sessions for a user per IMP-SEC-03.
    pub async fn revoke_user_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<u64, DomainError> {
        let result = sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool).await.map_err(pg_err)?;
        Ok(result.rows_affected())
    }
}

/// Row type for user from PG.
#[derive(sqlx::FromRow)]
pub struct PgUserRow {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub disabled: bool,
    pub created_at: chrono::NaiveDateTime,
}

/// Row type for session from PG.
#[derive(sqlx::FromRow)]
pub struct PgSessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub role: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}
