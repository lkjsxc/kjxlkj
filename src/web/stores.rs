use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::fs;
use uuid::Uuid;

use crate::adapters::filesystem::FilesystemAdapter;
use crate::app_state::AppState;
use crate::core::auth::{AdminUser, SessionRecord};
use crate::core::content::{
    parse_markdown_document, path_for_slug, revision_token, serialize_markdown_document,
    Frontmatter, ParsedMarkdown,
};
use crate::error::AppError;
use crate::web::state::{
    AdminStore, ContentStore, SaveConflict, SaveOutcome, SessionStore, WebState,
};

pub fn build_runtime_web_state(app_state: AppState) -> WebState {
    let admin_store: Arc<dyn AdminStore> = Arc::new(RuntimeAdminStore {
        app_state: app_state.clone(),
    });
    let session_store: Arc<dyn SessionStore> = Arc::new(RuntimeSessionStore {
        app_state: app_state.clone(),
    });
    let content_store: Arc<dyn ContentStore> = Arc::new(RuntimeContentStore {
        filesystem: app_state.filesystem.clone(),
    });

    WebState {
        admin_store,
        session_store,
        content_store,
    }
}

#[derive(Clone)]
struct RuntimeAdminStore {
    app_state: AppState,
}

#[async_trait]
impl AdminStore for RuntimeAdminStore {
    async fn has_admin_user(&self) -> Result<bool, AppError> {
        self.app_state.postgres.has_admin_user().await
    }

    async fn find_admin_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError> {
        self.app_state
            .postgres
            .admins()
            .find_by_username(username)
            .await
    }

    async fn create_admin(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AdminUser, AppError> {
        self.app_state
            .postgres
            .admins()
            .create(username, password_hash)
            .await
    }
}

#[derive(Clone)]
struct RuntimeSessionStore {
    app_state: AppState,
}

#[async_trait]
impl SessionStore for RuntimeSessionStore {
    async fn create_session(&self, admin_id: i64) -> Result<SessionRecord, AppError> {
        self.app_state.postgres.sessions().create(admin_id).await
    }

    async fn lookup_session(&self, session_id: Uuid) -> Result<Option<SessionRecord>, AppError> {
        self.app_state.postgres.sessions().lookup(session_id).await
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<bool, AppError> {
        self.app_state.postgres.sessions().delete(session_id).await
    }

    async fn cleanup_expired(&self, now: DateTime<Utc>) -> Result<u64, AppError> {
        self.app_state
            .postgres
            .sessions()
            .cleanup_expired(now)
            .await
    }
}

#[derive(Clone)]
struct RuntimeContentStore {
    filesystem: Arc<FilesystemAdapter>,
}

#[async_trait]
impl ContentStore for RuntimeContentStore {
    async fn list_public_slugs(&self) -> Result<Vec<String>, AppError> {
        self.filesystem.list_public_slugs().await
    }

    async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError> {
        self.filesystem.list_admin_slugs().await
    }

    async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError> {
        self.filesystem.read_article(slug).await
    }

    async fn create_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError> {
        let path = path_for_slug(self.filesystem.root(), slug)?;
        let markdown = serialize_markdown_document(&Frontmatter { title, private }, body);
        fs::write(&path, markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))
    }

    async fn save_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
        last_known_revision: Option<&str>,
    ) -> Result<SaveOutcome, AppError> {
        let path = path_for_slug(self.filesystem.root(), slug)?;
        let persisted_revision = match fs::read_to_string(&path).await {
            Ok(markdown) => Some(revision_token(&markdown)),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => None,
            Err(source) => return Err(AppError::content_io(path.display().to_string(), source)),
        };
        let markdown = serialize_markdown_document(&Frontmatter { title, private }, body);
        fs::write(&path, &markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;

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
        let current = path_for_slug(self.filesystem.root(), slug)?;
        let next = path_for_slug(self.filesystem.root(), new_slug)?;
        fs::rename(&current, &next)
            .await
            .map_err(|source| AppError::content_io(current.display().to_string(), source))
    }

    async fn delete_article(&self, slug: &str) -> Result<(), AppError> {
        let path = path_for_slug(self.filesystem.root(), slug)?;
        fs::remove_file(&path)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))
    }

    async fn toggle_article_private(&self, slug: &str) -> Result<bool, AppError> {
        let path = path_for_slug(self.filesystem.root(), slug)?;
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
        Ok(next_value)
    }
}
