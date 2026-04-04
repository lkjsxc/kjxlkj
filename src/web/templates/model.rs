//! View models for HTML templates

#[derive(Clone, Debug)]
pub struct NavLink {
    pub href: String,
    pub relation: &'static str,
    pub title: String,
    pub summary: String,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub struct HistoryLink {
    pub href: String,
    pub label: String,
    pub summary: String,
    pub created_at: String,
    pub status: &'static str,
}

#[derive(Clone, Debug)]
pub struct NoteChrome {
    pub id: String,
    pub alias: Option<String>,
    pub title: String,
    pub current_href: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_favorite: bool,
    pub visibility: &'static str,
    pub previous: Option<NavLink>,
    pub next: Option<NavLink>,
    pub history_href: String,
}

#[derive(Clone, Debug)]
pub struct NoteAnalytics {
    pub total: i64,
    pub views_7d: i64,
    pub views_30d: i64,
    pub views_90d: i64,
    pub last_viewed_at: Option<String>,
}

#[derive(Clone, Debug)]
pub struct IndexMetric {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct IndexItem {
    pub id: String,
    pub href: String,
    pub title: String,
    pub summary: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_favorite: bool,
    pub visibility: Option<&'static str>,
    pub metrics: Vec<IndexMetric>,
}
