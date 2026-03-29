//! Searchable note listing queries

use super::listing_cursor::decode_cursor;
use super::listing_queries::{browse_records, search_records, top_records};
use super::{DbPool, ListedRecord};
use crate::error::AppError;

pub use super::listing_sort::ListSort;

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub include_private: bool,
    pub limit: i64,
    pub query: Option<String>,
    pub sort: ListSort,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ListPage {
    pub records: Vec<ListedRecord>,
    pub next_cursor: Option<String>,
}

pub async fn list_records(pool: &DbPool, request: &ListRequest) -> Result<ListPage, AppError> {
    let limit = request.limit.clamp(1, MAX_LIMIT);
    let query = request
        .query
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let cursor = decode_cursor(request.cursor.as_deref(), query, &request.sort)?;
    if let Some(query) = query {
        search_records(
            pool,
            request.include_private,
            limit,
            query,
            &request.sort,
            cursor.as_ref(),
        )
        .await
    } else {
        browse_records(
            pool,
            request.include_private,
            limit,
            &request.sort,
            cursor.as_ref(),
        )
        .await
    }
}

pub async fn list_recent_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
) -> Result<Vec<ListedRecord>, AppError> {
    top_records(pool, include_private, limit, false).await
}

pub async fn list_favorite_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
) -> Result<Vec<ListedRecord>, AppError> {
    top_records(pool, include_private, limit, true).await
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            include_private: false,
            limit: DEFAULT_LIMIT,
            query: None,
            sort: ListSort::UpdatedDesc,
            cursor: None,
        }
    }
}
