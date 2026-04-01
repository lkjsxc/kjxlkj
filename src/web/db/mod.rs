//! PostgreSQL database adapter

mod analytics;
mod auth;
mod favorites;
mod history;
mod list_scope;
mod listing;
mod listing_cursor;
mod listing_direction;
mod listing_queries;
mod listing_sort;
mod listing_sort_sql;
mod migrations;
mod models;
mod popular_window;
mod records;
mod settings;

pub use analytics::{get_note_view_stats, list_popular_records, record_note_view};
pub use auth::{
    create_admin, create_session, delete_session, is_setup, validate_session, verify_credentials,
};
pub use favorites::{list_all_favorite_records, reorder_favorites};
pub use history::{
    get_next_record, get_previous_record, get_record_revision, list_record_revisions,
};
pub use list_scope::ListScope;
pub use listing::{
    list_favorite_records, list_recent_records, list_records, ListDirection, ListRequest, ListSort,
};
pub use models::{AppSettings, ListedRecord, NoteStats, NoteViewStats, Record, RecordRevision};
pub use popular_window::PopularWindow;
pub use records::{create_record, delete_record, get_record, get_record_by_ref, update_record};
pub use settings::{get_note_stats, get_settings, update_settings};

use crate::error::AppError;
use deadpool_postgres::{Manager, Pool, Runtime};
use tokio_postgres::NoTls;

pub type DbPool = Pool;

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
