use super::listing::{ListDirection, ListPage, ListSort};
use super::listing_cursor::{page_from_rows, row_to_listed_resource, Cursor, PageCursorContext};
use super::listing_params::{BrowseParams, SearchParams};
use super::{DbPool, ListKind, ListScope, ListedResource, PopularWindow};
use crate::error::AppError;

pub(super) struct ListingQuery<'a> {
    pub(super) include_private: bool,
    pub(super) limit: i64,
    pub(super) query: Option<&'a str>,
    pub(super) direction: &'a ListDirection,
    pub(super) kind: &'a ListKind,
    pub(super) scope: &'a ListScope,
    pub(super) sort: &'a ListSort,
    pub(super) popular_window: PopularWindow,
    pub(super) cursor: Option<&'a Cursor>,
}

pub(super) async fn browse_resources(
    pool: &DbPool,
    request: &ListingQuery<'_>,
) -> Result<ListPage, AppError> {
    let favorite_filter = if request.scope.favorites_only() {
        "AND r.is_favorite = TRUE"
    } else {
        ""
    };
    let kind_filter = request.kind.sql_filter("r");
    let params = BrowseParams::new(request);
    let popular = popular_cte(request.popular_window);
    let sql = format!(
        "WITH popular AS ({popular}), \
         listed AS (SELECT r.id, r.kind, r.alias, r.title, r.summary, r.body, r.media_family, r.file_key, \
         r.content_type, r.byte_size, r.sha256_hex, r.original_filename, r.width, r.height, \
         r.duration_ms, r.is_favorite, r.favorite_position, r.is_private, r.view_count_total, \
         r.last_viewed_at, r.created_at, r.updated_at, r.summary AS preview, COALESCE(p.popular_views, 0)::BIGINT AS popular_views, \
         LOWER(r.title) AS title_key, 0::DOUBLE PRECISION AS rank, 0::DOUBLE PRECISION AS fuzzy \
         FROM resources r LEFT JOIN popular p ON p.resource_id = r.id \
         WHERE r.deleted_at IS NULL AND ($1 OR r.is_private = FALSE) {favorite_filter} {kind_filter}) \
         SELECT * FROM listed WHERE {} AND {} ORDER BY {} LIMIT $11",
        request.sort.binding_clause(2),
        request.sort.cursor_filter(request.direction, 2),
        request.sort.order_clause(request.direction)
    );
    let rows = client(pool)
        .await?
        .query(&sql, &params.refs())
        .await
        .map_err(db_err)?;
    Ok(page_from_rows(rows, request.limit, &context(request)))
}

pub(super) async fn search_resources(
    pool: &DbPool,
    request: &ListingQuery<'_>,
) -> Result<ListPage, AppError> {
    let query = request.query.unwrap_or_default();
    let favorite_filter = if request.scope.favorites_only() {
        "AND r.is_favorite = TRUE"
    } else {
        ""
    };
    let kind_filter = request.kind.sql_filter("r");
    let params = SearchParams::new(request, query);
    let popular = popular_cte(request.popular_window);
    let sql = format!(
        "WITH q AS (SELECT websearch_to_tsquery('simple', $2) AS tsq, $2::TEXT AS raw), \
         popular AS ({popular}), \
         matched AS (SELECT r.id, r.kind, r.alias, r.title, r.summary, r.body, r.media_family, r.file_key, \
         r.content_type, r.byte_size, r.sha256_hex, r.original_filename, r.width, r.height, r.duration_ms, \
         r.is_favorite, r.favorite_position, r.is_private, r.view_count_total, r.last_viewed_at, r.created_at, r.updated_at, \
         COALESCE(NULLIF(TRIM(ts_headline('simple', body, (SELECT tsq FROM q), 'StartSel=,StopSel=,MaxWords=18,MinWords=8,ShortWord=2,FragmentDelimiter= ... ')), ''), summary) AS preview, \
         COALESCE(p.popular_views, 0)::BIGINT AS popular_views, LOWER(r.title) AS title_key, \
         ts_rank_cd(r.search_document, (SELECT tsq FROM q))::DOUBLE PRECISION AS rank, \
         GREATEST(similarity(COALESCE(r.alias, ''), (SELECT raw FROM q)), similarity(r.title, (SELECT raw FROM q)), \
         similarity(r.body, (SELECT raw FROM q)), similarity(COALESCE(r.original_filename, ''), (SELECT raw FROM q)))::DOUBLE PRECISION AS fuzzy \
         FROM resources r LEFT JOIN popular p ON p.resource_id = r.id \
         WHERE r.deleted_at IS NULL AND ($1 OR r.is_private = FALSE) {favorite_filter} {kind_filter} \
         AND (r.search_document @@ (SELECT tsq FROM q) OR r.alias ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR r.title ILIKE '%' || (SELECT raw FROM q) || '%' OR r.body ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR COALESCE(r.original_filename, '') ILIKE '%' || (SELECT raw FROM q) || '%' \
         OR similarity(COALESCE(r.alias, ''), (SELECT raw FROM q)) >= 0.15 \
         OR similarity(r.title, (SELECT raw FROM q)) >= 0.15 OR similarity(r.body, (SELECT raw FROM q)) >= 0.05)) \
         SELECT * FROM matched WHERE {} AND {} ORDER BY {} LIMIT $12",
        request.sort.binding_clause(3),
        request.sort.cursor_filter(request.direction, 3),
        request.sort.order_clause(request.direction)
    );
    let rows = client(pool)
        .await?
        .query(&sql, &params.refs())
        .await
        .map_err(db_err)?;
    Ok(page_from_rows(rows, request.limit, &context(request)))
}

pub(super) async fn top_resources(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    favorites_only: bool,
) -> Result<Vec<ListedResource>, AppError> {
    let (filter, order) = if favorites_only {
        (
            "AND is_favorite = TRUE",
            "favorite_position ASC NULLS LAST, id ASC",
        )
    } else {
        ("", "updated_at DESC, id ASC")
    };
    let sql = format!(
        "SELECT id, kind, alias, title, summary, body, media_family, file_key, content_type, byte_size, \
         sha256_hex, original_filename, width, height, duration_ms, is_favorite, favorite_position, \
         is_private, view_count_total, last_viewed_at, created_at, updated_at, summary AS preview, NULL::BIGINT AS popular_views \
         FROM resources WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE) {filter} ORDER BY {order} LIMIT $2"
    );
    client(pool)
        .await?
        .query(&sql, &[&include_private, &limit])
        .await
        .map(|rows| rows.into_iter().map(row_to_listed_resource).collect())
        .map_err(db_err)
}

fn context<'a>(request: &'a ListingQuery<'a>) -> PageCursorContext<'a> {
    PageCursorContext {
        query: request.query,
        kind: request.kind,
        scope: request.scope,
        direction: request.direction,
        sort: request.sort,
        popular_window: request.popular_window,
        has_cursor: request.cursor.is_some(),
    }
}

fn popular_cte(window: PopularWindow) -> String {
    window.days().map_or_else(
        || {
            "SELECT id AS resource_id, view_count_total AS popular_views \
             FROM resources WHERE deleted_at IS NULL"
                .to_string()
        },
        |days| {
            format!(
                "SELECT resource_id, SUM(view_count)::BIGINT AS popular_views \
                 FROM resource_daily_views WHERE view_date >= CURRENT_DATE - ({days} - 1) GROUP BY resource_id"
            )
        },
    )
}

fn db_err(error: tokio_postgres::Error) -> AppError {
    AppError::DatabaseError(error.to_string())
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
