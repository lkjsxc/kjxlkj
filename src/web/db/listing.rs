//! Searchable note listing queries

use super::{DbPool, ListedRecord, Record};
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};

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

pub async fn list_recent_records(pool: &DbPool, include_private: bool, limit: i64) -> Result<Vec<ListedRecord>, AppError> {
    top_records(pool, include_private, limit, false).await
}

pub async fn list_favorite_records(pool: &DbPool, include_private: bool, limit: i64) -> Result<Vec<ListedRecord>, AppError> {
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

fn page_from_rows(mut rows: Vec<tokio_postgres::Row>, limit: i64) -> ListPage {
    let next_cursor = if rows.len() as i64 > limit {
        rows.pop().map(|row| encode_cursor(&row.get::<_, DateTime<Utc>>("updated_at"), &row.get::<_, String>("id")))
    } else {
        None
    };
    ListPage {
        records: rows.into_iter().map(row_to_listed_record).collect(),
        next_cursor,
    }
}

fn row_to_listed_record(row: tokio_postgres::Row) -> ListedRecord {
    ListedRecord {
        record: Record {
            id: row.get("id"),
            alias: row.get("alias"),
            title: row.get("title"),
            summary: row.get("summary"),
            body: row.get("body"),
            is_favorite: row.get("is_favorite"),
            is_private: row.get("is_private"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        },
        preview: row.get("preview"),
    }
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

#[derive(Clone, Debug)]
struct Cursor { updated_at: DateTime<Utc>, id: String }

fn encode_cursor(updated_at: &DateTime<Utc>, id: &str) -> String {
    URL_SAFE_NO_PAD.encode(format!("{}|{id}", updated_at.to_rfc3339()))
}

fn decode_cursor(cursor: &str) -> Result<Cursor, AppError> {
    let raw = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let text = String::from_utf8(raw)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let Some((updated_at, id)) = text.split_once('|') else {
        return Err(AppError::InvalidRequest("invalid cursor".to_string()));
    };
    let updated_at = updated_at
        .parse()
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    Ok(Cursor {
        updated_at,
        id: id.to_string(),
    })
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
