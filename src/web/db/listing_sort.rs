use super::listing_direction::ListDirection;
use super::listing_scope::ListScope;
use super::listing_sort_sql;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSort {
    Relevance,
    UpdatedDesc,
    UpdatedAsc,
    CreatedDesc,
    CreatedAsc,
    TitleAsc,
    TitleDesc,
    FavoriteOrder,
    Popular,
}

impl ListSort {
    pub fn resolve(value: Option<&str>, query_present: bool, scope: &ListScope) -> Self {
        match value.and_then(Self::parse) {
            Some(sort) => sort,
            None => match scope {
                ListScope::Favorites => Self::FavoriteOrder,
                ListScope::Popular => Self::Popular,
                ListScope::All if query_present => Self::Relevance,
                ListScope::All => Self::UpdatedDesc,
            },
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
            Self::FavoriteOrder => "favorite_order",
            Self::Popular => "popular",
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
            (Self::FavoriteOrder, ListDirection::Next) => "favorite_key ASC, id ASC",
            (Self::FavoriteOrder, ListDirection::Prev) => "favorite_key DESC, id DESC",
            (Self::Popular, ListDirection::Next) => {
                "popular_views DESC, view_count_total DESC, updated_at DESC, id ASC"
            }
            (Self::Popular, ListDirection::Prev) => {
                "popular_views ASC, view_count_total ASC, updated_at ASC, id DESC"
            }
        }
    }

    pub fn cursor_filter(&self, direction: &ListDirection, base: i32) -> String {
        listing_sort_sql::cursor_filter(self, direction, base)
    }

    pub fn binding_clause(&self, base: i32) -> String {
        listing_sort_sql::binding_clause(base)
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
            "favorite_order" => Some(Self::FavoriteOrder),
            "popular" => Some(Self::Popular),
            _ => None,
        }
    }
}
