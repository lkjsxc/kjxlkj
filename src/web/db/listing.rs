//! Searchable note listing queries

use super::listing_cursor::{decode_cursor, page_from_rows, row_to_listed_record, Cursor};
use super::{DbPool, ListedRecord};
use crate::error::AppError;

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub include_private: bool,
    pub limit: i64,
    pub query: Option<String>,
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
    let cursor = request.cursor.as_deref().map(decode_cursor).transpose()?;
    if let Some(query) = query {
        search_records(pool, request.include_private, limit, query, cursor.as_ref()).await
    } else {
        browse_records(pool, request.include_private, limit, cursor.as_ref()).await
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

async fn browse_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    cursor: Option<&Cursor>,
) -> Result<ListPage, AppError> {
    let rows = client(pool)
        .await?
        .query(
            "SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, summary AS preview \
             FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) \
             AND ($2::TIMESTAMPTZ IS NULL OR updated_at < $2 OR (updated_at = $2 AND id > $3)) \
             ORDER BY updated_at DESC, id ASC LIMIT $4",
            &[
                &include_private,
                &cursor.map(|item| item.updated_at),
                &cursor.map(|item| item.id.as_str()),
                &(limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(page_from_rows(rows, limit))
}

async fn search_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    query: &str,
    cursor: Option<&Cursor>,
) -> Result<ListPage, AppError> {
    let rows = client(pool)
        .await?
        .query(
            "WITH q AS (SELECT websearch_to_tsquery('simple', $2) AS tsq, $2::TEXT AS raw) \
             SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, \
             COALESCE(NULLIF(TRIM(ts_headline('simple', body, (SELECT tsq FROM q), \
             'StartSel=,StopSel=,MaxWords=18,MinWords=8,ShortWord=2,FragmentDelimiter= ... ')), ''), summary) AS preview, \
             ts_rank_cd(search_document, (SELECT tsq FROM q)) AS rank, \
             GREATEST(similarity(COALESCE(alias, ''), (SELECT raw FROM q)), \
             similarity(title, (SELECT raw FROM q)), similarity(body, (SELECT raw FROM q))) AS fuzzy \
             FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) \
             AND (search_document @@ (SELECT tsq FROM q) OR alias ILIKE '%' || (SELECT raw FROM q) || '%' \
             OR title ILIKE '%' || (SELECT raw FROM q) || '%' OR body ILIKE '%' || (SELECT raw FROM q) || '%' \
             OR similarity(COALESCE(alias, ''), (SELECT raw FROM q)) >= 0.15 \
             OR similarity(title, (SELECT raw FROM q)) >= 0.15 OR similarity(body, (SELECT raw FROM q)) >= 0.05) \
             AND ($3::TIMESTAMPTZ IS NULL OR updated_at < $3 OR (updated_at = $3 AND id > $4)) \
             ORDER BY rank DESC, fuzzy DESC, updated_at DESC, id ASC LIMIT $5",
            &[
                &include_private,
                &query,
                &cursor.map(|item| item.updated_at),
                &cursor.map(|item| item.id.as_str()),
                &(limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(page_from_rows(rows, limit))
}

async fn top_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    favorites_only: bool,
) -> Result<Vec<ListedRecord>, AppError> {
    let rows = client(pool)
        .await?
        .query(
            "SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, summary AS preview \
             FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) AND ($2 = FALSE OR is_favorite = TRUE) \
             ORDER BY updated_at DESC, id ASC LIMIT $3",
            &[&include_private, &favorites_only, &limit.clamp(1, MAX_LIMIT)],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(rows.into_iter().map(row_to_listed_record).collect())
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            include_private: false,
            limit: DEFAULT_LIMIT,
            query: None,
            cursor: None,
        }
    }
}
