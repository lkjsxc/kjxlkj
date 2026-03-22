use chrono::Utc;
use tokio::fs;

use crate::adapters::filesystem::FilesystemAdapter;
use crate::core::content::{
    path_for_slug, revision_token, serialize_markdown_document, Frontmatter,
};
use crate::error::AppError;
use crate::web::state::{SaveConflict, SaveOutcome};
use crate::web::stores::content::RuntimeContentStore;
use crate::web::stores::content_index::index_saved_article;

impl RuntimeContentStore {
    pub async fn create_article_impl(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let frontmatter = Frontmatter { title, private };
        let markdown = serialize_markdown_document(&frontmatter, body);
        let created_at = Utc::now();
        fs::write(&path, markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        self.app_state
            .postgres
            .search()
            .index_article(slug, frontmatter.title.as_deref(), body, private, false)
            .await?;
        self.sync_article_metadata(slug, created_at, created_at)
            .await?;
        self.maybe_commit_history(slug, "create")?;
        Ok(())
    }

    pub async fn save_article_impl(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
        last_known_revision: Option<&str>,
    ) -> Result<SaveOutcome, AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let persisted_revision = read_persisted_revision(&path).await?;
        let markdown = serialize_markdown_document(&Frontmatter { title, private }, body);
        fs::write(&path, &markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        index_saved_article(&self.app_state, slug, &markdown).await?;
        let updated_at = Utc::now();
        let created_at = self.read_or_assign_created_at(slug, updated_at).await?;
        self.sync_article_metadata(slug, created_at, updated_at)
            .await?;
        let conflict = build_conflict(last_known_revision, persisted_revision.as_deref());
        self.maybe_commit_history(slug, "save")?;
        Ok(SaveOutcome {
            revision: revision_token(&markdown),
            conflict,
            updated_at,
        })
    }

    pub async fn rename_article_impl(&self, slug: &str, new_slug: &str) -> Result<(), AppError> {
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

    pub async fn delete_article_impl(&self, slug: &str) -> Result<(), AppError> {
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

    pub async fn list_trashed_admin_slugs_impl(&self) -> Result<Vec<String>, AppError> {
        let trash_root = self.app_state.filesystem.root().join(".trash");
        FilesystemAdapter::new(trash_root).list_admin_slugs().await
    }

    pub async fn restore_article_impl(&self, slug: &str) -> Result<(), AppError> {
        let trash_root = self.app_state.filesystem.root().join(".trash");
        let src = path_for_slug(&trash_root, slug)?;
        let dst = path_for_slug(self.app_state.filesystem.root(), slug)?;
        match fs::rename(&src, &dst).await {
            Ok(()) => {}
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(AppError::content_io(src.display().to_string(), source));
            }
            Err(source) => return Err(AppError::content_io(src.display().to_string(), source)),
        }
        self.app_state
            .postgres
            .search()
            .set_trashed(slug, false)
            .await
    }

    pub async fn permanent_delete_article_impl(&self, slug: &str) -> Result<(), AppError> {
        let trash_root = self.app_state.filesystem.root().join(".trash");
        let path = path_for_slug(&trash_root, slug)?;
        match fs::remove_file(&path).await {
            Ok(()) => {}
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(AppError::content_io(path.display().to_string(), source));
            }
            Err(source) => return Err(AppError::content_io(path.display().to_string(), source)),
        }
        self.app_state.postgres.search().delete_article(slug).await
    }
}

fn build_conflict(
    last_known_revision: Option<&str>,
    persisted_revision: Option<&str>,
) -> Option<SaveConflict> {
    let submitted_revision = last_known_revision
        .map(str::trim)
        .filter(|value| !value.is_empty());
    match (submitted_revision, persisted_revision) {
        (Some(submitted), Some(persisted)) if submitted != persisted => Some(SaveConflict {
            persisted_revision: persisted.to_owned(),
            submitted_revision: submitted.to_owned(),
        }),
        _ => None,
    }
}

async fn read_persisted_revision(path: &std::path::Path) -> Result<Option<String>, AppError> {
    match fs::read_to_string(path).await {
        Ok(markdown) => Ok(Some(revision_token(&markdown))),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(AppError::content_io(path.display().to_string(), source)),
    }
}
