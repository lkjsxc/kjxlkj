//! Command-line completion system.
//!
//! Implements completion providers for command-line mode as specified in
//! `/docs/spec/scripting/cmdline-completion.md`.

use std::path::PathBuf;

/// The type of completion being requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionType {
    /// File paths.
    File,
    /// Directory paths.
    Dir,
    /// Buffer names.
    Buffer,
    /// Command names.
    Command,
    /// Option names.
    Option,
    /// Colorscheme names.
    Color,
    /// Highlight group names.
    Highlight,
    /// Autocommand event names.
    Event,
    /// Help topics.
    Help,
    /// Filetypes.
    Filetype,
    /// Shell commands.
    ShellCmd,
    /// Custom completion.
    Custom,
}

/// A completion item returned by a provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    /// The text to insert.
    pub text: String,
    /// Optional display text (if different from insert text).
    pub display: Option<String>,
    /// Optional info/documentation.
    pub info: Option<String>,
}

impl CompletionItem {
    /// Create a simple completion item.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            display: None,
            info: None,
        }
    }

    /// Create a completion item with display text.
    pub fn with_display(text: impl Into<String>, display: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            display: Some(display.into()),
            info: None,
        }
    }
}

/// Wildmode option for completion behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WildMode {
    /// Complete to longest common match.
    #[default]
    Longest,
    /// Complete to next full match.
    Full,
    /// Complete to longest, then cycle full.
    LongestFull,
    /// Show list of matches.
    List,
    /// List, then cycle full.
    ListFull,
}

/// Completion state for command mode.
#[derive(Debug, Clone)]
pub struct CompletionState {
    /// Current completions.
    items: Vec<CompletionItem>,
    /// Current selection index.
    index: Option<usize>,
    /// Original text before completion started.
    original: String,
    /// Completion prefix position in the command line.
    prefix_pos: usize,
}

impl CompletionState {
    /// Create a new completion state.
    pub fn new(original: String, prefix_pos: usize) -> Self {
        Self {
            items: Vec::new(),
            index: None,
            original,
            prefix_pos,
        }
    }

    /// Set the completion items.
    pub fn set_items(&mut self, items: Vec<CompletionItem>) {
        self.items = items;
        self.index = None;
    }

    /// Get the current items.
    pub fn items(&self) -> &[CompletionItem] {
        &self.items
    }

    /// Get the current selection index.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Get the original text.
    pub fn original(&self) -> &str {
        &self.original
    }

    /// Get the prefix position.
    pub fn prefix_pos(&self) -> usize {
        self.prefix_pos
    }

    /// Move to the next completion.
    pub fn move_next(&mut self) -> Option<&CompletionItem> {
        if self.items.is_empty() {
            return None;
        }
        self.index = Some(match self.index {
            None => 0,
            Some(i) => (i + 1) % self.items.len(),
        });
        self.index.map(|i| &self.items[i])
    }

    /// Move to the previous completion.
    pub fn move_prev(&mut self) -> Option<&CompletionItem> {
        if self.items.is_empty() {
            return None;
        }
        self.index = Some(match self.index {
            None => self.items.len().saturating_sub(1),
            Some(0) => self.items.len().saturating_sub(1),
            Some(i) => i - 1,
        });
        self.index.map(|i| &self.items[i])
    }

    /// Get the currently selected item.
    pub fn current(&self) -> Option<&CompletionItem> {
        self.index.map(|i| &self.items[i])
    }

    /// Accept the current completion.
    pub fn accept(&self) -> Option<String> {
        self.current().map(|item| item.text.clone())
    }

    /// Cancel and return the original.
    pub fn cancel(&self) -> &str {
        &self.original
    }

    /// Check if there are any completions.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of completions.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

/// A completion provider trait.
pub trait CompletionProvider: Send + Sync {
    /// Get completions for the given prefix.
    fn complete(&self, prefix: &str) -> Vec<CompletionItem>;

    /// Get the completion type this provider handles.
    fn completion_type(&self) -> CompletionType;
}

/// Built-in command completion provider.
#[derive(Debug, Default)]
pub struct CommandCompletionProvider {
    commands: Vec<String>,
}

impl CommandCompletionProvider {
    /// Create a new command completion provider.
    pub fn new() -> Self {
        Self {
            commands: vec![
                "quit".to_string(),
                "q".to_string(),
                "write".to_string(),
                "w".to_string(),
                "wq".to_string(),
                "edit".to_string(),
                "e".to_string(),
                "buffer".to_string(),
                "b".to_string(),
                "bnext".to_string(),
                "bn".to_string(),
                "bprev".to_string(),
                "bp".to_string(),
                "split".to_string(),
                "sp".to_string(),
                "vsplit".to_string(),
                "vs".to_string(),
                "new".to_string(),
                "vnew".to_string(),
                "close".to_string(),
                "only".to_string(),
                "set".to_string(),
                "help".to_string(),
                "h".to_string(),
                "source".to_string(),
                "so".to_string(),
            ],
        }
    }
}

impl CompletionProvider for CommandCompletionProvider {
    fn complete(&self, prefix: &str) -> Vec<CompletionItem> {
        self.commands
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| CompletionItem::new(cmd.clone()))
            .collect()
    }

    fn completion_type(&self) -> CompletionType {
        CompletionType::Command
    }
}

/// File completion provider.
#[derive(Debug, Default)]
pub struct FileCompletionProvider {
    /// Whether to show hidden files.
    show_hidden: bool,
    /// Patterns to ignore.
    ignore_patterns: Vec<String>,
}

