use std::sync::Arc;

use crate::adapters::{auth_store::PgAuthStore, content_store::FsContentStore};

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<PgAuthStore>,
    pub content: Arc<FsContentStore>,
}
