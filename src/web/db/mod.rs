//! PostgreSQL database adapter

mod auth;
mod history;
mod migrations;
mod models;
mod records;

pub use auth::{
    create_admin, create_session, delete_session, is_setup, validate_session, verify_credentials,
};
pub use history::{get_next_slug, get_previous_slug, get_record_revision, get_record_revisions};
pub use models::{Record, RecordRevision};
pub use records::{create_record, delete_record, get_record, list_records, update_record};

use crate::error::AppError;
use deadpool_postgres::{Manager, Pool, Runtime};
use tokio_postgres::NoTls;

pub type DbPool = Pool;

/// Create a database connection pool
pub async fn create_pool(database_url: &str) -> Result<DbPool, AppError> {
    let config: tokio_postgres::Config = database_url
        .parse()
        .map_err(|e| AppError::DatabaseError(format!("Invalid database URL: {e}")))?;

    let manager = Manager::new(config, NoTls);
    let pool = Pool::builder(manager)
        .max_size(16)
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| AppError::DatabaseError(format!("Pool creation failed: {e}")))?;

    migrations::run_migrations(&pool).await?;
    Ok(pool)
}
