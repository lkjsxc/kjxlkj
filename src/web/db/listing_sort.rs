use super::listing_direction::ListDirection;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSort {
    Relevance,
    UpdatedDesc,
    UpdatedAsc,
    CreatedDesc,
    CreatedAsc,
    TitleAsc,
    TitleDesc,
}

impl ListSort {
    pub fn resolve(value: Option<&str>, query_present: bool) -> Self {
        let parsed = value.and_then(Self::parse);
        match (query_present, parsed) {
            (true, Some(sort)) => sort,
            (true, None) => Self::Relevance,
            (false, Some(Self::UpdatedAsc)) => Self::UpdatedAsc,
            (false, Some(Self::CreatedDesc)) => Self::CreatedDesc,
            (false, Some(Self::CreatedAsc)) => Self::CreatedAsc,
            (false, Some(Self::TitleAsc)) => Self::TitleAsc,
            (false, Some(Self::TitleDesc)) => Self::TitleDesc,
            (false, _) => Self::UpdatedDesc,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Relevance => "relevance",
            Self::UpdatedDesc => "updated_desc",
            Self::UpdatedAsc => "updated_asc",
            Self::CreatedDesc => "created_desc",
            Self::CreatedAsc => "created_asc",
            Self::TitleAsc => "title_asc",
            Self::TitleDesc => "title_desc",
        }
    }

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
        }
    }

    pub fn cursor_filter(&self, direction: &ListDirection, base: i32) -> String {
        let updated = base;
        let created = base + 1;
        let title = base + 2;
        let rank = base + 3;
        let fuzzy = base + 4;
        let id = base + 5;
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
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} \
                 OR (updated_at = ${updated} AND id > ${id}))"
            ),
            (Self::UpdatedDesc, ListDirection::Prev) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} \
                 OR (updated_at = ${updated} AND id < ${id}))"
            ),
            (Self::UpdatedAsc, ListDirection::Next) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} \
                 OR (updated_at = ${updated} AND id > ${id}))"
            ),
            (Self::UpdatedAsc, ListDirection::Prev) => format!(
                "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} \
                 OR (updated_at = ${updated} AND id < ${id}))"
            ),
            (Self::CreatedDesc, ListDirection::Next) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} \
                 OR (created_at = ${created} AND id > ${id}))"
            ),
            (Self::CreatedDesc, ListDirection::Prev) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} \
                 OR (created_at = ${created} AND id < ${id}))"
            ),
            (Self::CreatedAsc, ListDirection::Next) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} \
                 OR (created_at = ${created} AND id > ${id}))"
            ),
            (Self::CreatedAsc, ListDirection::Prev) => format!(
                "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} \
                 OR (created_at = ${created} AND id < ${id}))"
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
        }
    }

    pub fn binding_clause(&self, base: i32) -> String {
        let updated = base;
        let created = base + 1;
        let title = base + 2;
        let rank = base + 3;
        let fuzzy = base + 4;
        let id = base + 5;
        format!(
            "(${updated}::TIMESTAMPTZ IS NULL OR ${updated}::TIMESTAMPTZ IS NOT NULL) \
             AND (${created}::TIMESTAMPTZ IS NULL OR ${created}::TIMESTAMPTZ IS NOT NULL) \
             AND (${title}::TEXT IS NULL OR ${title}::TEXT IS NOT NULL) \
             AND (${rank}::DOUBLE PRECISION IS NULL OR ${rank}::DOUBLE PRECISION IS NOT NULL) \
             AND (${fuzzy}::DOUBLE PRECISION IS NULL OR ${fuzzy}::DOUBLE PRECISION IS NOT NULL) \
             AND (${id}::TEXT IS NULL OR ${id}::TEXT IS NOT NULL)"
        )
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "relevance" => Some(Self::Relevance),
            "updated_desc" => Some(Self::UpdatedDesc),
            "updated_asc" => Some(Self::UpdatedAsc),
            "created_desc" => Some(Self::CreatedDesc),
            "created_asc" => Some(Self::CreatedAsc),
            "title_asc" => Some(Self::TitleAsc),
            "title_desc" => Some(Self::TitleDesc),
            _ => None,
        }
    }
}
