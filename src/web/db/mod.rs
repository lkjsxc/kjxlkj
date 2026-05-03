//! PostgreSQL database adapter

mod analytics;
mod auth;
mod discovery;
mod external_embeds;
mod favorites;
mod list_kind;
mod list_scope;
mod listing;
mod listing_cursor;
mod listing_direction;
mod listing_params;
mod listing_queries;
mod listing_row;
mod listing_sort;
mod listing_sort_sql;
mod media;
mod media_attachments;
mod migrations;
mod models;
mod navigation;
mod password;
mod password_reset;
mod popular_window;
mod resource_ids;
mod resource_support;
mod resources;
mod resources_scoped;
mod settings;
mod settings_model;
mod snapshots;
mod snapshots_cursor;
mod spaces;
mod write_support;

pub use analytics::{count_resource_view, get_resource_view_stats, list_popular_resources};
pub use auth::{
    create_admin, create_session, delete_session, is_setup, validate_session, verify_credentials,
};
pub use discovery::list_public_sitemap_resources;
pub use external_embeds::{
    list_external_embeds, stale_external_embed_urls, upsert_external_embed,
    upsert_external_embed_error,
};
pub use favorites::{list_all_favorite_resources, reorder_favorites};
pub use list_kind::ListKind;
pub use list_scope::ListScope;
pub use listing::{
    list_favorite_resources, list_favorite_resources_in_space, list_recent_resources,
    list_recent_resources_in_space, list_resources, ListDirection, ListRequest, ListSort,
};
pub use media::{create_media, MediaBlob};
pub use media_attachments::{attach_media_to_note, AttachmentCreate, NoteAttachmentUpdate};
pub use models::{
    ExternalEmbed, ListedResource, MediaFamily, Resource, ResourceKind, ResourceSnapshot,
    ResourceStats, ResourceViewStats, SitemapResource,
};
pub use navigation::{get_next_resource, get_previous_resource};
pub use password_reset::{
    issue_password_reset_token, reset_admin_password, update_admin_password, verify_admin_password,
};
pub use popular_window::PopularWindow;
pub use resource_ids::generate_resource_id;
pub use resources::{
    create_resource, delete_resource, get_resource, get_resource_by_alias, get_resource_by_ref,
    update_resource,
};
pub use resources_scoped::{
    create_resource_in_space, delete_resource_in_space, get_resource_by_ref_in_space,
    update_resource_in_space,
};
pub use settings::{get_resource_stats, get_settings, update_settings};
pub use settings_model::AppSettings;
pub use snapshots::{get_snapshot_target, list_resource_snapshots, SnapshotTarget};
pub use spaces::require_space;

use crate::error::AppError;
use deadpool_postgres::{Manager, Pool, Runtime};
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::NoTls;

pub type DbPool = Pool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, AppError> {
    let config: tokio_postgres::Config = database_url
        .parse()
        .map_err(|e| AppError::DatabaseError(format!("Invalid database URL: {e}")))?;
    let mut last_error = String::new();
    for _ in 0..30 {
        let manager = Manager::new(config.clone(), NoTls);
        let pool = Pool::builder(manager)
            .max_size(16)
            .runtime(Runtime::Tokio1)
            .build()
            .map_err(|e| AppError::DatabaseError(format!("Pool creation failed: {e}")))?;
        match migrations::run_migrations(&pool).await {
            Ok(()) => return Ok(pool),
            Err(error) => {
                last_error = error.to_string();
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    Err(AppError::DatabaseError(format!(
        "Connection failed: {last_error}"
    )))
}
