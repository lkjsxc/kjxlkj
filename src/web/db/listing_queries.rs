use super::listing::{ListDirection, ListPage, ListScope, ListSort};
use super::listing_cursor::{page_from_rows, row_to_listed_record, Cursor, PageParams};
use super::{DbPool, ListedRecord, PopularWindow};
use crate::error::AppError;

pub(super) struct QueryParams<'a> {
    pub(super) include_private: bool,
    pub(super) limit: i64,
    pub(super) scope: &'a ListScope,
    pub(super) popular_window: PopularWindow,
    pub(super) direction: &'a ListDirection,
    pub(super) sort: &'a ListSort,
    pub(super) cursor: Option<&'a Cursor>,
}

pub(super) async fn browse_records(
    pool: &DbPool,
    params: &QueryParams<'_>,
) -> Result<ListPage, AppError> {
    let sql = format!(
        "WITH popular AS (SELECT record_id, COALESCE(SUM(view_count), 0)::BIGINT AS popular_views \
         FROM record_daily_views WHERE view_date >= CURRENT_DATE - ($2::INT - 1) GROUP BY record_id), \
         listed AS (SELECT r.id, r.alias, r.title, r.summary, r.body, r.is_favorite, r.favorite_position, \
         r.is_private, r.view_count_total, r.last_viewed_at, r.created_at, r.updated_at, \
         r.summary AS preview, COALESCE(popular.popular_views, 0)::BIGINT AS popular_views, \
         COALESCE(r.favorite_position, 9223372036854775807)::BIGINT AS favorite_key, \
         LOWER(r.title) AS title_key, 0::DOUBLE PRECISION AS rank, 0::DOUBLE PRECISION AS fuzzy \
         FROM records r LEFT JOIN popular ON popular.record_id = r.id \
         WHERE r.deleted_at IS NULL AND ($1 OR r.is_private = FALSE) {}) \
         SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
         view_count_total, last_viewed_at, created_at, updated_at, preview, popular_views, \
         favorite_key, title_key, rank, fuzzy FROM listed WHERE {} AND {} ORDER BY {} LIMIT $12",
        params.scope.filter_clause(),
        params.sort.binding_clause(3),
        params.sort.cursor_filter(params.direction, 3),
        params.sort.order_clause(params.direction)
    );
    let rows = client(pool)
        .await?
        .query(
            &sql,
            &[
                &params.include_private,
                &params.popular_window.days(),
                &params.cursor.and_then(|item| item.updated_at),
                &params.cursor.and_then(|item| item.created_at),
                &params.cursor.and_then(|item| item.title_key.as_deref()),
                &params.cursor.and_then(|item| item.rank),
                &params.cursor.and_then(|item| item.fuzzy),
                &params.cursor.and_then(|item| item.favorite_key),
                &params.cursor.and_then(|item| item.popular_views),
                &params.cursor.and_then(|item| item.view_count_total),
                &params.cursor.map(|item| item.id.as_str()),
                &(params.limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(page_from_rows(rows, &page_params(params, None)))
}

pub(super) async fn search_records(
    pool: &DbPool,
    query: &str,
    params: &QueryParams<'_>,
) -> Result<ListPage, AppError> {
    let sql = format!(
        "WITH q AS (SELECT websearch_to_tsquery('simple', $3) AS tsq, $3::TEXT AS raw), \
         popular AS (SELECT record_id, COALESCE(SUM(view_count), 0)::BIGINT AS popular_views \
         FROM record_daily_views WHERE view_date >= CURRENT_DATE - ($2::INT - 1) GROUP BY record_id), \
         matched AS (SELECT r.id, r.alias, r.title, r.summary, r.body, r.is_favorite, r.favorite_position, \
         r.is_private, r.view_count_total, r.last_viewed_at, r.created_at, r.updated_at, \
         COALESCE(NULLIF(TRIM(ts_headline('simple', r.body, (SELECT tsq FROM q), \
         'StartSel=,StopSel=,MaxWords=18,MinWords=8,ShortWord=2,FragmentDelimiter= ... ')), ''), r.summary) AS preview, \
         COALESCE(popular.popular_views, 0)::BIGINT AS popular_views, \
         COALESCE(r.favorite_position, 9223372036854775807)::BIGINT AS favorite_key, \
         LOWER(r.title) AS title_key, ts_rank_cd(r.search_document, (SELECT tsq FROM q))::DOUBLE PRECISION AS rank, \
         GREATEST(similarity(COALESCE(r.alias, ''), (SELECT raw FROM q)), similarity(r.title, (SELECT raw FROM q)), \
         similarity(r.body, (SELECT raw FROM q)))::DOUBLE PRECISION AS fuzzy \
         FROM records r LEFT JOIN popular ON popular.record_id = r.id \
         WHERE r.deleted_at IS NULL AND ($1 OR r.is_private = FALSE) {} \
         AND (r.search_document @@ (SELECT tsq FROM q) OR r.alias ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR r.title ILIKE '%' || (SELECT raw FROM q) || '%' OR r.body ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR similarity(COALESCE(r.alias, ''), (SELECT raw FROM q)) >= 0.15 \
         OR similarity(r.title, (SELECT raw FROM q)) >= 0.15 OR similarity(r.body, (SELECT raw FROM q)) >= 0.05)) \
         SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
         view_count_total, last_viewed_at, created_at, updated_at, preview, popular_views, \
         favorite_key, title_key, rank, fuzzy FROM matched WHERE {} AND {} ORDER BY {} LIMIT $13",
        params.scope.filter_clause(),
        params.sort.binding_clause(4),
        params.sort.cursor_filter(params.direction, 4),
        params.sort.order_clause(params.direction)
    );
    let rows = client(pool)
        .await?
        .query(
            &sql,
            &[
                &params.include_private,
                &params.popular_window.days(),
                &query,
                &params.cursor.and_then(|item| item.updated_at),
                &params.cursor.and_then(|item| item.created_at),
                &params.cursor.and_then(|item| item.title_key.as_deref()),
                &params.cursor.and_then(|item| item.rank),
                &params.cursor.and_then(|item| item.fuzzy),
                &params.cursor.and_then(|item| item.favorite_key),
                &params.cursor.and_then(|item| item.popular_views),
                &params.cursor.and_then(|item| item.view_count_total),
                &params.cursor.map(|item| item.id.as_str()),
                &(params.limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(page_from_rows(rows, &page_params(params, Some(query))))
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

fn page_params<'a>(params: &'a QueryParams<'a>, query: Option<&'a str>) -> PageParams<'a> {
    PageParams {
        limit: params.limit,
        query,
        scope: params.scope,
        popular_window: params.popular_window,
        direction: params.direction,
        sort: params.sort,
        has_cursor: params.cursor.is_some(),
    }
}
