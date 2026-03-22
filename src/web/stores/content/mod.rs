mod base;
mod history;
mod mutation;
mod timeline;

use async_trait::async_trait;

use crate::app_state::AppState;
use crate::core::content::ParsedMarkdown;
use crate::error::AppError;
use crate::web::state::{
    ArticleHistory, ArticleNavigation, ArticleSummary, ContentStore, SaveOutcome, SearchHit,
};

#[derive(Clone)]
pub struct RuntimeContentStore {
    pub app_state: AppState,
}

#[async_trait]
impl ContentStore for RuntimeContentStore {
    async fn list_public_slugs(&self) -> Result<Vec<String>, AppError> {
        self.load_slug_list(false).await
    }

    async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        self.load_slug_list(true).await
    }

    async fn list_public_articles(&self) -> Result<Vec<ArticleSummary>, AppError> {
        self.load_articles(false).await
    }

    async fn list_admin_articles(&self) -> Result<Vec<ArticleSummary>, AppError> {
        self.load_articles(true).await
    }

    async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        self.read_article_impl(slug).await
    }

    async fn create_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        self.create_article_impl(slug, title, body, private).await
    }

    async fn save_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
        last_known_revision: Option<&str>,
    ) -> Result<SaveOutcome, AppError> {
        self.save_article_impl(slug, title, body, private, last_known_revision)
            .await
    }

    async fn rename_article(&self, slug: &str, new_slug: &str) -> Result<(), AppError> {
        self.rename_article_impl(slug, new_slug).await
    }

    async fn delete_article(&self, slug: &str) -> Result<(), AppError> {
        self.delete_article_impl(slug).await
    }

    async fn toggle_article_private(&self, slug: &str) -> Result<bool, AppError> {
        self.toggle_article_private_impl(slug).await
    }

    async fn list_trashed_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        self.list_trashed_admin_slugs_impl().await
    }

    async fn restore_article(&self, slug: &str) -> Result<(), AppError> {
        self.restore_article_impl(slug).await
    }

    async fn permanent_delete_article(&self, slug: &str) -> Result<(), AppError> {
        self.permanent_delete_article_impl(slug).await
    }

    async fn search_articles(&self, query: &str, admin: bool) -> Result<Vec<SearchHit>, AppError> {
        self.search_articles_impl(query, admin).await
    }

    async fn trigger_search_reindex(&self) -> Result<(), AppError> {
        self.trigger_search_reindex_impl().await
    }

    async fn article_navigation(
        &self,
        slug: &str,
        admin: bool,
    ) -> Result<ArticleNavigation, AppError> {
        self.article_navigation_impl(slug, admin).await
    }

    async fn article_history(&self, slug: &str) -> Result<ArticleHistory, AppError> {
        self.article_history_impl(slug).await
    }

    async fn restore_article_version(&self, slug: &str, commit_id: &str) -> Result<(), AppError> {
        self.restore_article_version_impl(slug, commit_id).await
    }
}
