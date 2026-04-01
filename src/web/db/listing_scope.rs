#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListScope {
    All,
    Favorites,
    Popular,
}

impl ListScope {
    pub fn resolve(value: Option<&str>) -> Self {
        match value {
            Some("favorites") => Self::Favorites,
            Some("popular") => Self::Popular,
            _ => Self::All,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Favorites => "favorites",
            Self::Popular => "popular",
        }
    }

    pub fn filter_clause(&self) -> &'static str {
        match self {
            Self::All | Self::Popular => "",
            Self::Favorites => "AND is_favorite = TRUE",
        }
    }
}
