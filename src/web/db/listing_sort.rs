use std::fmt::Write;

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

    pub fn order_clause(&self) -> &'static str {
        match self {
            Self::Relevance => "rank DESC, fuzzy DESC, updated_at DESC, id ASC",
            Self::UpdatedDesc => "updated_at DESC, id ASC",
            Self::UpdatedAsc => "updated_at ASC, id ASC",
            Self::CreatedDesc => "created_at DESC, id ASC",
            Self::CreatedAsc => "created_at ASC, id ASC",
            Self::TitleAsc => "title_key ASC, id ASC",
            Self::TitleDesc => "title_key DESC, id ASC",
        }
    }

    pub fn cursor_filter(&self, base: i32) -> String {
        let updated = base;
        let created = base + 1;
        let title = base + 2;
        let rank = base + 3;
        let fuzzy = base + 4;
        let id = base + 5;
        let mut clause = String::new();
        match self {
            Self::Relevance => {
                let _ = write!(
                    clause,
                    "(${rank}::DOUBLE PRECISION IS NULL OR rank < ${rank} \
                     OR (rank = ${rank} AND fuzzy < ${fuzzy}) \
                     OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at < ${updated}) \
                     OR (rank = ${rank} AND fuzzy = ${fuzzy} AND updated_at = ${updated} AND id > ${id}))"
                );
            }
            Self::UpdatedDesc => {
                let _ = write!(
                    clause,
                    "(${updated}::TIMESTAMPTZ IS NULL OR updated_at < ${updated} \
                     OR (updated_at = ${updated} AND id > ${id}))"
                );
            }
            Self::UpdatedAsc => {
                let _ = write!(
                    clause,
                    "(${updated}::TIMESTAMPTZ IS NULL OR updated_at > ${updated} \
                     OR (updated_at = ${updated} AND id > ${id}))"
                );
            }
            Self::CreatedDesc => {
                let _ = write!(
                    clause,
                    "(${created}::TIMESTAMPTZ IS NULL OR created_at < ${created} \
                     OR (created_at = ${created} AND id > ${id}))"
                );
            }
            Self::CreatedAsc => {
                let _ = write!(
                    clause,
                    "(${created}::TIMESTAMPTZ IS NULL OR created_at > ${created} \
                     OR (created_at = ${created} AND id > ${id}))"
                );
            }
            Self::TitleAsc => {
                let _ = write!(
                    clause,
                    "(${title}::TEXT IS NULL OR title_key > ${title} OR (title_key = ${title} AND id > ${id}))"
                );
            }
            Self::TitleDesc => {
                let _ = write!(
                    clause,
                    "(${title}::TEXT IS NULL OR title_key < ${title} OR (title_key = ${title} AND id > ${id}))"
                );
            }
        }
        clause
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
