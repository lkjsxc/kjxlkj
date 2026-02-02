//! Autocommand events.
//!
//! Event-driven automation for editor actions.

use std::collections::HashMap;

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
        // Simple glob: *.ext
        if let Some(ext) = self.pattern.strip_prefix("*.") {
            return filename.ends_with(&format!(".{}", ext));
        }
        filename == self.pattern
    }
}

/// Autocommand manager.
#[derive(Debug, Default)]
pub struct AutoCmdManager {
    /// Commands by event.
    commands: HashMap<AutoEvent, Vec<AutoCmd>>,
    /// Defined groups.
    groups: Vec<String>,
}

impl AutoCmdManager {
    /// Creates a new manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines a group.
    pub fn define_group(&mut self, name: &str) {
        if !self.groups.contains(&name.to_string()) {
            self.groups.push(name.to_string());
        }
    }

    /// Adds an autocommand.
    pub fn add(&mut self, cmd: AutoCmd) {
        self.commands.entry(cmd.event).or_default().push(cmd);
    }

    /// Removes all commands for a group.
    pub fn clear_group(&mut self, group: &str) {
        for cmds in self.commands.values_mut() {
            cmds.retain(|c| c.group.as_deref() != Some(group));
        }
    }

    /// Gets commands for an event.
    pub fn get(&self, event: AutoEvent, filename: &str) -> Vec<&AutoCmd> {
        self.commands
            .get(&event)
            .map(|cmds| cmds.iter().filter(|c| c.matches(filename)).collect())
            .unwrap_or_default()
    }

    /// Returns command count.
    pub fn len(&self) -> usize {
        self.commands.values().map(|v| v.len()).sum()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_event_name() {
        assert_eq!(AutoEvent::BufEnter.name(), "BufEnter");
    }

    #[test]
    fn test_autocmd_new() {
        let cmd = AutoCmd::new(AutoEvent::BufReadPost, "*.rs", "setlocal ft=rust");
        assert_eq!(cmd.pattern, "*.rs");
    }

    #[test]
    fn test_autocmd_matches() {
        let cmd = AutoCmd::new(AutoEvent::BufReadPost, "*.rs", "echo");
        assert!(cmd.matches("main.rs"));
        assert!(!cmd.matches("main.py"));
    }

    #[test]
    fn test_autocmd_matches_star() {
        let cmd = AutoCmd::new(AutoEvent::BufEnter, "*", "echo");
        assert!(cmd.matches("anything.txt"));
    }

    #[test]
    fn test_autocmd_manager_add() {
        let mut mgr = AutoCmdManager::new();
        mgr.add(AutoCmd::new(AutoEvent::BufEnter, "*", "echo"));
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_autocmd_manager_get() {
        let mut mgr = AutoCmdManager::new();
        mgr.add(AutoCmd::new(AutoEvent::BufReadPost, "*.rs", "echo"));
        let cmds = mgr.get(AutoEvent::BufReadPost, "main.rs");
        assert_eq!(cmds.len(), 1);
    }
}
