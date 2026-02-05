//! Quickfix and location list model.
//!
//! Implements quickfix list functionality as specified in
//! `/docs/spec/features/navigation/quickfix.md`.

use std::path::PathBuf;

/// Type of quickfix entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuickfixType {
    /// Error.
    #[default]
    Error,
    /// Warning.
    Warning,
    /// Info.
    Info,
    /// Hint.
    Hint,
    /// Search result.
    Search,
}

/// A single entry in the quickfix/location list.
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub col: usize,
    /// Entry type.
    pub entry_type: QuickfixType,
    /// Description text.
    pub text: String,
    /// Optional error code.
    pub code: Option<String>,
}

impl QuickfixEntry {
    /// Create a new quickfix entry.
    pub fn new(path: PathBuf, line: usize, col: usize, text: impl Into<String>) -> Self {
        Self {
            path,
            line,
            col,
            entry_type: QuickfixType::Error,
            text: text.into(),
            code: None,
        }
    }

    /// With entry type.
    pub fn with_type(mut self, entry_type: QuickfixType) -> Self {
        self.entry_type = entry_type;
        self
    }

    /// With code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Format as display string.
    pub fn display(&self) -> String {
        let filename = self.path.file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| self.path.to_string_lossy().to_string());
        
        let type_char = match self.entry_type {
            QuickfixType::Error => 'E',
            QuickfixType::Warning => 'W',
            QuickfixType::Info => 'I',
            QuickfixType::Hint => 'H',
            QuickfixType::Search => 'S',
        };
        
        format!("{}:{}:{} {} {}", filename, self.line, self.col, type_char, self.text)
    }
}

/// Quickfix list state.
#[derive(Debug, Default)]
pub struct QuickfixList {
    /// List entries.
    entries: Vec<QuickfixEntry>,
    /// Current position.
    cursor: usize,
    /// Title for the list.
    title: String,
}

