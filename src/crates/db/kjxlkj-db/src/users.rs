//! User repository.

use sqlx::SqlitePool;
use uuid::Uuid;
use time::OffsetDateTime;

use kjxlkj_domain::{User, Session, SecurityEvent, GlobalRole, SecurityEventType};

/// User repository for database operations.
pub struct UserRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new user.
    pub async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, display_name, global_role, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.display_name)
        .bind(serde_json::to_string(&user.global_role).unwrap())
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Find user by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, String, bool, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, email, password_hash, display_name, global_role, is_active, created_at, updated_at
            FROM users WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, email, password_hash, display_name, global_role, is_active, created_at, updated_at)| User {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            email,
            password_hash,
            display_name,
            global_role: serde_json::from_str(&global_role).unwrap_or_default(),
            is_active,
            created_at,
            updated_at,
        }))
    }

    /// Find user by email.
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, String, bool, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, email, password_hash, display_name, global_role, is_active, created_at, updated_at
            FROM users WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, email, password_hash, display_name, global_role, is_active, created_at, updated_at)| User {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            email,
            password_hash,
            display_name,
            global_role: serde_json::from_str(&global_role).unwrap_or_default(),
            is_active,
            created_at,
            updated_at,
        }))
    }

    /// List all users.
    pub async fn list(&self) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, Option<String>, String, bool, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, email, password_hash, display_name, global_role, is_active, created_at, updated_at
            FROM users WHERE is_active = true ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, email, password_hash, display_name, global_role, is_active, created_at, updated_at)| User {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                email,
                password_hash,
                display_name,
                global_role: serde_json::from_str(&global_role).unwrap_or_default(),
                is_active,
                created_at,
                updated_at,
            })
            .collect())
    }

    /// Update user role.
    pub async fn update_role(&self, id: Uuid, role: GlobalRole) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users SET global_role = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?
            "#,
        )
        .bind(serde_json::to_string(&role).unwrap())
        .bind(id.to_string())
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Count users.
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(self.pool)
            .await?;
        Ok(row.0)
    }
}

/// Session repository.
pub struct SessionRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SessionRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new session.
    pub async fn create(&self, session: &Session) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, token, expires_at, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(session.id.to_string())
        .bind(session.user_id.to_string())
        .bind(&session.token)
        .bind(session.expires_at)
        .bind(session.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Find session by token.
    pub async fn find_by_token(&self, token: &str) -> Result<Option<Session>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, user_id, token, expires_at, created_at
            FROM sessions WHERE token = ?
            "#,
        )
        .bind(token)
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, user_id, token, expires_at, created_at)| Session {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            user_id: Uuid::parse_str(&user_id).unwrap_or_default(),
            token,
            expires_at,
            created_at,
        }))
    }

    /// Delete session.
    pub async fn delete(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE token = ?")
            .bind(token)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}

/// Security event repository.
pub struct SecurityEventRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SecurityEventRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a security event.
    pub async fn create(&self, event: &SecurityEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO security_events (id, user_id, event_type, ip_address, user_agent, details, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(event.id.to_string())
        .bind(event.user_id.map(|u| u.to_string()))
        .bind(serde_json::to_string(&event.event_type).unwrap())
        .bind(&event.ip_address)
        .bind(&event.user_agent)
        .bind(&event.details)
        .bind(event.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
