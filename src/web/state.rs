use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::core::auth::{AdminUser, SessionRecord};
use crate::core::content::ParsedMarkdown;
use crate::error::AppError;

use super::stores::build_runtime_web_state;

#[async_trait]
pub trait AdminStore: Send + Sync {
    async fn has_admin_user(&self) -> Result<bool, AppError>;
    async fn find_admin_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError>;
    async fn create_admin(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AdminUser, AppError>;
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create_session(&self, admin_id: i64) -> Result<SessionRecord, AppError>;
    async fn lookup_session(&self, session_id: Uuid) -> Result<Option<SessionRecord>, AppError>;
    async fn delete_session(&self, session_id: Uuid) -> Result<bool, AppError>;
    async fn cleanup_expired(&self, now: DateTime<Utc>) -> Result<u64, AppError>;
}

#[async_trait]
pub trait ContentStore: Send + Sync {
    async fn list_public_slugs(&self) -> Result<Vec<String>, AppError>;
    async fn list_admin_slugs(&self) -> Result<Vec<String>, AppError>;
    async fn read_article(&self, slug: &str) -> Result<ParsedMarkdown, AppError>;
    async fn create_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError>;
    async fn save_article(
        &self,
        slug: &str,
        title: Option<String>,
        body: &str,
        private: bool,
    ) -> Result<(), AppError>;
    async fn rename_article(&self, slug: &str, new_slug: &str) -> Result<(), AppError>;
    async fn delete_article(&self, slug: &str) -> Result<(), AppError>;
    async fn toggle_article_private(&self, slug: &str) -> Result<bool, AppError>;
}

#[derive(Clone)]
pub struct WebState {
    pub admin_store: Arc<dyn AdminStore>,
    pub session_store: Arc<dyn SessionStore>,
    pub content_store: Arc<dyn ContentStore>,
}

impl WebState {
    pub fn from_app_state(app_state: AppState) -> Self {
        build_runtime_web_state(app_state)
    }

    pub fn new_for_tests(
        admin_store: Arc<dyn AdminStore>,
        session_store: Arc<dyn SessionStore>,
        content_store: Arc<dyn ContentStore>,
    ) -> Self {
        Self {
            admin_store,
            session_store,
            content_store,
        }
    }
}
