use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::auth::{
    normalize_session_timeout_minutes, AdminUser, SessionRecord, FIXED_ADMIN_USERNAME,
};
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AuthStore {
    pool: PgPool,
}

impl AuthStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ensure_ready(&self) -> Result<(), AppError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_users (
                id BIGSERIAL PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now()
            )",
        )
        .execute(&self.pool)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sessions (
                id UUID PRIMARY KEY,
                admin_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE,
                expires_at TIMESTAMPTZ NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now()
            )",
        )
        .execute(&self.pool)
        .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_admin_id ON sessions(admin_id)")
            .execute(&self.pool)
            .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at)")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn has_admin_user(&self) -> Result<bool, AppError> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM admin_users")
            .fetch_one(&self.pool)
            .await?;
        Ok(count > 0)
    }

    pub async fn load_admin(&self) -> Result<Option<AdminUser>, AppError> {
        let row = sqlx::query_as::<_, (i64, String, String)>(
            "SELECT id, username, password_hash FROM admin_users WHERE username = $1 LIMIT 1",
        )
        .bind(FIXED_ADMIN_USERNAME)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(id, username, password_hash)| AdminUser {
            id,
            username,
            password_hash,
        }))
    }

    pub async fn create_admin(&self, password_hash: &str) -> Result<AdminUser, AppError> {
        if self.load_admin().await?.is_some() {
            return Err(AppError::InvalidRequest(
                "setup is already completed".to_owned(),
            ));
        }
        let row = sqlx::query_as::<_, (i64, String, String)>(
            "INSERT INTO admin_users (username, password_hash)
             VALUES ($1, $2)
             RETURNING id, username, password_hash",
        )
        .bind(FIXED_ADMIN_USERNAME)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(AdminUser {
            id: row.0,
            username: row.1,
            password_hash: row.2,
        })
    }

    pub async fn create_session(
        &self,
        admin_id: i64,
        timeout_minutes: i32,
    ) -> Result<SessionRecord, AppError> {
        let now = Utc::now();
        let timeout = normalize_session_timeout_minutes(timeout_minutes);
        let record =
            SessionRecord::new_with_timeout_minutes(Uuid::new_v4(), admin_id, now, timeout);
        let row = sqlx::query_as::<_, (Uuid, i64, DateTime<Utc>, DateTime<Utc>)>(
            "INSERT INTO sessions (id, admin_id, created_at, expires_at)
             VALUES ($1, $2, $3, $4)
             RETURNING id, admin_id, created_at, expires_at",
        )
        .bind(record.id)
        .bind(record.admin_id)
        .bind(record.created_at)
        .bind(record.expires_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(SessionRecord {
            id: row.0,
            admin_id: row.1,
            created_at: row.2,
            expires_at: row.3,
        })
    }

    pub async fn lookup_session(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionRecord>, AppError> {
        let row = sqlx::query_as::<_, (Uuid, i64, DateTime<Utc>, DateTime<Utc>)>(
            "SELECT id, admin_id, created_at, expires_at FROM sessions WHERE id = $1",
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(
            row.map(|(id, admin_id, created_at, expires_at)| SessionRecord {
                id,
                admin_id,
                created_at,
                expires_at,
            }),
        )
    }

    pub async fn delete_session(&self, session_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn cleanup_expired(&self, now: DateTime<Utc>) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM sessions WHERE expires_at <= $1")
            .bind(now)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
