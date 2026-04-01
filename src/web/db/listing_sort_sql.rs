use super::listing_direction::ListDirection;
use super::listing_sort::ListSort;

pub fn cursor_filter(sort: &ListSort, direction: &ListDirection, base: i32) -> String {
    let updated = base;
    let created = base + 1;
    let title = base + 2;
    let rank = base + 3;
    let fuzzy = base + 4;
    let favorite = base + 5;
    let popular = base + 6;
    let total = base + 7;
    let id = base + 8;
    match (sort, direction) {
        (ListSort::Relevance, ListDirection::Next) => format!(
            "(${rank}::DOUBLE PRECISION IS NULL OR rank < ${rank} \
             OR (rank = ${rank} AND fuzzy < ${fuzzy}) \
             OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at < ${updated}) \
             OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at = ${updated} AND id > ${id}))"
        ),
        (ListSort::Relevance, ListDirection::Prev) => format!(
            "(${rank}::DOUBLE PRECISION IS NULL OR rank > ${rank} \
             OR (rank = ${rank} AND fuzzy > ${fuzzy}) \
             OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at > ${updated}) \
             OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at = ${updated} AND id < ${id}))"
        ),
        (ListSort::UpdatedDesc, ListDirection::Next) => format!(
            "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} \
             OR (updated_at = ${updated} AND id > ${id}))"
        ),
        (ListSort::UpdatedDesc, ListDirection::Prev) => format!(
            "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} \
             OR (updated_at = ${updated} AND id < ${id}))"
        ),
        (ListSort::UpdatedAsc, ListDirection::Next) => format!(
            "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} \
             OR (updated_at = ${updated} AND id > ${id}))"
        ),
        (ListSort::UpdatedAsc, ListDirection::Prev) => format!(
            "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} \
             OR (updated_at = ${updated} AND id < ${id}))"
        ),
        (ListSort::CreatedDesc, ListDirection::Next) => format!(
            "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} \
             OR (created_at = ${created} AND id > ${id}))"
        ),
        (ListSort::CreatedDesc, ListDirection::Prev) => format!(
            "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} \
             OR (created_at = ${created} AND id < ${id}))"
        ),
        (ListSort::CreatedAsc, ListDirection::Next) => format!(
            "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} \
             OR (created_at = ${created} AND id > ${id}))"
        ),
        (ListSort::CreatedAsc, ListDirection::Prev) => format!(
            "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} \
             OR (created_at = ${created} AND id < ${id}))"
        ),
        (ListSort::TitleAsc, ListDirection::Next) => {
            format!(
                "(${title}::TEXT IS NULL OR title_key > ${title} OR (title_key = ${title} AND id > ${id}))"
            )
        }
        (ListSort::TitleAsc, ListDirection::Prev) => {
            format!(
                "(${title}::TEXT IS NULL OR title_key < ${title} OR (title_key = ${title} AND id < ${id}))"
            )
        }
        (ListSort::TitleDesc, ListDirection::Next) => {
            format!(
                "(${title}::TEXT IS NULL OR title_key < ${title} OR (title_key = ${title} AND id > ${id}))"
            )
        }
        (ListSort::TitleDesc, ListDirection::Prev) => {
            format!(
                "(${title}::TEXT IS NULL OR title_key > ${title} OR (title_key = ${title} AND id < ${id}))"
            )
        }
        (ListSort::FavoriteOrder, ListDirection::Next) => format!(
            "(${favorite}::BIGINT IS NULL OR favorite_key > ${favorite} \
             OR (favorite_key = ${favorite} AND id > ${id}))"
        ),
        (ListSort::FavoriteOrder, ListDirection::Prev) => format!(
            "(${favorite}::BIGINT IS NULL OR favorite_key < ${favorite} \
             OR (favorite_key = ${favorite} AND id < ${id}))"
        ),
        (ListSort::Popular, ListDirection::Next) => format!(
            "(${popular}::BIGINT IS NULL OR popular_views < ${popular} \
             OR (popular_views = ${popular} AND view_count_total < ${total}) \
             OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at < ${updated}) \
             OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at = ${updated} AND id > ${id}))"
        ),
        (ListSort::Popular, ListDirection::Prev) => format!(
            "(${popular}::BIGINT IS NULL OR popular_views > ${popular} \
             OR (popular_views = ${popular} AND view_count_total > ${total}) \
             OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at > ${updated}) \
             OR (popular_views = ${popular} AND view_count_total = ${total} AND updated_at = ${updated} AND id < ${id}))"
        ),
    }
}

pub fn binding_clause(base: i32) -> String {
    let updated = base;
    let created = base + 1;
    let title = base + 2;
    let rank = base + 3;
    let fuzzy = base + 4;
    let favorite = base + 5;
    let popular = base + 6;
    let total = base + 7;
    let id = base + 8;
    format!(
        "(${updated}::TIMESTAMPTZ IS NULL OR ${updated}::TIMESTAMPTZ IS NOT NULL) \
         AND (${created}::TIMESTAMPTZ IS NULL OR ${created}::TIMESTAMPTZ IS NOT NULL) \
         AND (${title}::TEXT IS NULL OR ${title}::TEXT IS NOT NULL) \
         AND (${rank}::DOUBLE PRECISION IS NULL OR ${rank}::DOUBLE PRECISION IS NOT NULL) \
         AND (${fuzzy}::DOUBLE PRECISION IS NULL OR ${fuzzy}::DOUBLE PRECISION IS NOT NULL) \
         AND (${favorite}::BIGINT IS NULL OR ${favorite}::BIGINT IS NOT NULL) \
         AND (${popular}::BIGINT IS NULL OR ${popular}::BIGINT IS NOT NULL) \
         AND (${total}::BIGINT IS NULL OR ${total}::BIGINT IS NOT NULL) \
         AND (${id}::TEXT IS NULL OR ${id}::TEXT IS NOT NULL)"
    )
}
