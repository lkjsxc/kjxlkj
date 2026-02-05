//! Insert mode completion.
//!
//! Provides completion functionality for Insert mode including keyword,
//! line, file path, and LSP completion.

use std::collections::HashSet;

/// Type of completion being performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertCompletionType {
    /// Keyword completion (Ctrl-n, Ctrl-p) - words from buffer.
    Keyword,
    /// Line completion (Ctrl-x Ctrl-l) - complete entire lines.
    Line,
    /// File path completion (Ctrl-x Ctrl-f) - filesystem paths.
    FilePath,
    /// Dictionary completion (Ctrl-x Ctrl-k).
    Dictionary,
    /// Thesaurus completion (Ctrl-x Ctrl-t).
    Thesaurus,
    /// Tag completion (Ctrl-x Ctrl-]).
    Tag,
    /// Include completion (Ctrl-x Ctrl-i) - from included files.
    Include,
    /// Definition completion (Ctrl-x Ctrl-d).
    Definition,
    /// Vim command completion (Ctrl-x Ctrl-v).
    Command,
    /// Omni completion (Ctrl-x Ctrl-o) - language-aware.
    Omni,
    /// User completion (Ctrl-x Ctrl-u) - custom function.
    User,
    /// Spell completion (Ctrl-x s).
    Spell,
}

impl InsertCompletionType {
    /// Get the key sequence that triggers this completion type.
    pub fn trigger_keys(&self) -> &'static str {
        match self {
            Self::Keyword => "<C-n> or <C-p>",
            Self::Line => "<C-x><C-l>",
            Self::FilePath => "<C-x><C-f>",
            Self::Dictionary => "<C-x><C-k>",
            Self::Thesaurus => "<C-x><C-t>",
            Self::Tag => "<C-x><C-]>",
            Self::Include => "<C-x><C-i>",
            Self::Definition => "<C-x><C-d>",
            Self::Command => "<C-x><C-v>",
            Self::Omni => "<C-x><C-o>",
            Self::User => "<C-x><C-u>",
            Self::Spell => "<C-x>s",
        }
    }
}

/// A single completion item.
#[derive(Debug, Clone)]
pub struct InsertCompletionItem {
    /// The completion text to insert.
    pub word: String,
    /// Optional abbreviated display text.
    pub abbr: Option<String>,
    /// Optional menu info (shown in popup).
    pub menu: Option<String>,
    /// Optional detailed info (shown in preview).
    pub info: Option<String>,
    /// Kind of completion (variable, function, etc.).
    pub kind: Option<String>,
    /// Whether this item is preselected.
    pub preselect: bool,
    /// Sort priority (lower is higher priority).
    pub priority: i32,
}

impl InsertCompletionItem {
    /// Create a simple completion item.
    pub fn new(word: impl Into<String>) -> Self {
        Self {
            word: word.into(),
            abbr: None,
            menu: None,
            info: None,
            kind: None,
            preselect: false,
            priority: 0,
        }
    }

    /// Set abbreviated display text.
    pub fn with_abbr(mut self, abbr: impl Into<String>) -> Self {
        self.abbr = Some(abbr.into());
        self
    }

    /// Set menu info.
    pub fn with_menu(mut self, menu: impl Into<String>) -> Self {
        self.menu = Some(menu.into());
        self
    }

    /// Set kind.
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    /// Get display text (abbr or word).
    pub fn display(&self) -> &str {
        self.abbr.as_deref().unwrap_or(&self.word)
    }
}

/// State of insert mode completion.
#[derive(Debug, Clone)]
pub struct InsertCompletionState {
    /// Type of completion active.
    completion_type: InsertCompletionType,
    /// Available completion items.
    items: Vec<InsertCompletionItem>,
    /// Currently selected index.
    selected: usize,
    /// Original text before completion started.
    original: String,
    /// Whether completion is active.
    active: bool,
    /// Start position of completion in line.
    start_col: usize,
}

impl InsertCompletionState {
    /// Create new completion state.
    pub fn new(completion_type: InsertCompletionType, original: String, start_col: usize) -> Self {
        Self {
            completion_type,
            items: Vec::new(),
            selected: 0,
            original,
            active: true,
            start_col,
        }
    }

