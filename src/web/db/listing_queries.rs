use super::listing::{ListPage, ListSort};
use super::listing_cursor::{page_from_rows, row_to_listed_record, Cursor};
use super::{DbPool, ListedRecord};
use crate::error::AppError;

pub(super) async fn browse_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    sort: &ListSort,
    cursor: Option<&Cursor>,
) -> Result<ListPage, AppError> {
    let sql = format!(
        "WITH listed AS (SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, \
         summary AS preview, LOWER(title) AS title_key, 0::DOUBLE PRECISION AS rank, 0::DOUBLE PRECISION AS fuzzy \
         FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE)) \
         SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, preview, title_key, rank, fuzzy \
         FROM listed WHERE {} AND {} ORDER BY {} LIMIT $8",
        sort.binding_clause(2),
        sort.cursor_filter(2),
        sort.order_clause()
    );
    let rows = client(pool)
        .await?
        .query(
            &sql,
            &[
                &include_private,
                &cursor.and_then(|item| item.updated_at),
                &cursor.and_then(|item| item.created_at),
                &cursor.and_then(|item| item.title_key.as_deref()),
                &cursor.and_then(|item| item.rank),
                &cursor.and_then(|item| item.fuzzy),
                &cursor.map(|item| item.id.as_str()),
                &(limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(page_from_rows(rows, limit, None, sort))
}

pub(super) async fn search_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    query: &str,
    sort: &ListSort,
    cursor: Option<&Cursor>,
) -> Result<ListPage, AppError> {
    let sql = format!(
        "WITH q AS (SELECT websearch_to_tsquery('simple', $2) AS tsq, $2::TEXT AS raw), \
         matched AS (SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, \
         COALESCE(NULLIF(TRIM(ts_headline('simple', body, (SELECT tsq FROM q), \
         'StartSel=,StopSel=,MaxWords=18,MinWords=8,ShortWord=2,FragmentDelimiter= ... ')), ''), summary) AS preview, \
         LOWER(title) AS title_key, ts_rank_cd(search_document, (SELECT tsq FROM q)) AS rank, \
         GREATEST(similarity(COALESCE(alias, ''), (SELECT raw FROM q)), similarity(title, (SELECT raw FROM q)), \
         similarity(body, (SELECT raw FROM q))) AS fuzzy \
         FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) \
         AND (search_document @@ (SELECT tsq FROM q) OR alias ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR title ILIKE '%' || (SELECT raw FROM q) || '%' OR body ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR similarity(COALESCE(alias, ''), (SELECT raw FROM q)) >= 0.15 \
         OR similarity(title, (SELECT raw FROM q)) >= 0.15 OR similarity(body, (SELECT raw FROM q)) >= 0.05)) \
         SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at, preview, title_key, rank, fuzzy \
         FROM matched WHERE {} AND {} ORDER BY {} LIMIT $9",
        sort.binding_clause(3),
        sort.cursor_filter(3),
        sort.order_clause()
    );
    let rows = client(pool)
        .await?
        .query(
            &sql,
            &[
                &include_private,
                &query,
                &cursor.and_then(|item| item.updated_at),
                &cursor.and_then(|item| item.created_at),
                &cursor.and_then(|item| item.title_key.as_deref()),
                &cursor.and_then(|item| item.rank),
                &cursor.and_then(|item| item.fuzzy),
                &cursor.map(|item| item.id.as_str()),
                &(limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(page_from_rows(rows, limit, Some(query), sort))
}

pub(super) async fn top_records(
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
            &[&include_private, &favorites_only, &limit],
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
