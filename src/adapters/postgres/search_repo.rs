use sqlx::PgPool;

use crate::error::AppError;
use crate::web::state::SearchHit;

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
}
