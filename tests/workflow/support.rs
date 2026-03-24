use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};

pub struct TestDatabase {
    admin_pool: PgPool,
    name: String,
    pub url: String,
}

impl TestDatabase {
    pub async fn create(prefix: &str) -> Self {
        let base = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://kjxlkj:kjxlkj@127.0.0.1:55432/kjxlkj".to_owned());
        let admin_url = replace_database_name(&base, "postgres");
        let admin_pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&admin_url)
            .await
            .expect("admin db");
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let name = format!("{prefix}_{suffix}");
        let create_query = format!(r#"CREATE DATABASE "{name}""#);
        admin_pool
            .execute(create_query.as_str())
            .await
            .expect("create db");
        let db_url = replace_database_name(&base, &name);
        Self {
            admin_pool,
            name,
            url: db_url,
        }
    }

    pub async fn drop(self) {
        let terminate = r#"
            SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = $1 AND pid <> pg_backend_pid()
        "#;
        let _ = self
            .admin_pool
            .execute(sqlx::query(terminate).bind(&self.name))
            .await;
        let drop_query = format!(r#"DROP DATABASE IF EXISTS "{}""#, self.name);
        let _ = self.admin_pool.execute(drop_query.as_str()).await;
    }
}

fn replace_database_name(base_url: &str, db_name: &str) -> String {
    let Some((prefix, tail)) = base_url.rsplit_once('/') else {
        panic!("DATABASE_URL must include a database path");
    };
    match tail.split_once('?') {
        Some((_, query)) => format!("{prefix}/{db_name}?{query}"),
        None => format!("{prefix}/{db_name}"),
    }
}
