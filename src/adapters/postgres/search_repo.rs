use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::AppError;
use crate::web::state::{ArticleSummary, SearchHit};

#[derive(Debug, Clone)]
pub struct SearchRepository {
    pool: PgPool,
}

impl SearchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn search(&self, query: &str, admin: bool) -> Result<Vec<SearchHit>, AppError> {
        let rows = sqlx::query_as::<_, (String, Option<String>, bool, Option<String>)>(
            "SELECT slug, title, private, snippet
             FROM search_articles($1, $2)",
        )
        .bind(query)
        .bind(admin)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::database_query)?;
        Ok(rows
            .into_iter()
            .map(|(slug, title, private, snippet)| SearchHit {
                slug,
                title,
                private,
                snippet: snippet.unwrap_or_default(),
            })
            .collect())
    }

    pub async fn clear_index(&self) -> Result<(), AppError> {
        sqlx::query("DELETE FROM article_search_index")
            .execute(&self.pool)
            .await
            .map_err(AppError::database_query)?;
        Ok(())
    }

    pub async fn index_article(
        &self,
        slug: &str,
        title: Option<&str>,
        body: &str,
        private: bool,
        trashed: bool,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO article_search_index (slug, title, body, private, trashed)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (slug) DO UPDATE
             SET title = EXCLUDED.title,
                 body = EXCLUDED.body,
                 private = EXCLUDED.private,
                 trashed = EXCLUDED.trashed,
                 updated_at = now()",
        )
        .bind(slug)
        .bind(title)
        .bind(body)
        .bind(private)
        .bind(trashed)
        .execute(&self.pool)
        .await
        .map_err(AppError::database_query)?;
        Ok(())
    }

    pub async fn load_articles(
        &self,
        include_private: bool,
    ) -> Result<Vec<ArticleSummary>, AppError> {
        let rows =
            sqlx::query_as::<_, (String, Option<String>, bool, DateTime<Utc>, DateTime<Utc>)>(
                "SELECT slug, title, private, created_at, updated_at
             FROM article_search_index
             WHERE trashed = false
               AND ($1 OR private = false)
             ORDER BY created_at ASC, slug ASC",
            )
            .bind(include_private)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::database_query)?;
        Ok(rows
            .into_iter()
            .map(
                |(slug, title, private, created_at, updated_at)| ArticleSummary {
                    slug,
                    title,
                    private,
                    created_at,
                    updated_at,
                },
            )
            .collect())
    }

    pub async fn rename_article(&self, slug: &str, new_slug: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE article_search_index SET slug = $2, updated_at = now() WHERE slug = $1",
        )
        .bind(slug)
        .bind(new_slug)
        .execute(&self.pool)
        .await
        .map_err(AppError::database_query)?;
        Ok(())
    }

    pub async fn set_trashed(&self, slug: &str, trashed: bool) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE article_search_index SET trashed = $2, updated_at = now() WHERE slug = $1",
        )
        .bind(slug)
        .bind(trashed)
        .execute(&self.pool)
        .await
        .map_err(AppError::database_query)?;
        Ok(())
    }

    pub async fn delete_article(&self, slug: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM article_search_index WHERE slug = $1")
            .bind(slug)
            .execute(&self.pool)
            .await
            .map_err(AppError::database_query)?;
        Ok(())
    }

    pub async fn update_article_timeline(
        &self,
        slug: &str,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE article_search_index
             SET created_at = $2, updated_at = $3
             WHERE slug = $1",
        )
        .bind(slug)
        .bind(created_at)
        .bind(updated_at)
        .execute(&self.pool)
        .await
        .map_err(AppError::database_query)?;
        Ok(())
    }
}
