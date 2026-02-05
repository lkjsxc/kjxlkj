//! Fuzzy finder picker model.
//!
//! Implements the finder state model as specified in
//! `/docs/spec/features/navigation/finder.md`.

use std::path::PathBuf;

/// Source type for finder results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FinderSource {
    /// File list from workspace.
    #[default]
    Files,
    /// Open buffers.
    Buffers,
    /// Recently opened files.
    Recent,
    /// Symbols (from LSP or syntax).
    Symbols,
    /// Grep matches.
    Grep,
    /// Git branches.
    Branches,
    /// Command palette.
    Commands,
}

/// Location for a finder item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinderLocation {
    /// File path only.
    File(PathBuf),
    /// File with line number.
    FileLine { path: PathBuf, line: usize },
    /// File with position.
    FilePosition {
        path: PathBuf,
        line: usize,
        col: usize,
    },
    /// Buffer by ID.
    Buffer(u64),
    /// Command by name.
    Command(String),
}

/// A single item in the finder list.
#[derive(Debug, Clone)]
pub struct FinderItem {
    /// Display label.
    pub label: String,
    /// Secondary label (path, context).
    pub description: Option<String>,
    /// Location to jump to.
    pub location: FinderLocation,
    /// Match score for ranking.
    pub score: i32,
    /// Preview hint (for file preview).
    pub preview_hint: Option<String>,
    /// Source type.
    pub source: FinderSource,
}

/// Action to take on item selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinderAction {
    /// Open in current window.
    Open,
    /// Open in horizontal split.
    Split,
    /// Open in vertical split.
    VSplit,
    /// Open in new tab.
    Tab,
    /// Reveal in explorer.
    Reveal,
}

/// Finder query state.
#[derive(Debug, Clone, Default)]
pub struct FinderQuery {
    /// Query text.
    pub text: String,
    /// Limit results.
    pub limit: usize,
    /// Source filter.
    pub source: Option<FinderSource>,
}

