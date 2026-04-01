use super::list_scope::ListScope;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSort {
    Relevance,
    UpdatedDesc,
    UpdatedAsc,
    CreatedDesc,
    CreatedAsc,
    TitleAsc,
    TitleDesc,
    PopularDesc,
    ViewsTotalDesc,
    FavoritePositionAsc,
}

impl ListSort {
    pub fn resolve(value: Option<&str>, query_present: bool, scope: &ListScope) -> Self {
        let parsed = value.and_then(Self::parse);
        if query_present {
            return parsed.unwrap_or(Self::Relevance);
        }
        match parsed {
            Some(Self::FavoritePositionAsc) if scope.favorites_only() => Self::FavoritePositionAsc,
            Some(Self::FavoritePositionAsc) => Self::UpdatedDesc,
            Some(sort) => sort,
            None if scope.favorites_only() => Self::FavoritePositionAsc,
            None => Self::UpdatedDesc,
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
            Self::PopularDesc => "popular_desc",
            Self::ViewsTotalDesc => "views_total_desc",
            Self::FavoritePositionAsc => "favorite_position_asc",
        }
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
            "popular_desc" => Some(Self::PopularDesc),
            "views_total_desc" => Some(Self::ViewsTotalDesc),
            "favorite_position_asc" => Some(Self::FavoritePositionAsc),
            _ => None,
        }
    }
}
