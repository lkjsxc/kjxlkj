use chrono::{Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct PgAuthStore {
    pub pool: PgPool,
}

impl PgAuthStore {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            pool: PgPool::connect(url).await?,
        })
    }

    pub async fn has_admin(&self) -> anyhow::Result<bool> {
        let row = sqlx::query("select exists(select 1 from admin_users)")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.try_get::<bool, _>(0)?)
    }

    pub async fn create_admin(&self, username: &str, password_hash: &str) -> anyhow::Result<()> {
        sqlx::query("insert into admin_users (username, password_hash) values ($1, $2)")
            .bind(username)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_password_hash(&self, username: &str) -> anyhow::Result<Option<String>> {
        let row = sqlx::query("select password_hash from admin_users where username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get::<String, _>(0)))
    }

    pub async fn create_session(&self, username: &str) -> anyhow::Result<Uuid> {
        let session_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7);
        sqlx::query(
            "insert into sessions (id, admin_id, expires_at) \
             select $1, id, $2 from admin_users where username = $3",
        )
        .bind(session_id)
        .bind(expires_at)
        .bind(username)
        .execute(&self.pool)
        .await?;
        Ok(session_id)
    }

    pub async fn valid_session(&self, id: Uuid) -> anyhow::Result<bool> {
        let row = sqlx::query(
            "select exists(select 1 from sessions where id = $1 and expires_at > now())",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.try_get::<bool, _>(0)?)
    }

    pub async fn delete_session(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("delete from sessions where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