impl FinderQuery {
    /// Create a new query.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            limit: 100,
            source: None,
        }
    }

    /// With a limit.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// With a source filter.
    pub fn with_source(mut self, source: FinderSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Check if query is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

/// Finder state.
#[derive(Debug, Default)]
pub struct Finder {
    /// Current query.
    query: FinderQuery,
    /// Filtered results.
    items: Vec<FinderItem>,
    /// Currently selected index.
    cursor: usize,
    /// Whether finder is visible.
    visible: bool,
    /// Current source mode.
    source: FinderSource,
    /// Loading state.
    loading: bool,
    /// Total results (may be more than displayed).
    total_count: usize,
}

impl Finder {
    /// Create a new finder.
    pub fn new() -> Self {
        Self {
            source: FinderSource::Files,
            ..Default::default()
        }
    }

    /// Show the finder with a specific source.
    pub fn show(&mut self, source: FinderSource) {
        self.visible = true;
        self.source = source;
        self.query = FinderQuery::default();
        self.items.clear();
        self.cursor = 0;
        self.loading = false;
    }

    /// Hide the finder.
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get current source.
    pub fn source(&self) -> FinderSource {
        self.source
    }

    /// Set query text.
    pub fn set_query(&mut self, text: String) {
        self.query.text = text;
        self.cursor = 0;
    }

    /// Get query text.
    pub fn query(&self) -> &str {
        &self.query.text
    }

    /// Set results.
    pub fn set_items(&mut self, items: Vec<FinderItem>, total: usize) {
        self.items = items;
        self.total_count = total;
        self.loading = false;
        if self.cursor >= self.items.len() && !self.items.is_empty() {
            self.cursor = self.items.len() - 1;
        }
    }

    /// Get current items.
    pub fn items(&self) -> &[FinderItem] {
        &self.items
    }

    /// Get selected item.
    pub fn selected(&self) -> Option<&FinderItem> {
        self.items.get(self.cursor)
    }

    /// Get cursor position.
    pub fn cursor_pos(&self) -> usize {
        self.cursor
    }

    /// Move cursor up.
    pub fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor down.
    pub fn move_down(&mut self) {
        if !self.items.is_empty() && self.cursor < self.items.len() - 1 {
            self.cursor += 1;
        }
    }

    /// Move cursor to top.
    pub fn move_to_top(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to bottom.
    pub fn move_to_bottom(&mut self) {
        if !self.items.is_empty() {
            self.cursor = self.items.len() - 1;
        }
    }

    /// Set loading state.
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    /// Check if loading.
    pub fn is_loading(&self) -> bool {
        self.loading
    }

    /// Get total count.
    pub fn total_count(&self) -> usize {
        self.total_count
    }

    /// Insert character into query.
    pub fn insert_char(&mut self, c: char) {
        self.query.text.push(c);
        self.cursor = 0;
    }

    /// Delete last character from query.
    pub fn delete_char(&mut self) {
        self.query.text.pop();
        self.cursor = 0;
    }

    /// Clear query.
    pub fn clear_query(&mut self) {
        self.query.text.clear();
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finder_new() {
        let finder = Finder::new();
        assert!(!finder.is_visible());
        assert_eq!(finder.source(), FinderSource::Files);
    }

    #[test]
    fn test_finder_show_hide() {
        let mut finder = Finder::new();
        finder.show(FinderSource::Buffers);
        assert!(finder.is_visible());
        assert_eq!(finder.source(), FinderSource::Buffers);

        finder.hide();
        assert!(!finder.is_visible());
    }

    #[test]
    fn test_finder_query() {
        let mut finder = Finder::new();
        finder.set_query("test".to_string());
        assert_eq!(finder.query(), "test");
    }

    #[test]
    fn test_finder_insert_char() {
        let mut finder = Finder::new();
        finder.insert_char('a');
        finder.insert_char('b');
        assert_eq!(finder.query(), "ab");
    }

    #[test]
    fn test_finder_delete_char() {
        let mut finder = Finder::new();
        finder.set_query("abc".to_string());
        finder.delete_char();
        assert_eq!(finder.query(), "ab");
    }

    #[test]
    fn test_finder_clear_query() {
        let mut finder = Finder::new();
        finder.set_query("test".to_string());
        finder.clear_query();
        assert!(finder.query().is_empty());
    }

    #[test]
    fn test_finder_items() {
        let mut finder = Finder::new();
        let items = vec![
            FinderItem {
                label: "main.rs".to_string(),
                description: Some("/project/src".to_string()),
                location: FinderLocation::File(PathBuf::from("/project/src/main.rs")),
                score: 100,
                preview_hint: None,
                source: FinderSource::Files,
            },
            FinderItem {
                label: "lib.rs".to_string(),
                description: Some("/project/src".to_string()),
                location: FinderLocation::File(PathBuf::from("/project/src/lib.rs")),
                score: 90,
                preview_hint: None,
                source: FinderSource::Files,
            },
        ];
        finder.set_items(items, 2);
        assert_eq!(finder.items().len(), 2);
        assert_eq!(finder.total_count(), 2);
    }

    #[test]
    fn test_finder_selected() {
        let mut finder = Finder::new();
        let items = vec![FinderItem {
            label: "test.rs".to_string(),
            description: None,
            location: FinderLocation::File(PathBuf::from("/test.rs")),
            score: 100,
            preview_hint: None,
            source: FinderSource::Files,
        }];
        finder.set_items(items, 1);
        assert!(finder.selected().is_some());
        assert_eq!(finder.selected().unwrap().label, "test.rs");
    }

    #[test]
    fn test_finder_navigation() {
        let mut finder = Finder::new();
        let items: Vec<_> = (0..5)
            .map(|i| FinderItem {
                label: format!("file{}.rs", i),
                description: None,
                location: FinderLocation::File(PathBuf::from(format!("/file{}.rs", i))),
                score: 100 - i as i32,
                preview_hint: None,
                source: FinderSource::Files,
            })
            .collect();
        finder.set_items(items, 5);

        assert_eq!(finder.cursor_pos(), 0);
        finder.move_down();
        assert_eq!(finder.cursor_pos(), 1);
        finder.move_down();
        assert_eq!(finder.cursor_pos(), 2);
        finder.move_up();
        assert_eq!(finder.cursor_pos(), 1);
        finder.move_to_bottom();
        assert_eq!(finder.cursor_pos(), 4);
        finder.move_to_top();
        assert_eq!(finder.cursor_pos(), 0);
    }

    #[test]
    fn test_finder_loading() {
        let mut finder = Finder::new();
        assert!(!finder.is_loading());
        finder.set_loading(true);
        assert!(finder.is_loading());
    }

    #[test]
    fn test_finder_query_builder() {
        let query = FinderQuery::new("test")
            .with_limit(50)
            .with_source(FinderSource::Grep);
        assert_eq!(query.text, "test");
        assert_eq!(query.limit, 50);
        assert_eq!(query.source, Some(FinderSource::Grep));
    }

    #[test]
    fn test_finder_location_variants() {
        let loc1 = FinderLocation::File(PathBuf::from("/a.rs"));
        let loc2 = FinderLocation::FileLine {
            path: PathBuf::from("/b.rs"),
            line: 10,
        };
        let loc3 = FinderLocation::FilePosition {
            path: PathBuf::from("/c.rs"),
            line: 5,
            col: 3,
        };
        let loc4 = FinderLocation::Buffer(1);
        let loc5 = FinderLocation::Command("quit".to_string());

        assert_eq!(loc1, loc1.clone());
        assert_eq!(loc2, loc2.clone());
        assert_eq!(loc3, loc3.clone());
        assert_eq!(loc4, loc4.clone());
        assert_eq!(loc5, loc5.clone());
    }

    #[test]
    fn test_finder_source_variants() {
        let sources = vec![
            FinderSource::Files,
            FinderSource::Buffers,
            FinderSource::Recent,
            FinderSource::Symbols,
            FinderSource::Grep,
            FinderSource::Branches,
            FinderSource::Commands,
        ];
        assert_eq!(sources.len(), 7);
    }

    #[test]
    fn test_finder_action_variants() {
        let actions = vec![
            FinderAction::Open,
            FinderAction::Split,
            FinderAction::VSplit,
            FinderAction::Tab,
            FinderAction::Reveal,
        ];
        assert_eq!(actions.len(), 5);
    }
}
