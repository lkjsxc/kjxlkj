use chrono::{DateTime, Utc};

use crate::error::AppError;
use crate::web::stores::content::RuntimeContentStore;

impl RuntimeContentStore {
    pub async fn read_or_assign_created_at(
        &self,
        slug: &str,
        fallback: DateTime<Utc>,
    ) -> Result<DateTime<Utc>, AppError> {
        let articles = self.app_state.postgres.search().load_articles(true).await?;
        Ok(articles
            .into_iter()
            .find(|item| item.slug == slug)
            .map(|item| item.created_at)
            .unwrap_or(fallback))
    }

    pub async fn sync_article_metadata(
        &self,
        slug: &str,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        self.app_state
            .postgres
            .search()
            .update_article_timeline(slug, created_at, updated_at)
            .await
    }
}
