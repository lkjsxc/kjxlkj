#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListKind {
    All,
    Note,
    Media,
}

impl ListKind {
    pub fn resolve(value: Option<&str>) -> Self {
        match value {
            Some("note") => Self::Note,
            Some("media") => Self::Media,
            _ => Self::All,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Note => "note",
            Self::Media => "media",
        }
    }

    pub fn sql_filter(&self, alias: &str) -> String {
        match self {
            Self::All => String::new(),
            Self::Note => format!("AND {alias}.kind = 'note'"),
            Self::Media => format!("AND {alias}.kind = 'media'"),
        }
    }
}
