#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListScope {
    All,
    Favorites,
}

impl ListScope {
    pub fn resolve(value: Option<&str>) -> Self {
        match value {
            Some("favorites") => Self::Favorites,
            _ => Self::All,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Favorites => "favorites",
        }
    }

    pub fn favorites_only(&self) -> bool {
        matches!(self, Self::Favorites)
    }
}
