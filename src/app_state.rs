use std::sync::Arc;

use crate::adapters::{filesystem::FilesystemAdapter, postgres::PostgresAdapter};
use crate::config::AppConfig;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub postgres: Arc<PostgresAdapter>,
    pub filesystem: Arc<FilesystemAdapter>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Result<Self, AppError> {
        let postgres = PostgresAdapter::new(config.database_url.clone())?;
        let filesystem = FilesystemAdapter::new(config.content_root.clone());

        Ok(Self {
            config: Arc::new(config),
            postgres: Arc::new(postgres),
            filesystem: Arc::new(filesystem),
        })
    }
}
