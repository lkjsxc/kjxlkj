//! View models for HTML templates

#[derive(Clone, Debug)]
pub struct NavLink {
    pub href: String,
    pub relation: &'static str,
    pub title: String,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub struct HistoryLink {
    pub href: String,
    pub label: String,
    pub created_at: String,
    pub status: &'static str,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct NoteChrome {
    pub id: String,
    pub title: String,
    pub current_href: String,
    pub created_at: String,
    pub updated_at: String,
    pub visibility: &'static str,
    pub previous: Option<NavLink>,
    pub next: Option<NavLink>,
    pub history_href: String,
}

#[derive(Clone, Debug)]
pub struct IndexItem {
    pub href: String,
    pub title: String,
    pub summary: String,
    pub created_at: String,
    pub updated_at: String,
    pub visibility: Option<&'static str>,
}
