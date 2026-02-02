//! Sign types for sign column support.
//!
//! Types for representing signs and sign definitions.

/// Sign priority.
pub type SignPriority = u32;

/// A sign definition.
#[derive(Debug, Clone)]
pub struct SignDefinition {
    /// Sign name.
    pub name: String,
    /// Text to display (1-2 chars).
    pub text: String,
    /// Highlight group for text.
    pub text_highlight: String,
    /// Highlight group for line.
    pub line_highlight: Option<String>,
    /// Highlight group for number column.
    pub num_highlight: Option<String>,
}

impl SignDefinition {
    /// Creates a new sign definition.
    pub fn new(name: &str, text: &str) -> Self {
        Self {
            name: name.to_string(),
            text: text.to_string(),
            text_highlight: "SignColumn".to_string(),
            line_highlight: None,
            num_highlight: None,
        }
    }

    /// Sets the text highlight.
    pub fn with_text_highlight(mut self, group: &str) -> Self {
        self.text_highlight = group.to_string();
        self
    }

    /// Sets the line highlight.
    pub fn with_line_highlight(mut self, group: &str) -> Self {
        self.line_highlight = Some(group.to_string());
        self
    }
}

/// A placed sign.
#[derive(Debug, Clone)]
pub struct Sign {
    /// Sign ID.
    pub id: usize,
    /// Sign definition name.
    pub name: String,
    /// Line number (1-based).
    pub line: usize,
    /// Priority.
    pub priority: SignPriority,
}

impl Sign {
    /// Creates a new sign.
    pub fn new(id: usize, name: &str, line: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            line,
            priority: 10,
        }
    }

    /// Sets the priority.
    pub fn with_priority(mut self, priority: SignPriority) -> Self {
        self.priority = priority;
        self
    }
}
