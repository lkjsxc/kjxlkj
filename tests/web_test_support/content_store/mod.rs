mod helpers;
mod query;
mod state;

use async_trait::async_trait;

use kjxlkj::core::content::ParsedMarkdown;
use kjxlkj::error::AppError;
use kjxlkj::web::state::{ContentStore, SaveOutcome, SearchHit};

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

    async fn toggle_article_private(&self, slug: &str) -> Result<bool, AppError> {
        self.state.toggle_private(slug).ok_or_else(|| missing(slug))
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
}

impl MockContentStore {
    pub fn insert_article(&self, slug: &str, private: bool, body: &str) {
        self.state.insert_simple(slug, private, body);
    }
}
