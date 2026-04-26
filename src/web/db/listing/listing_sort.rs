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
    Popular1dDesc,
    Popular7dDesc,
    Popular30dDesc,
    Popular90dDesc,
    PopularAllDesc,
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
            Self::Popular1dDesc => "popular_1d_desc",
            Self::Popular7dDesc => "popular_7d_desc",
            Self::Popular30dDesc => "popular_30d_desc",
            Self::Popular90dDesc => "popular_90d_desc",
            Self::PopularAllDesc => "popular_all_desc",
            Self::FavoritePositionAsc => "favorite_position_asc",
        }
    }

    pub fn popular_window(&self) -> Option<super::PopularWindow> {
        match self {
            Self::Popular1dDesc => Some(super::PopularWindow::Days1),
            Self::Popular7dDesc => Some(super::PopularWindow::Days7),
            Self::Popular30dDesc => Some(super::PopularWindow::Days30),
            Self::Popular90dDesc => Some(super::PopularWindow::Days90),
            Self::PopularAllDesc => Some(super::PopularWindow::All),
            _ => None,
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
            "popular_1d_desc" => Some(Self::Popular1dDesc),
            "popular_7d_desc" => Some(Self::Popular7dDesc),
            "popular_30d_desc" => Some(Self::Popular30dDesc),
            "popular_90d_desc" => Some(Self::Popular90dDesc),
            "popular_all_desc" => Some(Self::PopularAllDesc),
            "favorite_position_asc" => Some(Self::FavoritePositionAsc),
            _ => None,
        }
    }
}
