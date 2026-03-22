use crate::core::content::ParsedMarkdown;
use crate::error::AppError;
use crate::web::state::{ArticleNavigation, ArticleSummary, SearchHit};
use crate::web::stores::content::RuntimeContentStore;
use crate::web::stores::content_index::reindex_all;

impl RuntimeContentStore {
    pub async fn load_slug_list(&self, include_private: bool) -> Result<Vec<String>, AppError> {
        Ok(self
            .load_articles(include_private)
            .await?
            .into_iter()
            .map(|article| article.slug)
            .collect())
    }

    pub async fn load_articles(
        &self,
        include_private: bool,
    ) -> Result<Vec<ArticleSummary>, AppError> {
        self.app_state
            .postgres
            .search()
            .load_articles(include_private)
            .await
    }

    pub async fn read_article_impl(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        self.app_state.filesystem.read_article(slug).await
    }

    pub async fn search_articles_impl(
        &self,
        query: &str,
        admin: bool,
    ) -> Result<Vec<SearchHit>, AppError> {
        self.app_state.postgres.search().search(query, admin).await
    }

    pub async fn trigger_search_reindex_impl(&self) -> Result<(), AppError> {
        reindex_all(&self.app_state).await
    }

    pub async fn article_navigation_impl(
        &self,
        slug: &str,
        admin: bool,
    ) -> Result<ArticleNavigation, AppError> {
        let items = self.load_articles(admin).await?;
        let index = items.iter().position(|item| item.slug == slug);
        let previous_slug = index
            .and_then(|i| i.checked_sub(1))
            .map(|i| items[i].slug.clone());
        let next_slug = index
            .and_then(|i| i.checked_add(1))
            .filter(|i| *i < items.len())
            .map(|i| items[i].slug.clone());
        Ok(ArticleNavigation {
            previous_slug,
            next_slug,
        })
    }
}
