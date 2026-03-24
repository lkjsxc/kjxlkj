//! PostgreSQL database adapter

use crate::error::AppError;
use deadpool_postgres::{Manager, Pool, Runtime};
use tokio_postgres::NoTls;
use uuid::Uuid;

pub type DbPool = Pool;

/// Create a database connection pool
pub async fn create_pool(database_url: &str) -> Result<DbPool, AppError> {
    let config: tokio_postgres::Config = database_url
        .parse()
        .map_err(|e| AppError::DatabaseError(format!("Invalid database URL: {}", e)))?;

    let manager = Manager::new(config, NoTls);
    let pool = Pool::builder(manager)
        .max_size(16)
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| AppError::DatabaseError(format!("Pool creation failed: {}", e)))?;

    run_migrations(&pool).await?;
    Ok(pool)
}

/// Run database migrations
async fn run_migrations(pool: &DbPool) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Connection failed: {}", e)))?;

    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS admin_user (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(255) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES admin_user(id) ON DELETE CASCADE,
                expires_at TIMESTAMPTZ NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);
            CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
            "#,
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Migration failed: {}", e)))?;

    Ok(())
}

/// Check if admin setup is complete
pub async fn is_setup(pool: &DbPool) -> Result<bool, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_one("SELECT EXISTS(SELECT 1 FROM admin_user) AS setup", &[])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get::<_, bool>("setup"))
}

/// Create admin user
pub async fn create_admin(pool: &DbPool, username: &str, password: &str) -> Result<Uuid, AppError> {
    let hash = bcrypt::hash(password, 12)
        .map_err(|e| AppError::StorageError(format!("Password hash failed: {}", e)))?;

    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_one(
            "INSERT INTO admin_user (username, password_hash) VALUES ($1, $2) RETURNING id",
            &[&username, &hash],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get("id"))
}

/// Verify admin credentials
pub async fn verify_credentials(
    pool: &DbPool,
    username: &str,
    password: &str,
) -> Result<Option<Uuid>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_opt(
            "SELECT id, password_hash FROM admin_user WHERE username = $1",
            &[&username],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    match row {
        Some(r) => {
            let hash: String = r.get("password_hash");
            let valid = bcrypt::verify(password, &hash).unwrap_or(false);
            if valid {
                Ok(Some(r.get("id")))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// Create a new session
pub async fn create_session(pool: &DbPool, user_id: Uuid, minutes: u32) -> Result<Uuid, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_one(
            "INSERT INTO sessions (user_id, expires_at) \
             VALUES ($1, NOW() + ($2 || ' minutes')::INTERVAL) \
             RETURNING id",
            &[&user_id, &minutes.to_string()],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.get("id"))
}

/// Validate a session
pub async fn validate_session(pool: &DbPool, session_id: Uuid) -> Result<Option<Uuid>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let row = client
        .query_opt(
            "SELECT user_id FROM sessions WHERE id = $1 AND expires_at > NOW()",
            &[&session_id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(row.map(|r| r.get("user_id")))
}

/// Delete a session
pub async fn delete_session(pool: &DbPool, session_id: Uuid) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    client
        .execute("DELETE FROM sessions WHERE id = $1", &[&session_id])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}