    /// Set completion items.
    pub fn set_items(&mut self, items: Vec<InsertCompletionItem>) {
        self.items = items;
        self.selected = 0;
    }

    /// Get the completion type.
    pub fn completion_type(&self) -> InsertCompletionType {
        self.completion_type
    }

    /// Get current items.
    pub fn items(&self) -> &[InsertCompletionItem] {
        &self.items
    }

    /// Get currently selected item.
    pub fn current(&self) -> Option<&InsertCompletionItem> {
        self.items.get(self.selected)
    }

    /// Get selected index.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Move to next item.
    pub fn move_next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    /// Move to previous item.
    pub fn move_prev(&mut self) {
        if !self.items.is_empty() {
            self.selected = if self.selected == 0 {
                self.items.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Page down (move by page_size items).
    pub fn page_down(&mut self, page_size: usize) {
        if !self.items.is_empty() {
            self.selected = (self.selected + page_size).min(self.items.len() - 1);
        }
    }

    /// Page up (move by page_size items).
    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    /// Get original text.
    pub fn original(&self) -> &str {
        &self.original
    }

    /// Get start column.
    pub fn start_col(&self) -> usize {
        self.start_col
    }

    /// Check if completion is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Cancel completion.
    pub fn cancel(&mut self) {
        self.active = false;
    }

    /// Accept current selection.
    pub fn accept(&self) -> Option<&str> {
        if self.active {
            self.current().map(|item| item.word.as_str())
        } else {
            None
        }
    }
}

/// Keyword completion provider.
#[derive(Debug, Default)]
pub struct KeywordCompleter {
    /// Minimum word length to include.
    min_word_length: usize,
}

impl KeywordCompleter {
    /// Create with default settings.
    pub fn new() -> Self {
        Self {
            min_word_length: 2,
        }
    }

    /// Set minimum word length.
    pub fn with_min_length(mut self, len: usize) -> Self {
        self.min_word_length = len;
        self
    }

    /// Extract keywords from buffer content.
    pub fn complete(&self, content: &str, prefix: &str) -> Vec<InsertCompletionItem> {
        let mut seen = HashSet::new();
        let mut items = Vec::new();
        let prefix_lower = prefix.to_lowercase();

        for word in content.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if word.len() >= self.min_word_length
                && word.to_lowercase().starts_with(&prefix_lower)
                && word != prefix
                && !seen.contains(word)
            {
                seen.insert(word.to_string());
                items.push(InsertCompletionItem::new(word));
            }
        }

        // Sort by frequency (appearance order for now)
        items
    }
}

/// Line completion provider.
#[derive(Debug, Default)]
pub struct LineCompleter;

impl LineCompleter {
    /// Create new line completer.
    pub fn new() -> Self {
        Self
    }

    /// Complete lines from buffer content.
    pub fn complete(&self, content: &str, prefix: &str) -> Vec<InsertCompletionItem> {
        let mut items = Vec::new();
        let prefix_lower = prefix.to_lowercase();

        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && trimmed.to_lowercase().starts_with(&prefix_lower)
                && trimmed != prefix
            {
                items.push(InsertCompletionItem::new(trimmed));
            }
        }

        items
    }
}

/// File path completion provider.
#[derive(Debug, Default)]
pub struct FilePathCompleter;

impl FilePathCompleter {
    /// Create new file path completer.
    pub fn new() -> Self {
        Self
    }

