use super::listing::{ListDirection, ListPage, ListSort};
use super::listing_cursor::{page_from_rows, row_to_listed_record, Cursor};
use super::{DbPool, ListedRecord};
use crate::error::AppError;

pub(super) async fn browse_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    sort: &ListSort,
    cursor: Option<&Cursor>,
) -> Result<ListPage, AppError> {
    let sql = format!(
        "WITH listed AS (SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
         view_count_total, last_viewed_at, created_at, updated_at, summary AS preview, \
         NULL::BIGINT AS popular_views, LOWER(title) AS title_key, 0::DOUBLE PRECISION AS rank, 0::DOUBLE PRECISION AS fuzzy \
         FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE)) \
         SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
         view_count_total, last_viewed_at, created_at, updated_at, preview, popular_views, title_key, rank, fuzzy \
         FROM listed WHERE {} AND {} ORDER BY {} LIMIT $8",
        sort.binding_clause(2),
        sort.cursor_filter(direction, 2),
        sort.order_clause(direction)
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
    Ok(page_from_rows(
        rows,
        limit,
        None,
        direction,
        sort,
        cursor.is_some(),
    ))
}

pub(super) async fn search_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    query: &str,
    direction: &ListDirection,
    sort: &ListSort,
    cursor: Option<&Cursor>,
) -> Result<ListPage, AppError> {
    let sql = format!(
        "WITH q AS (SELECT websearch_to_tsquery('simple', $2) AS tsq, $2::TEXT AS raw), \
         matched AS (SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
         view_count_total, last_viewed_at, created_at, updated_at, \
         COALESCE(NULLIF(TRIM(ts_headline('simple', body, (SELECT tsq FROM q), \
         'StartSel=,StopSel=,MaxWords=18,MinWords=8,ShortWord=2,FragmentDelimiter= ... ')), ''), summary) AS preview, \
         NULL::BIGINT AS popular_views, LOWER(title) AS title_key, \
         ts_rank_cd(search_document, (SELECT tsq FROM q))::DOUBLE PRECISION AS rank, \
         GREATEST(similarity(COALESCE(alias, ''), (SELECT raw FROM q)), similarity(title, (SELECT raw FROM q)), \
         similarity(body, (SELECT raw FROM q)))::DOUBLE PRECISION AS fuzzy \
         FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) \
         AND (search_document @@ (SELECT tsq FROM q) OR alias ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR title ILIKE '%' || (SELECT raw FROM q) || '%' OR body ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR similarity(COALESCE(alias, ''), (SELECT raw FROM q)) >= 0.15 \
         OR similarity(title, (SELECT raw FROM q)) >= 0.15 OR similarity(body, (SELECT raw FROM q)) >= 0.05)) \
         SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
         view_count_total, last_viewed_at, created_at, updated_at, preview, popular_views, title_key, rank, fuzzy \
         FROM matched WHERE {} AND {} ORDER BY {} LIMIT $9",
        sort.binding_clause(3),
        sort.cursor_filter(direction, 3),
        sort.order_clause(direction)
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
    Ok(page_from_rows(
        rows,
        limit,
        Some(query),
        direction,
        sort,
        cursor.is_some(),
    ))
}

pub(super) async fn top_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    favorites_only: bool,
) -> Result<Vec<ListedRecord>, AppError> {
    let (filter, order) = if favorites_only {
        (
            "AND is_favorite = TRUE",
            "favorite_position ASC NULLS LAST, id ASC",
        )
    } else {
        ("", "updated_at DESC, id ASC")
    };
    let rows = client(pool)
        .await?
        .query(
            &format!(
                "SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
                 view_count_total, last_viewed_at, created_at, updated_at, summary AS preview, NULL::BIGINT AS popular_views \
                 FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) {filter} \
                 ORDER BY {order} LIMIT $2"
            ),
            &[&include_private, &limit],
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
