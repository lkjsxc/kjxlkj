use sqlx::PgPool;

use crate::core::auth::AdminUser;
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AdminRepository {
    pool: PgPool,
}

impl AdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn has_any_admin(&self) -> Result<bool, AppError> {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM admin_users)")
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::database_query)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError> {
        let admin = sqlx::query_as::<_, (i64, String, String)>(
            "SELECT id, username, password_hash FROM admin_users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database_query)?;

        Ok(admin.map(|(id, username, password_hash)| AdminUser {
            id,
            username,
            password_hash,
        }))
    }

    pub async fn create(&self, username: &str, password_hash: &str) -> Result<AdminUser, AppError> {
        let (id, username, password_hash) = sqlx::query_as::<_, (i64, String, String)>(
            "INSERT INTO admin_users (username, password_hash)
             VALUES ($1, $2)
             RETURNING id, username, password_hash",
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::database_query)?;

        Ok(AdminUser {
            id,
            username,
            password_hash,
        })
    }
}
