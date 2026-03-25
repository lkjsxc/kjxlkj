//! View models for HTML templates

#[derive(Clone, Debug)]
pub struct NavLink {
    pub href: String,
    pub label: String,
    pub meta: String,
}

#[derive(Clone, Debug)]
pub struct HistoryLink {
    pub href: String,
    pub label: String,
    pub meta: String,
    pub status: &'static str,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct NoteChrome {
    pub title: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
    pub visibility: &'static str,
    pub previous: Option<NavLink>,
    pub next: Option<NavLink>,
    pub history: Vec<HistoryLink>,
    pub history_href: String,
}

impl NoteChrome {
    pub fn with_history(mut self, history: Vec<HistoryLink>) -> Self {
        self.history = history;
        self
    }
}

#[derive(Clone, Debug)]
pub struct IndexItem {
    pub href: String,
    pub title: String,
    pub slug: String,
    pub meta: String,
    pub status: &'static str,
}
