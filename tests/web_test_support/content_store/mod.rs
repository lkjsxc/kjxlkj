mod helpers;
mod query;
mod state;

use async_trait::async_trait;

use kjxlkj::core::content::ParsedMarkdown;
use kjxlkj::error::AppError;
use kjxlkj::web::state::{
    ArticleHistory, ArticleNavigation, ArticleSummary, ContentStore, SaveOutcome, SearchHit,
};

use self::helpers::missing;
use self::query::search_hits;
use self::state::MockContentState;

#[derive(Clone, Default)]
pub struct MockContentStore {
    state: MockContentState,
}

#[async_trait]
impl ContentStore for MockContentStore {
    async fn list_public_slugs(&self) -> Result<Vec<String>, AppError> {
        Ok(self.state.list_slugs(false))
    }

    async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        Ok(self.state.list_slugs(true))
    }

    async fn list_public_articles(&self) -> Result<Vec<ArticleSummary>, AppError> {
        Ok(self.state.list_articles(false))
    }

    async fn list_admin_articles(&self) -> Result<Vec<ArticleSummary>, AppError> {
        Ok(self.state.list_articles(true))
    }

    async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        self.state.read(slug).ok_or_else(|| missing(slug))
    }

    async fn create_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        self.state.upsert(slug, title, body, private);
        Ok(())
    }

    async fn save_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
        last_known_revision: Option<&str>,
    ) -> Result<SaveOutcome, AppError> {
        self.state
            .save(slug, title, body, private, last_known_revision)
    }

    async fn rename_article(&self, slug: &str, new_slug: &str) -> Result<(), AppError> {
        self.state.rename(slug, new_slug);
        Ok(())
    }

    async fn delete_article(&self, slug: &str) -> Result<(), AppError> {
        self.state.move_to_trash(slug);
        Ok(())
    }

    async fn list_trashed_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        Ok(self.state.list_trash_slugs())
    }

    async fn restore_article(&self, slug: &str) -> Result<(), AppError> {
        if self
            .state
            .trash
            .lock()
            .expect("trash lock poisoned")
            .contains_key(slug)
        {
            self.state.restore(slug);
            return Ok(());
        }
        Err(missing(slug))
    }

    async fn permanent_delete_article(&self, slug: &str) -> Result<(), AppError> {
        if self.state.remove_from_trash(slug) {
            return Ok(());
        }
        Err(missing(slug))
    }

    async fn search_articles(&self, query: &str, admin: bool) -> Result<Vec<SearchHit>, AppError> {
        Ok(search_hits(&self.state, query, admin))
    }

    async fn trigger_search_reindex(&self) -> Result<(), AppError> {
        Ok(())
    }

    async fn article_navigation(
        &self,
        slug: &str,
        admin: bool,
    ) -> Result<ArticleNavigation, AppError> {
        Ok(self.state.navigation_for(slug, admin))
    }

    async fn article_history(&self, slug: &str) -> Result<ArticleHistory, AppError> {
        Ok(self.state.history_for(slug))
    }

    async fn restore_article_version(&self, slug: &str, commit_id: &str) -> Result<(), AppError> {
        self.state.restore_version(slug, commit_id)
    }
}

impl MockContentStore {
    pub fn insert_article(&self, slug: &str, private: bool, body: &str) {
        self.state.insert_simple(slug, private, body);
    }

    pub fn set_article_timeline(
        &self,
        slug: &str,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> bool {
        self.state.set_timeline(slug, created_at, updated_at)
    }
}
