//! Searchable resource listing queries

use super::listing_cursor::decode_cursor;
use super::listing_queries::{browse_resources, search_resources, top_resources, ListingQuery};
use super::{DbPool, ListKind, ListScope, ListedResource, PopularWindow};
use crate::error::AppError;

pub use super::listing_direction::ListDirection;
pub use super::listing_sort::ListSort;

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub include_private: bool,
    pub limit: i64,
    pub query: Option<String>,
    pub direction: ListDirection,
    pub kind: ListKind,
    pub scope: ListScope,
    pub sort: ListSort,
    pub popular_window: PopularWindow,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ListPage {
    pub resources: Vec<ListedResource>,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
}

pub async fn list_resources(pool: &DbPool, request: &ListRequest) -> Result<ListPage, AppError> {
    let limit = request.limit.clamp(1, MAX_LIMIT);
    let has_cursor = request.cursor.is_some();
    let query = request
        .query
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let direction = if has_cursor {
        request.direction.clone()
    } else {
        ListDirection::Next
    };
    let cursor = decode_cursor(
        request.cursor.as_deref(),
        query,
        &request.sort,
        &request.kind,
        &request.scope,
        request.popular_window,
    )?;
    if let Some(query) = query {
        search_resources(
            pool,
            &query_request(request, limit, query, &direction, cursor.as_ref()),
        )
        .await
    } else {
        browse_resources(
            pool,
            &query_request(request, limit, "", &direction, cursor.as_ref()),
        )
        .await
    }
}

pub async fn list_recent_resources(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
) -> Result<Vec<ListedResource>, AppError> {
    top_resources(pool, include_private, limit, false).await
}

pub async fn list_favorite_resources(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
) -> Result<Vec<ListedResource>, AppError> {
    top_resources(pool, include_private, limit, true).await
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            include_private: false,
            limit: DEFAULT_LIMIT,
            query: None,
            direction: ListDirection::Next,
            kind: ListKind::All,
            scope: ListScope::All,
            sort: ListSort::UpdatedDesc,
            popular_window: PopularWindow::Days30,
            cursor: None,
        }
    }
}

fn query_request<'a>(
    request: &'a ListRequest,
    limit: i64,
    query: &'a str,
    direction: &'a ListDirection,
    cursor: Option<&'a super::listing_cursor::Cursor>,
) -> ListingQuery<'a> {
    ListingQuery {
        include_private: request.include_private,
        limit,
        query: (!query.is_empty()).then_some(query),
        direction,
        kind: &request.kind,
        scope: &request.scope,
        sort: &request.sort,
        popular_window: request.popular_window,
        cursor,
    }
}
