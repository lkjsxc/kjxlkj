use super::listing_queries::ListingQuery;

pub(super) struct BrowseParams<'a> {
    include_private: bool,
    space_slug: Option<&'a str>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    title_key: Option<&'a str>,
    rank: Option<f64>,
    fuzzy: Option<f64>,
    id: Option<&'a str>,
    favorite_position: Option<i64>,
    popular_views: Option<i64>,
    view_count_total: Option<i64>,
    limit: i64,
}

impl<'a> BrowseParams<'a> {
    pub(super) fn new(request: &'a ListingQuery<'a>) -> Self {
        Self {
            include_private: request.include_private,
            space_slug: request.space_slug,
            updated_at: request.cursor.and_then(|item| item.updated_at),
            created_at: request.cursor.and_then(|item| item.created_at),
            title_key: request.cursor.and_then(|item| item.title_key.as_deref()),
            rank: request.cursor.and_then(|item| item.rank),
            fuzzy: request.cursor.and_then(|item| item.fuzzy),
            id: request.cursor.map(|item| item.id.as_str()),
            favorite_position: request.cursor.and_then(|item| item.favorite_position),
            popular_views: request.cursor.and_then(|item| item.popular_views),
            view_count_total: request.cursor.and_then(|item| item.view_count_total),
            limit: request.limit + 1,
        }
    }

    pub(super) fn refs(&'a self) -> [&'a (dyn tokio_postgres::types::ToSql + Sync); 12] {
        [
            &self.include_private,
            &self.updated_at,
            &self.created_at,
            &self.title_key,
            &self.rank,
            &self.fuzzy,
            &self.id,
            &self.favorite_position,
            &self.popular_views,
            &self.view_count_total,
            &self.limit,
            &self.space_slug,
        ]
    }
}

pub(super) struct SearchParams<'a> {
    query: &'a str,
    browse: BrowseParams<'a>,
}

impl<'a> SearchParams<'a> {
    pub(super) fn new(request: &'a ListingQuery<'a>, query: &'a str) -> Self {
        Self {
            query,
            browse: BrowseParams::new(request),
        }
    }

    pub(super) fn refs(&'a self) -> [&'a (dyn tokio_postgres::types::ToSql + Sync); 13] {
        [
            &self.browse.include_private,
            &self.query,
            &self.browse.updated_at,
            &self.browse.created_at,
            &self.browse.title_key,
            &self.browse.rank,
            &self.browse.fuzzy,
            &self.browse.id,
            &self.browse.favorite_position,
            &self.browse.popular_views,
            &self.browse.view_count_total,
            &self.browse.limit,
            &self.browse.space_slug,
        ]
    }
}
