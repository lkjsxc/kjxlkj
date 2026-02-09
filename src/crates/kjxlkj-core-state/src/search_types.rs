//! Search types and data structures.

/// Search direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    Forward,
    Backward,
}

/// A search match in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchMatch {
    /// Line index (0-based).
    pub line: usize,
    /// Start column (0-based, byte offset).
    pub col_start: usize,
    /// End column (0-based, byte offset, exclusive).
    pub col_end: usize,
}

/// Case sensitivity mode for search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CaseMode {
    /// Always case-sensitive.
    CaseSensitive,
    /// Always case-insensitive.
    CaseInsensitive,
    /// Ignore case unless pattern contains uppercase (smartcase).
    #[default]
    SmartCase,
}

/// Search state.
#[derive(Debug, Clone)]
pub struct SearchState {
    /// Current search pattern.
    pub pattern: Option<String>,
    /// Search direction.
    pub direction: SearchDirection,
    /// Whether search wraps around the buffer.
    pub wrap_scan: bool,
    /// Case sensitivity mode.
    pub case_mode: CaseMode,
    /// Search history (most recent first).
    pub history: Vec<String>,
    /// Maximum history size.
    pub max_history: usize,
    /// All matches in the current buffer (for highlighting).
    pub matches: Vec<SearchMatch>,
    /// Index of the current match in `matches`.
    pub current_match: Option<usize>,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            pattern: None,
            direction: SearchDirection::Forward,
            wrap_scan: true,
            case_mode: CaseMode::SmartCase,
            history: Vec::new(),
            max_history: 50,
            matches: Vec::new(),
            current_match: None,
        }
    }

    /// Set a new search pattern and add it to history.
    pub fn set_pattern(&mut self, pattern: String, direction: SearchDirection) {
        self.direction = direction;
        if self.history.first().map(|s| s.as_str()) != Some(&pattern) {
            self.history.insert(0, pattern.clone());
            if self.history.len() > self.max_history {
                self.history.pop();
            }
        }
        self.pattern = Some(pattern);
        self.matches.clear();
        self.current_match = None;
    }

    /// Get the effective case sensitivity for the current pattern.
    pub fn is_case_sensitive(&self) -> bool {
        match self.case_mode {
            CaseMode::CaseSensitive => true,
            CaseMode::CaseInsensitive => false,
            CaseMode::SmartCase => self
                .pattern
                .as_ref()
                .map(|p| p.chars().any(|c| c.is_uppercase()))
                .unwrap_or(false),
        }
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
