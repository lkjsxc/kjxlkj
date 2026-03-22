use sqlx::PgPool;

use crate::core::auth::{AdminUser, FIXED_ADMIN_USERNAME};
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

    pub async fn load_fixed_admin(&self) -> Result<Option<AdminUser>, AppError> {
        let admin = sqlx::query_as::<_, (i64, String)>(
            "SELECT id, password_hash FROM admin_users WHERE username = $1 LIMIT 1",
        )
        .bind(FIXED_ADMIN_USERNAME)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database_query)?;

        Ok(admin.map(|(id, password_hash)| AdminUser { id, password_hash }))
    }

    pub async fn create_fixed_admin(&self, password_hash: &str) -> Result<AdminUser, AppError> {
        let (id, password_hash) = sqlx::query_as::<_, (i64, String)>(
            "INSERT INTO admin_users (username, password_hash)
             VALUES ($1, $2)
             RETURNING id, password_hash",
        )
        .bind(FIXED_ADMIN_USERNAME)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::database_query)?;

        Ok(AdminUser { id, password_hash })
    }
}
