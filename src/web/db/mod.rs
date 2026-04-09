//! PostgreSQL database adapter

mod analytics;
mod auth;
mod discovery;
mod favorites;
mod list_kind;
mod list_scope;
mod listing;
mod listing_cursor;
mod listing_direction;
mod listing_params;
mod listing_queries;
mod listing_sort;
mod listing_sort_sql;
mod media;
mod migrations;
mod models;
mod navigation;
mod popular_window;
mod record_support;
mod records;
mod resource_ids;
mod revisions;
mod revisions_cursor;
mod settings;

pub use analytics::{get_note_view_stats, list_popular_records, record_note_view};
pub use auth::{
    create_admin, create_session, delete_session, is_setup, validate_session, verify_credentials,
};
pub use discovery::list_public_sitemap_records;
pub use favorites::{list_all_favorite_records, reorder_favorites};
pub use list_kind::ListKind;
pub use list_scope::ListScope;
pub use listing::{
    list_favorite_records, list_recent_records, list_records, ListDirection, ListRequest, ListSort,
};
pub use media::{create_media, replace_media_file, MediaBlob};
pub use models::{
    AppSettings, ListedRecord, MediaFamily, NoteStats, NoteViewStats, Record, RecordKind,
    RecordSnapshot, SitemapRecord,
};
pub use navigation::{get_next_record, get_previous_record};
pub use popular_window::PopularWindow;
pub use records::{
    create_record, delete_record, get_record, get_record_by_alias, get_record_by_ref, update_record,
};
pub use resource_ids::generate_resource_id;
pub use revisions::{get_snapshot_resource, list_record_snapshots, SnapshotResource};
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
