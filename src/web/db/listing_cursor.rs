use super::listing::{ListDirection, ListPage, ListSort};
use super::listing_row::row_to_listed_resource;
use super::{ListKind, ListScope, ListedResource, PopularWindow};
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct Cursor {
    pub(super) query: Option<String>,
    pub(super) sort: String,
    pub(super) kind: String,
    pub(super) scope: String,
    pub(super) popular_window: String,
    pub(super) id: String,
    pub(super) updated_at: Option<DateTime<Utc>>,
    pub(super) created_at: Option<DateTime<Utc>>,
    pub(super) title_key: Option<String>,
    pub(super) rank: Option<f64>,
    pub(super) fuzzy: Option<f64>,
    pub(super) favorite_position: Option<i64>,
    pub(super) popular_views: Option<i64>,
    pub(super) view_count_total: Option<i64>,
}

pub(super) struct PageCursorContext<'a> {
    pub(super) query: Option<&'a str>,
    pub(super) kind: &'a ListKind,
    pub(super) scope: &'a ListScope,
    pub(super) direction: &'a ListDirection,
    pub(super) sort: &'a ListSort,
    pub(super) popular_window: PopularWindow,
    pub(super) has_cursor: bool,
}

pub(super) fn page_from_rows(
    mut rows: Vec<tokio_postgres::Row>,
    limit: i64,
    context: &PageCursorContext<'_>,
) -> ListPage {
    let has_more = rows.len() as i64 > limit;
    if has_more {
        rows.pop();
    }
    let mut entries = rows
        .into_iter()
        .map(|row| PageEntry {
            cursor: cursor_from_row(
                &row,
                context.query,
                context.kind,
                context.scope,
                context.sort,
                context.popular_window,
            ),
            resource: row_to_listed_resource(row),
        })
        .collect::<Vec<_>>();
    if matches!(context.direction, ListDirection::Prev) {
        entries.reverse();
    }
    ListPage {
        previous_cursor: edge_cursor(
            &entries,
            context.direction,
            has_more,
            context.has_cursor,
            true,
        ),
        next_cursor: edge_cursor(
            &entries,
            context.direction,
            has_more,
            context.has_cursor,
            false,
        ),
        resources: entries.into_iter().map(|entry| entry.resource).collect(),
    }
}

pub(super) fn decode_cursor(
    cursor: Option<&str>,
    query: Option<&str>,
    sort: &ListSort,
    kind: &ListKind,
    scope: &ListScope,
    popular_window: PopularWindow,
) -> Result<Option<Cursor>, AppError> {
    let Some(cursor) = cursor else {
        return Ok(None);
    };
    let raw = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let text = String::from_utf8(raw)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let cursor: Cursor = serde_json::from_str(&text)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    if cursor.query.as_deref() != query
        || cursor.sort != sort.as_str()
        || cursor.kind != kind.as_str()
        || cursor.scope != scope.as_str()
        || cursor.popular_window != popular_window.as_str()
    {
        return Err(AppError::InvalidRequest("invalid cursor".to_string()));
    }
    Ok(Some(cursor))
}

fn cursor_from_row(
    row: &tokio_postgres::Row,
    query: Option<&str>,
    kind: &ListKind,
    scope: &ListScope,
    sort: &ListSort,
    popular_window: PopularWindow,
) -> Cursor {
    Cursor {
        query: query.map(str::to_string),
        sort: sort.as_str().to_string(),
        kind: kind.as_str().to_string(),
        scope: scope.as_str().to_string(),
        popular_window: popular_window.as_str().to_string(),
        id: row.get("id"),
        updated_at: Some(row.get("updated_at")),
        created_at: Some(row.get("created_at")),
        title_key: Some(row.get("title_key")),
        rank: matches!(sort, ListSort::Relevance).then(|| row.get("rank")),
        fuzzy: matches!(sort, ListSort::Relevance).then(|| row.get("fuzzy")),
        favorite_position: row.get("favorite_position"),
        popular_views: row.try_get("popular_views").ok(),
        view_count_total: Some(row.get("view_count_total")),
    }
}

fn edge_cursor(
    entries: &[PageEntry],
    direction: &ListDirection,
    has_more: bool,
    has_cursor: bool,
    previous: bool,
) -> Option<String> {
    let available = match (direction, previous) {
        (ListDirection::Next, true) => has_cursor,
        (ListDirection::Next, false) => has_more,
        (ListDirection::Prev, true) => has_more,
        (ListDirection::Prev, false) => has_cursor,
    };
    let entry = if available {
        if previous {
            entries.first()
        } else {
            entries.last()
        }
    } else {
        None
    }?;
    Some(URL_SAFE_NO_PAD.encode(serde_json::to_string(&entry.cursor).unwrap()))
}

struct PageEntry {
    resource: ListedResource,
    cursor: Cursor,
}