    /// Complete file paths.
    pub fn complete(&self, prefix: &str) -> Vec<InsertCompletionItem> {
        let mut items = Vec::new();

        // Expand ~ to home directory
        let expanded = if let Some(rest) = prefix.strip_prefix('~') {
            if let Some(home) = dirs_next::home_dir() {
                home.to_string_lossy().to_string() + rest
            } else {
                prefix.to_string()
            }
        } else {
            prefix.to_string()
        };

        // Find directory and file prefix
        let (dir, file_prefix) = if let Some(pos) = expanded.rfind('/') {
            (&expanded[..=pos], &expanded[pos + 1..])
        } else {
            ("./", expanded.as_str())
        };

        // Read directory
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(file_prefix) {
                        let mut path = format!("{}{}", dir, name);
                        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            path.push('/');
                        }
                        items.push(
                            InsertCompletionItem::new(&path)
                                .with_kind(if path.ends_with('/') { "dir" } else { "file" }),
                        );
                    }
                }
            }
        }

        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_type_triggers() {
        assert!(!InsertCompletionType::Keyword.trigger_keys().is_empty());
        assert!(!InsertCompletionType::FilePath.trigger_keys().is_empty());
    }

    #[test]
    fn test_completion_item_new() {
        let item = InsertCompletionItem::new("test");
        assert_eq!(item.word, "test");
        assert_eq!(item.display(), "test");
    }

    #[test]
    fn test_completion_item_with_abbr() {
        let item = InsertCompletionItem::new("testing").with_abbr("test");
        assert_eq!(item.display(), "test");
    }

    #[test]
    fn test_completion_state_navigation() {
        let mut state = InsertCompletionState::new(
            InsertCompletionType::Keyword,
            "te".to_string(),
            0,
        );
        state.set_items(vec![
            InsertCompletionItem::new("test"),
            InsertCompletionItem::new("testing"),
            InsertCompletionItem::new("tested"),
        ]);

        assert_eq!(state.selected(), 0);
        assert_eq!(state.current().map(|i| i.word.as_str()), Some("test"));

        state.move_next();
        assert_eq!(state.selected(), 1);
        assert_eq!(state.current().map(|i| i.word.as_str()), Some("testing"));

        state.move_next();
        assert_eq!(state.selected(), 2);

        state.move_next();
        assert_eq!(state.selected(), 0); // Wraps around

        state.move_prev();
        assert_eq!(state.selected(), 2); // Wraps around backwards
    }

    #[test]
    fn test_completion_state_paging() {
        let mut state = InsertCompletionState::new(
            InsertCompletionType::Keyword,
            "".to_string(),
            0,
        );
        let items: Vec<_> = (0..20).map(|i| InsertCompletionItem::new(format!("item{}", i))).collect();
        state.set_items(items);

        state.page_down(5);
        assert_eq!(state.selected(), 5);

        state.page_down(5);
        assert_eq!(state.selected(), 10);

        state.page_up(3);
        assert_eq!(state.selected(), 7);

        state.page_up(10);
        assert_eq!(state.selected(), 0);
    }

    #[test]
    fn test_completion_state_accept() {
        let mut state = InsertCompletionState::new(
            InsertCompletionType::Keyword,
            "te".to_string(),
            0,
        );
        state.set_items(vec![InsertCompletionItem::new("test")]);

        assert_eq!(state.accept(), Some("test"));

        state.cancel();
        assert_eq!(state.accept(), None);
    }

    #[test]
    fn test_keyword_completer() {
        let completer = KeywordCompleter::new();
        let content = "hello world hello_world testing test tested";
        let items = completer.complete(content, "te");

        let words: Vec<_> = items.iter().map(|i| i.word.as_str()).collect();
        assert!(words.contains(&"testing"));
        assert!(words.contains(&"test"));
        assert!(words.contains(&"tested"));
        assert!(!words.contains(&"hello")); // Doesn't start with "te"
    }

    #[test]
    fn test_keyword_completer_case_insensitive() {
        let completer = KeywordCompleter::new();
        let content = "Hello HELLO hElLo";
        let items = completer.complete(content, "he");

        assert!(!items.is_empty());
    }

    #[test]
    fn test_keyword_completer_min_length() {
        let completer = KeywordCompleter::new().with_min_length(4);
        let content = "ab abc abcd abcde";
        let items = completer.complete(content, "ab");

        let words: Vec<_> = items.iter().map(|i| i.word.as_str()).collect();
        assert!(words.contains(&"abcd"));
        assert!(words.contains(&"abcde"));
        assert!(!words.contains(&"abc")); // Too short
    }

    #[test]
    fn test_line_completer() {
        let completer = LineCompleter::new();
        let content = "hello world\ntesting line\ntest another";
        let items = completer.complete(content, "te");

        let lines: Vec<_> = items.iter().map(|i| i.word.as_str()).collect();
        assert!(lines.contains(&"testing line"));
        assert!(lines.contains(&"test another"));
        assert!(!lines.contains(&"hello world"));
    }
}
