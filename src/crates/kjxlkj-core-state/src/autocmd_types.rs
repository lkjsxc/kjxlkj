//! Autocommand types.
//!
//! Types for autocommand events and definitions.

/// Autocommand event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoEvent {
    /// Buffer about to be read.
    BufReadPre,
    /// Buffer read completed.
    BufReadPost,
    /// Buffer about to be written.
    BufWritePre,
    /// Buffer written.
    BufWritePost,
    /// New buffer created.
    BufNew,
    /// Buffer added to list.
    BufAdd,
    /// Buffer deleted.
    BufDelete,
    /// Entering a buffer.
    BufEnter,
    /// Leaving a buffer.
    BufLeave,
    /// Window entered.
    WinEnter,
    /// Window left.
    WinLeave,
    /// Tab entered.
    TabEnter,
    /// Tab left.
    TabLeave,
    /// Vim started.
    VimEnter,
    /// About to exit.
    VimLeavePre,
    /// Exiting.
    VimLeave,
    /// Insert mode entered.
    InsertEnter,
    /// Insert mode left.
    InsertLeave,
    /// Text changed.
    TextChanged,
    /// Text changed in insert mode.
    TextChangedI,
    /// Cursor moved.
    CursorMoved,
    /// Cursor moved in insert mode.
    CursorMovedI,
    /// File type detected.
    FileType,
}

impl AutoEvent {
    /// Returns the event name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::BufReadPre => "BufReadPre",
            Self::BufReadPost => "BufReadPost",
            Self::BufWritePre => "BufWritePre",
            Self::BufWritePost => "BufWritePost",
            Self::BufNew => "BufNew",
            Self::BufAdd => "BufAdd",
            Self::BufDelete => "BufDelete",
            Self::BufEnter => "BufEnter",
            Self::BufLeave => "BufLeave",
            Self::WinEnter => "WinEnter",
            Self::WinLeave => "WinLeave",
            Self::TabEnter => "TabEnter",
            Self::TabLeave => "TabLeave",
            Self::VimEnter => "VimEnter",
            Self::VimLeavePre => "VimLeavePre",
            Self::VimLeave => "VimLeave",
            Self::InsertEnter => "InsertEnter",
            Self::InsertLeave => "InsertLeave",
            Self::TextChanged => "TextChanged",
            Self::TextChangedI => "TextChangedI",
            Self::CursorMoved => "CursorMoved",
            Self::CursorMovedI => "CursorMovedI",
            Self::FileType => "FileType",
        }
    }
}

/// An autocommand.
#[derive(Debug, Clone)]
pub struct AutoCmd {
    /// Event trigger.
    pub event: AutoEvent,
    /// Pattern to match.
    pub pattern: String,
    /// Command to execute.
    pub command: String,
    /// Optional group.
    pub group: Option<String>,
    /// Whether to run only once.
    pub once: bool,
}

impl AutoCmd {
    /// Creates a new autocommand.
    pub fn new(event: AutoEvent, pattern: &str, command: &str) -> Self {
        Self {
            event,
            pattern: pattern.to_string(),
            command: command.to_string(),
            group: None,
            once: false,
        }
    }

    /// Sets the group.
    pub fn with_group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }

    /// Matches a file pattern.
    pub fn matches(&self, filename: &str) -> bool {
        if self.pattern == "*" {
            return true;
        }
        if let Some(ext) = self.pattern.strip_prefix("*.") {
            return filename.ends_with(&format!(".{}", ext));
        }
        filename == self.pattern
    }
}
