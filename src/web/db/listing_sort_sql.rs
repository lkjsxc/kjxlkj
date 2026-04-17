use super::listing_direction::ListDirection;
use super::listing_sort::ListSort;

impl ListSort {
    pub fn order_clause(&self, direction: &ListDirection) -> &'static str {
        match (self, direction) {
            (Self::Relevance, ListDirection::Next) => {
                "rank DESC, fuzzy DESC, updated_at DESC, id ASC"
            }
            (Self::Relevance, ListDirection::Prev) => {
                "rank ASC, fuzzy ASC, updated_at ASC, id DESC"
            }
            (Self::UpdatedDesc, ListDirection::Next) => "updated_at DESC, id ASC",
            (Self::UpdatedDesc, ListDirection::Prev) => "updated_at ASC, id DESC",
            (Self::UpdatedAsc, ListDirection::Next) => "updated_at ASC, id ASC",
            (Self::UpdatedAsc, ListDirection::Prev) => "updated_at DESC, id DESC",
            (Self::CreatedDesc, ListDirection::Next) => "created_at DESC, id ASC",
            (Self::CreatedDesc, ListDirection::Prev) => "created_at ASC, id DESC",
            (Self::CreatedAsc, ListDirection::Next) => "created_at ASC, id ASC",
            (Self::CreatedAsc, ListDirection::Prev) => "created_at DESC, id DESC",
            (Self::TitleAsc, ListDirection::Next) => "title_key ASC, id ASC",
            (Self::TitleAsc, ListDirection::Prev) => "title_key DESC, id DESC",
            (Self::TitleDesc, ListDirection::Next) => "title_key DESC, id ASC",
            (Self::TitleDesc, ListDirection::Prev) => "title_key ASC, id DESC",
            (Self::Popular1dDesc | Self::Popular7dDesc | Self::Popular30dDesc | Self::Popular90dDesc | Self::PopularAllDesc, ListDirection::Next) => {
                "popular_views DESC, view_count_total DESC, updated_at DESC, id ASC"
            }
            (Self::Popular1dDesc | Self::Popular7dDesc | Self::Popular30dDesc | Self::Popular90dDesc | Self::PopularAllDesc, ListDirection::Prev) => {
                "popular_views ASC, view_count_total ASC, updated_at ASC, id DESC"
            }
            (Self::FavoritePositionAsc, ListDirection::Next) => {
                "favorite_position ASC NULLS LAST, id ASC"
            }
            (Self::FavoritePositionAsc, ListDirection::Prev) => {
                "favorite_position DESC NULLS LAST, id DESC"
            }
        }
    }

    pub fn cursor_filter(&self, direction: &ListDirection, base: i32) -> String {
        let (updated, created, title, rank, fuzzy, id, favorite, popular, total) = offsets(base);
        match (self, direction) {
            (Self::Relevance, ListDirection::Next) => format!(
                "(${rank}::DOUBLE PRECISION IS NULL OR rank < ${rank} \
                 OR (rank = ${rank} AND fuzzy < ${fuzzy}) \
                 OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at < ${updated}) \
                 OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at = ${updated} AND id > ${id}))"
            ),
            (Self::Relevance, ListDirection::Prev) => format!(
                "(${rank}::DOUBLE PRECISION IS NULL OR rank > ${rank} \
                 OR (rank = ${rank} AND fuzzy > ${fuzzy}) \
                 OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at > ${updated}) \
                 OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at = ${updated} AND id < ${id}))"
            ),
            (Self::UpdatedDesc, ListDirection::Next) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} OR (updated_at = ${updated} AND id > ${id}))"
            ),
            (Self::UpdatedDesc, ListDirection::Prev) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} OR (updated_at = ${updated} AND id < ${id}))"
            ),
            (Self::UpdatedAsc, ListDirection::Next) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} OR (updated_at = ${updated} AND id > ${id}))"
            ),
            (Self::UpdatedAsc, ListDirection::Prev) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} OR (updated_at = ${updated} AND id < ${id}))"
            ),
            (Self::CreatedDesc, ListDirection::Next) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} OR (created_at = ${created} AND id > ${id}))"
            ),
            (Self::CreatedDesc, ListDirection::Prev) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} OR (created_at = ${created} AND id < ${id}))"
            ),
            (Self::CreatedAsc, ListDirection::Next) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} OR (created_at = ${created} AND id > ${id}))"
            ),
            (Self::CreatedAsc, ListDirection::Prev) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} OR (created_at = ${created} AND id < ${id}))"
            ),
            (Self::TitleAsc, ListDirection::Next) => format!(
                "(${title}::TEXT IS NULL OR title_key > ${title} OR (title_key = ${title} AND id > ${id}))"
            ),
            (Self::TitleAsc, ListDirection::Prev) => format!(
                "(${title}::TEXT IS NULL OR title_key < ${title} OR (title_key = ${title} AND id < ${id}))"
            ),
            (Self::TitleDesc, ListDirection::Next) => format!(
                "(${title}::TEXT IS NULL OR title_key < ${title} OR (title_key = ${title} AND id > ${id}))"
            ),
            (Self::TitleDesc, ListDirection::Prev) => format!(
                "(${title}::TEXT IS NULL OR title_key > ${title} OR (title_key = ${title} AND id < ${id}))"
            ),
            (Self::Popular1dDesc | Self::Popular7dDesc | Self::Popular30dDesc | Self::Popular90dDesc | Self::PopularAllDesc, ListDirection::Next) => format!(
                "(${popular}::BIGINT IS NULL OR popular_views < ${popular} \
                 OR (popular_views = ${popular} AND view_count_total < ${total}) \
                 OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at < ${updated}) \
                 OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at = ${updated} AND id > ${id}))"
            ),
            (Self::Popular1dDesc | Self::Popular7dDesc | Self::Popular30dDesc | Self::Popular90dDesc | Self::PopularAllDesc, ListDirection::Prev) => format!(
                "(${popular}::BIGINT IS NULL OR popular_views > ${popular} \
                 OR (popular_views = ${popular} AND view_count_total > ${total}) \
                 OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at > ${updated}) \
                 OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at = ${updated} AND id < ${id}))"
            ),
            (Self::FavoritePositionAsc, ListDirection::Next) => format!(
                "(${favorite}::BIGINT IS NULL OR favorite_position > ${favorite} OR (favorite_position = ${favorite} AND id > ${id}))"
            ),
            (Self::FavoritePositionAsc, ListDirection::Prev) => format!(
                "(${favorite}::BIGINT IS NULL OR favorite_position < ${favorite} OR (favorite_position = ${favorite} AND id < ${id}))"
            ),
        }
    }

    pub fn binding_clause(&self, base: i32) -> String {
        let (updated, created, title, rank, fuzzy, id, favorite, popular, total) = offsets(base);
        format!(
            "(${updated}::TIMESTAMPTZ IS NULL OR ${updated}::TIMESTAMPTZ IS NOT NULL) \
             AND (${created}::TIMESTAMPTZ IS NULL OR ${created}::TIMESTAMPTZ IS NOT NULL) \
             AND (${title}::TEXT IS NULL OR ${title}::TEXT IS NOT NULL) \
             AND (${rank}::DOUBLE PRECISION IS NULL OR ${rank}::DOUBLE PRECISION IS NOT NULL) \
             AND (${fuzzy}::DOUBLE PRECISION IS NULL OR ${fuzzy}::DOUBLE PRECISION IS NOT NULL) \
             AND (${id}::TEXT IS NULL OR ${id}::TEXT IS NOT NULL) \
             AND (${favorite}::BIGINT IS NULL OR ${favorite}::BIGINT IS NOT NULL) \
             AND (${popular}::BIGINT IS NULL OR ${popular}::BIGINT IS NOT NULL) \
             AND (${total}::BIGINT IS NULL OR ${total}::BIGINT IS NOT NULL)"
        )
    }
}

fn offsets(base: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    (
        base,
        base + 1,
        base + 2,
        base + 3,
        base + 4,
        base + 5,
        base + 6,
        base + 7,
        base + 8,
    )
}