impl FileCompletionProvider {
    /// Create a new file completion provider.
    pub fn new() -> Self {
        Self {
            show_hidden: false,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
            ],
        }
    }

    /// Set whether to show hidden files.
    pub fn set_show_hidden(&mut self, show: bool) {
        self.show_hidden = show;
    }
}

impl CompletionProvider for FileCompletionProvider {
    fn complete(&self, prefix: &str) -> Vec<CompletionItem> {
        let path = if prefix.is_empty() {
            PathBuf::from(".")
        } else if prefix.starts_with('~') {
            // Expand home directory
            if let Some(home) = dirs_next::home_dir() {
                if prefix == "~" {
                    home
                } else {
                    home.join(&prefix[2..])
                }
            } else {
                PathBuf::from(prefix)
            }
        } else {
            PathBuf::from(prefix)
        };

        let (dir, file_prefix) = if path.is_dir() {
            (path, String::new())
        } else {
            let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
            let file = path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            (parent.to_path_buf(), file)
        };

        let mut items = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden files unless enabled
                if !self.show_hidden && name.starts_with('.') {
                    continue;
                }

                // Skip ignored patterns
                if self.ignore_patterns.contains(&name) {
                    continue;
                }

                // Filter by prefix
                if !name.starts_with(&file_prefix) {
                    continue;
                }

                let full_path = if prefix.is_empty() || dir.as_os_str() == "." {
                    name.clone()
                } else {
                    dir.join(&name).to_string_lossy().to_string()
                };

                let display = if entry.path().is_dir() {
                    format!("{}/", name)
                } else {
                    name
                };

                items.push(CompletionItem::with_display(full_path, display));
            }
        }

        items.sort_by(|a, b| a.text.cmp(&b.text));
        items
    }

    fn completion_type(&self) -> CompletionType {
        CompletionType::File
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_item_new() {
        let item = CompletionItem::new("test");
        assert_eq!(item.text, "test");
        assert!(item.display.is_none());
        assert!(item.info.is_none());
    }

    #[test]
    fn test_completion_item_with_display() {
        let item = CompletionItem::with_display("test", "Test Item");
        assert_eq!(item.text, "test");
        assert_eq!(item.display, Some("Test Item".to_string()));
    }

    #[test]
    fn test_completion_state_next() {
        let mut state = CompletionState::new("te".to_string(), 0);
        state.set_items(vec![
            CompletionItem::new("test"),
            CompletionItem::new("testing"),
            CompletionItem::new("tester"),
        ]);

        assert_eq!(state.index(), None);

        let item = state.move_next().unwrap();
        assert_eq!(item.text, "test");
        assert_eq!(state.index(), Some(0));

        let item = state.move_next().unwrap();
        assert_eq!(item.text, "testing");
        assert_eq!(state.index(), Some(1));

        let item = state.move_next().unwrap();
        assert_eq!(item.text, "tester");
        assert_eq!(state.index(), Some(2));

        // Wrap around
        let item = state.move_next().unwrap();
        assert_eq!(item.text, "test");
        assert_eq!(state.index(), Some(0));
    }

    #[test]
    fn test_completion_state_prev() {
        let mut state = CompletionState::new("te".to_string(), 0);
        state.set_items(vec![
            CompletionItem::new("test"),
            CompletionItem::new("testing"),
            CompletionItem::new("tester"),
        ]);

        // Start from end when no selection
        let item = state.move_prev().unwrap();
        assert_eq!(item.text, "tester");
        assert_eq!(state.index(), Some(2));

        let item = state.move_prev().unwrap();
        assert_eq!(item.text, "testing");
        assert_eq!(state.index(), Some(1));
    }

    #[test]
    fn test_completion_state_empty() {
        let mut state = CompletionState::new("xyz".to_string(), 0);
        state.set_items(vec![]);

        assert!(state.is_empty());
        assert!(state.move_next().is_none());
        assert!(state.move_prev().is_none());
    }

    #[test]
    fn test_command_completion_provider() {
        let provider = CommandCompletionProvider::new();
        let items = provider.complete("w");

        assert!(items.iter().any(|i| i.text == "write"));
        assert!(items.iter().any(|i| i.text == "wq"));
        assert!(!items.iter().any(|i| i.text == "quit"));
    }

    #[test]
    fn test_command_completion_provider_empty() {
        let provider = CommandCompletionProvider::new();
        let items = provider.complete("");

        // Should return all commands
        assert!(items.len() > 10);
    }

    #[test]
    fn test_command_completion_provider_no_match() {
        let provider = CommandCompletionProvider::new();
        let items = provider.complete("xyz");

        assert!(items.is_empty());
    }

    #[test]
    fn test_completion_type_enum() {
        assert_eq!(CompletionType::File, CompletionType::File);
        assert_ne!(CompletionType::File, CompletionType::Dir);
    }

    #[test]
    fn test_wildmode_default() {
        let mode = WildMode::default();
        assert_eq!(mode, WildMode::Longest);
    }

    #[test]
    fn test_completion_state_accept() {
        let mut state = CompletionState::new("te".to_string(), 0);
        state.set_items(vec![CompletionItem::new("test")]);
        state.move_next();

        let accepted = state.accept();
        assert_eq!(accepted, Some("test".to_string()));
    }

    #[test]
    fn test_completion_state_cancel() {
        let state = CompletionState::new("original".to_string(), 0);
        assert_eq!(state.cancel(), "original");
    }
}