impl QuickfixList {
    /// Create a new quickfix list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with title.
    pub fn with_title(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Get list title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set entries.
    pub fn set_entries(&mut self, entries: Vec<QuickfixEntry>) {
        self.entries = entries;
        self.cursor = 0;
    }

    /// Add an entry.
    pub fn add(&mut self, entry: QuickfixEntry) {
        self.entries.push(entry);
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.cursor = 0;
    }

    /// Get entries.
    pub fn entries(&self) -> &[QuickfixEntry] {
        &self.entries
    }

    /// Get current entry.
    pub fn current(&self) -> Option<&QuickfixEntry> {
        self.entries.get(self.cursor)
    }

    /// Get cursor position.
    pub fn cursor_pos(&self) -> usize {
        self.cursor
    }

    /// Move to next entry.
    pub fn next_entry(&mut self) -> Option<&QuickfixEntry> {
        if !self.entries.is_empty() && self.cursor < self.entries.len() - 1 {
            self.cursor += 1;
        }
        self.current()
    }

    /// Move to previous entry.
    pub fn prev_entry(&mut self) -> Option<&QuickfixEntry> {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        self.current()
    }

    /// Move to first entry.
    pub fn first(&mut self) -> Option<&QuickfixEntry> {
        self.cursor = 0;
        self.current()
    }

    /// Move to last entry.
    pub fn last(&mut self) -> Option<&QuickfixEntry> {
        if !self.entries.is_empty() {
            self.cursor = self.entries.len() - 1;
        }
        self.current()
    }

    /// Move to specific index.
    pub fn goto(&mut self, index: usize) -> Option<&QuickfixEntry> {
        if index < self.entries.len() {
            self.cursor = index;
        }
        self.current()
    }

    /// Get count of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Quickfix manager holding both global quickfix and per-window location lists.
#[derive(Debug, Default)]
pub struct QuickfixManager {
    /// Global quickfix list.
    quickfix: QuickfixList,
    /// Location lists by window ID.
    location_lists: std::collections::HashMap<u64, QuickfixList>,
    /// Whether quickfix window is visible.
    quickfix_visible: bool,
    /// Whether location list window is visible (per window).
    location_visible: std::collections::HashMap<u64, bool>,
}

impl QuickfixManager {
    /// Create a new quickfix manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the global quickfix list.
    pub fn quickfix(&self) -> &QuickfixList {
        &self.quickfix
    }

    /// Get mutable global quickfix list.
    pub fn quickfix_mut(&mut self) -> &mut QuickfixList {
        &mut self.quickfix
    }

    /// Get location list for a window.
    pub fn location(&self, window_id: u64) -> Option<&QuickfixList> {
        self.location_lists.get(&window_id)
    }

    /// Get mutable location list for a window.
    pub fn location_mut(&mut self, window_id: u64) -> &mut QuickfixList {
        self.location_lists.entry(window_id).or_default()
    }

    /// Toggle quickfix window visibility.
    pub fn toggle_quickfix(&mut self) {
        self.quickfix_visible = !self.quickfix_visible;
    }

    /// Open quickfix window.
    pub fn open_quickfix(&mut self) {
        self.quickfix_visible = true;
    }

    /// Close quickfix window.
    pub fn close_quickfix(&mut self) {
        self.quickfix_visible = false;
    }

    /// Check if quickfix is visible.
    pub fn is_quickfix_visible(&self) -> bool {
        self.quickfix_visible
    }

    /// Toggle location list visibility for a window.
    pub fn toggle_location(&mut self, window_id: u64) {
        let visible = self.location_visible.entry(window_id).or_insert(false);
        *visible = !*visible;
    }

    /// Open location list for a window.
    pub fn open_location(&mut self, window_id: u64) {
        self.location_visible.insert(window_id, true);
    }

    /// Close location list for a window.
    pub fn close_location(&mut self, window_id: u64) {
        self.location_visible.insert(window_id, false);
    }

    /// Check if location list is visible for a window.
    pub fn is_location_visible(&self, window_id: u64) -> bool {
        self.location_visible.get(&window_id).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickfix_type_default() {
        assert_eq!(QuickfixType::default(), QuickfixType::Error);
    }

    #[test]
    fn test_quickfix_entry_new() {
        let entry = QuickfixEntry::new(
            PathBuf::from("/test/file.rs"),
            10,
            5,
            "error message",
        );
        assert_eq!(entry.line, 10);
        assert_eq!(entry.col, 5);
        assert_eq!(entry.text, "error message");
    }

    #[test]
    fn test_quickfix_entry_with_type() {
        let entry = QuickfixEntry::new(PathBuf::from("/test.rs"), 1, 1, "warning")
            .with_type(QuickfixType::Warning);
        assert_eq!(entry.entry_type, QuickfixType::Warning);
    }

    #[test]
    fn test_quickfix_entry_with_code() {
        let entry = QuickfixEntry::new(PathBuf::from("/test.rs"), 1, 1, "error")
            .with_code("E0001");
        assert_eq!(entry.code, Some("E0001".to_string()));
    }

    #[test]
    fn test_quickfix_entry_display() {
        let entry = QuickfixEntry::new(PathBuf::from("/src/main.rs"), 42, 10, "unused variable");
        let display = entry.display();
        assert!(display.contains("main.rs"));
        assert!(display.contains("42"));
        assert!(display.contains("10"));
        assert!(display.contains("E")); // Error type
        assert!(display.contains("unused variable"));
    }

    #[test]
    fn test_quickfix_list_new() {
        let list = QuickfixList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_quickfix_list_with_title() {
        let list = QuickfixList::with_title("Compiler Errors");
        assert_eq!(list.title(), "Compiler Errors");
    }

    #[test]
    fn test_quickfix_list_add() {
        let mut list = QuickfixList::new();
        list.add(QuickfixEntry::new(PathBuf::from("/a.rs"), 1, 1, "error"));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_quickfix_list_set_entries() {
        let mut list = QuickfixList::new();
        let entries = vec![
            QuickfixEntry::new(PathBuf::from("/a.rs"), 1, 1, "error1"),
            QuickfixEntry::new(PathBuf::from("/b.rs"), 2, 1, "error2"),
        ];
        list.set_entries(entries);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_quickfix_list_navigation() {
        let mut list = QuickfixList::new();
        let entries = vec![
            QuickfixEntry::new(PathBuf::from("/a.rs"), 1, 1, "error1"),
            QuickfixEntry::new(PathBuf::from("/b.rs"), 2, 1, "error2"),
            QuickfixEntry::new(PathBuf::from("/c.rs"), 3, 1, "error3"),
        ];
        list.set_entries(entries);

        assert_eq!(list.cursor_pos(), 0);
        assert_eq!(list.current().unwrap().line, 1);

        list.next_entry();
        assert_eq!(list.cursor_pos(), 1);
        assert_eq!(list.current().unwrap().line, 2);

        list.prev_entry();
        assert_eq!(list.cursor_pos(), 0);

        list.last();
        assert_eq!(list.cursor_pos(), 2);

        list.first();
        assert_eq!(list.cursor_pos(), 0);
    }

    #[test]
    fn test_quickfix_list_goto() {
        let mut list = QuickfixList::new();
        let entries = vec![
            QuickfixEntry::new(PathBuf::from("/a.rs"), 1, 1, "error1"),
            QuickfixEntry::new(PathBuf::from("/b.rs"), 2, 1, "error2"),
        ];
        list.set_entries(entries);

        list.goto(1);
        assert_eq!(list.cursor_pos(), 1);

        // Out of bounds doesn't change
        list.goto(100);
        assert_eq!(list.cursor_pos(), 1);
    }

    #[test]
    fn test_quickfix_list_clear() {
        let mut list = QuickfixList::new();
        list.add(QuickfixEntry::new(PathBuf::from("/a.rs"), 1, 1, "error"));
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_quickfix_manager_new() {
        let manager = QuickfixManager::new();
        assert!(!manager.is_quickfix_visible());
    }

    #[test]
    fn test_quickfix_manager_quickfix() {
        let mut manager = QuickfixManager::new();
        manager.quickfix_mut().add(QuickfixEntry::new(
            PathBuf::from("/test.rs"),
            1,
            1,
            "error",
        ));
        assert_eq!(manager.quickfix().len(), 1);
    }

    #[test]
    fn test_quickfix_manager_location() {
        let mut manager = QuickfixManager::new();
        manager.location_mut(1).add(QuickfixEntry::new(
            PathBuf::from("/test.rs"),
            1,
            1,
            "warning",
        ));
        assert_eq!(manager.location(1).unwrap().len(), 1);
        assert!(manager.location(2).is_none());
    }

    #[test]
    fn test_quickfix_manager_visibility() {
        let mut manager = QuickfixManager::new();

        assert!(!manager.is_quickfix_visible());
        manager.open_quickfix();
        assert!(manager.is_quickfix_visible());
        manager.close_quickfix();
        assert!(!manager.is_quickfix_visible());
        manager.toggle_quickfix();
        assert!(manager.is_quickfix_visible());
    }

    #[test]
    fn test_quickfix_manager_location_visibility() {
        let mut manager = QuickfixManager::new();
        let window_id = 1;

        assert!(!manager.is_location_visible(window_id));
        manager.open_location(window_id);
        assert!(manager.is_location_visible(window_id));
        manager.close_location(window_id);
        assert!(!manager.is_location_visible(window_id));
        manager.toggle_location(window_id);
        assert!(manager.is_location_visible(window_id));
    }

    #[test]
    fn test_quickfix_entry_types() {
        let types = vec![
            QuickfixType::Error,
            QuickfixType::Warning,
            QuickfixType::Info,
            QuickfixType::Hint,
            QuickfixType::Search,
        ];
        assert_eq!(types.len(), 5);
    }
}
