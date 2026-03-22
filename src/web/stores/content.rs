use async_trait::async_trait;
use tokio::fs;

use crate::adapters::filesystem::FilesystemAdapter;
use crate::app_state::AppState;
use crate::core::content::{
    parse_markdown_document, path_for_slug, revision_token, serialize_markdown_document,
    Frontmatter, ParsedMarkdown,
};
use crate::error::AppError;
use crate::web::state::{ContentStore, SaveConflict, SaveOutcome, SearchHit};
use crate::web::stores::content_index::{index_saved_article, reindex_all};

#[derive(Clone)]
pub struct RuntimeContentStore {
    pub app_state: AppState,
}

#[async_trait]
impl ContentStore for RuntimeContentStore {
    async fn list_public_slugs(&self) -> Result<Vec<String>, AppError> {
        self.app_state.filesystem.list_public_slugs().await
    }

    async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        self.app_state.filesystem.list_admin_slugs().await
    }

    async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        self.app_state.filesystem.read_article(slug).await
    }

    async fn create_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let frontmatter = Frontmatter { title, private };
        let markdown = serialize_markdown_document(&frontmatter, body);
        fs::write(&path, markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        self.app_state
            .postgres
            .search()
            .index_article(slug, frontmatter.title.as_deref(), body, private, false)
            .await
    }

    async fn save_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
        last_known_revision: Option<&str>,
    ) -> Result<SaveOutcome, AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let persisted_revision = match fs::read_to_string(&path).await {
            Ok(markdown) => Some(revision_token(&markdown)),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => None,
            Err(source) => return Err(AppError::content_io(path.display().to_string(), source)),
        };
        let markdown = serialize_markdown_document(&Frontmatter { title, private }, body);
        fs::write(&path, &markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        index_saved_article(&self.app_state, slug, &markdown).await?;
        let submitted_revision = last_known_revision
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let conflict = match (submitted_revision, persisted_revision.as_deref()) {
            (Some(submitted), Some(persisted)) if submitted != persisted => Some(SaveConflict {
                persisted_revision: persisted.to_owned(),
                submitted_revision: submitted.to_owned(),
            }),
            _ => None,
        };
        Ok(SaveOutcome {
            revision: revision_token(&markdown),
            conflict,
        })
    }

    async fn rename_article(&self, slug: &str, new_slug: &str) -> Result<(), AppError> {
        let current = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let next = path_for_slug(self.app_state.filesystem.root(), new_slug)?;
        fs::rename(&current, &next)
            .await
            .map_err(|source| AppError::content_io(current.display().to_string(), source))?;
        self.app_state
            .postgres
            .search()
            .rename_article(slug, new_slug)
            .await
    }

    async fn delete_article(&self, slug: &str) -> Result<(), AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let trash_root = self.app_state.filesystem.root().join(".trash");
        let trash_path = path_for_slug(&trash_root, slug)?;
        fs::create_dir_all(&trash_root)
            .await
            .map_err(|source| AppError::content_io(trash_root.display().to_string(), source))?;
        fs::rename(&path, &trash_path)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        self.app_state
            .postgres
            .search()
            .set_trashed(slug, true)
            .await
    }

    async fn toggle_article_private(&self, slug: &str) -> Result<bool, AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let markdown = fs::read_to_string(&path)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        let mut parsed = parse_markdown_document(&markdown)?;
        parsed.frontmatter.private = !parsed.frontmatter.private;
        let next_value = parsed.frontmatter.private;
        let output = serialize_markdown_document(&parsed.frontmatter, &parsed.body);
        fs::write(&path, output)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        self.app_state
            .postgres
            .search()
            .index_article(
                slug,
                parsed.frontmatter.title.as_deref(),
                &parsed.body,
                parsed.frontmatter.private,
                false,
            )
            .await?;
        Ok(next_value)
    }

    async fn list_trashed_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        let trash_root = self.app_state.filesystem.root().join(".trash");
        FilesystemAdapter::new(trash_root).list_admin_slugs().await
    }

    async fn restore_article(&self, slug: &str) -> Result<(), AppError> {
        let trash_root = self.app_state.filesystem.root().join(".trash");
        let src = path_for_slug(&trash_root, slug)?;
        let dst = path_for_slug(self.app_state.filesystem.root(), slug)?;
        match fs::rename(&src, &dst).await {
            Ok(()) => {}
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(AppError::content_io(src.display().to_string(), source))
            }
            Err(source) => return Err(AppError::content_io(src.display().to_string(), source)),
        }
        self.app_state
            .postgres
            .search()
            .set_trashed(slug, false)
            .await
    }

    async fn permanent_delete_article(&self, slug: &str) -> Result<(), AppError> {
        let trash_root = self.app_state.filesystem.root().join(".trash");
        let path = path_for_slug(&trash_root, slug)?;
        match fs::remove_file(&path).await {
            Ok(()) => {}
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(AppError::content_io(path.display().to_string(), source))
            }
            Err(source) => return Err(AppError::content_io(path.display().to_string(), source)),
        }
        self.app_state.postgres.search().delete_article(slug).await
    }

    async fn search_articles(&self, query: &str, admin: bool) -> Result<Vec<SearchHit>, AppError> {
        self.app_state.postgres.search().search(query, admin).await
    }

    async fn trigger_search_reindex(&self) -> Result<(), AppError> {
        reindex_all(&self.app_state).await
    }
}
