use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::auth::SessionRecord;
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        admin_id: i64,
        timeout_minutes: i32,
    ) -> Result<SessionRecord, AppError> {
        let session = Self::build_new_session(admin_id, Utc::now(), timeout_minutes);
        let (id, admin_id, expires_at, created_at) =
            sqlx::query_as::<_, (Uuid, i64, DateTime<Utc>, DateTime<Utc>)>(
                "INSERT INTO sessions (id, admin_id, expires_at, created_at)
             VALUES ($1, $2, $3, $4)
             RETURNING id, admin_id, expires_at, created_at",
            )
            .bind(session.id)
            .bind(session.admin_id)
            .bind(session.expires_at)
            .bind(session.created_at)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::database_query)?;

        Ok(SessionRecord {
            id,
            admin_id,
            expires_at,
            created_at,
        })
    }

    pub async fn lookup(&self, session_id: Uuid) -> Result<Option<SessionRecord>, AppError> {
        let session = sqlx::query_as::<_, (Uuid, i64, DateTime<Utc>, DateTime<Utc>)>(
            "SELECT id, admin_id, expires_at, created_at FROM sessions WHERE id = $1",
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database_query)?;

        Ok(
            session.map(|(id, admin_id, expires_at, created_at)| SessionRecord {
                id,
                admin_id,
                expires_at,
                created_at,
            }),
        )
    }

    pub async fn delete(&self, session_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::database_query)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn cleanup_expired(&self, now: DateTime<Utc>) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM sessions WHERE expires_at <= $1")
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(AppError::database_query)?;

        Ok(result.rows_affected())
    }

    fn build_new_session(
        admin_id: i64,
        created_at: DateTime<Utc>,
        timeout_minutes: i32,
    ) -> SessionRecord {
        SessionRecord::new_with_timeout_minutes(
            Uuid::new_v4(),
            admin_id,
            created_at,
            i64::from(timeout_minutes),
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use crate::adapters::postgres::session_repo::SessionRepository;

    #[test]
    fn session_builder_sets_24_hour_expiry() {
        let created_at = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();
        let session = SessionRepository::build_new_session(9, created_at, 24 * 60);

        assert_eq!(session.admin_id, 9);
        assert_eq!(session.expires_at, created_at + Duration::hours(24));
    }
}
