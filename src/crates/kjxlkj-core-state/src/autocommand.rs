//! Autocommand system for event-driven automation.
//!
//! Implements autocommands that run actions when specific events occur.

use std::collections::HashMap;

/// Autocommand event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutocmdEvent {
    // Buffer events
    /// New buffer created.
    BufNew,
    /// File read into buffer.
    BufRead,
    /// File read post-processing.
    BufReadPost,
    /// Before writing buffer.
    BufWrite,
    /// After writing buffer.
    BufWritePost,
    /// Enter buffer.
    BufEnter,
    /// Leave buffer.
    BufLeave,
    /// Buffer deleted.
    BufDelete,
    /// Buffer hidden.
    BufHidden,
    /// Buffer unloaded.
    BufUnload,
    /// Buffer window enter.
    BufWinEnter,
    /// Buffer window leave.
    BufWinLeave,

    // File events
    /// Filetype detected.
    FileType,
    /// Before reading file.
    FileReadPre,
    /// After reading file.
    FileReadPost,
    /// Before writing file.
    FileWritePre,
    /// After writing file.
    FileWritePost,

    // Window events
    /// Enter window.
    WinEnter,
    /// Leave window.
    WinLeave,
    /// New window created.
    WinNew,
    /// Window closed.
    WinClosed,
    /// Window resized.
    WinResized,

    // Editor events
    /// Editor started.
    VimEnter,
    /// Editor closing.
    VimLeave,
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,

    // Mode events
    /// Enter insert mode.
    InsertEnter,
    /// Leave insert mode.
    InsertLeave,
    /// Enter visual mode.
    VisualEnter,
    /// Leave visual mode.
    VisualLeave,
    /// Enter command line.
    CmdlineEnter,
    /// Leave command line.
    CmdlineLeave,

    // Text events
    /// Text changed (normal mode).
    TextChanged,
    /// Text changed (insert mode).
    TextChangedI,
    /// Text yanked.
    TextYankPost,
    /// Cursor moved (normal mode).
    CursorMoved,
    /// Cursor moved (insert mode).
    CursorMovedI,
    /// Cursor hold (idle).
    CursorHold,
    /// Cursor hold (insert mode).
    CursorHoldI,

    // Completion events
    /// Completion done.
    CompleteDone,

    // LSP events
    /// LSP attached.
    LspAttach,
    /// LSP detached.
    LspDetach,

    // User events (for plugins).
    User,
}

impl AutocmdEvent {
    /// Get event name as string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::BufNew => "BufNew",
            Self::BufRead => "BufRead",
            Self::BufReadPost => "BufReadPost",
            Self::BufWrite => "BufWrite",
            Self::BufWritePost => "BufWritePost",
            Self::BufEnter => "BufEnter",
            Self::BufLeave => "BufLeave",
            Self::BufDelete => "BufDelete",
            Self::BufHidden => "BufHidden",
            Self::BufUnload => "BufUnload",
            Self::BufWinEnter => "BufWinEnter",
            Self::BufWinLeave => "BufWinLeave",
            Self::FileType => "FileType",
            Self::FileReadPre => "FileReadPre",
            Self::FileReadPost => "FileReadPost",
            Self::FileWritePre => "FileWritePre",
            Self::FileWritePost => "FileWritePost",
            Self::WinEnter => "WinEnter",
            Self::WinLeave => "WinLeave",
            Self::WinNew => "WinNew",
            Self::WinClosed => "WinClosed",
            Self::WinResized => "WinResized",
            Self::VimEnter => "VimEnter",
            Self::VimLeave => "VimLeave",
            Self::FocusGained => "FocusGained",
            Self::FocusLost => "FocusLost",
            Self::InsertEnter => "InsertEnter",
            Self::InsertLeave => "InsertLeave",
            Self::VisualEnter => "VisualEnter",
            Self::VisualLeave => "VisualLeave",
            Self::CmdlineEnter => "CmdlineEnter",
            Self::CmdlineLeave => "CmdlineLeave",
            Self::TextChanged => "TextChanged",
            Self::TextChangedI => "TextChangedI",
            Self::TextYankPost => "TextYankPost",
            Self::CursorMoved => "CursorMoved",
            Self::CursorMovedI => "CursorMovedI",
            Self::CursorHold => "CursorHold",
            Self::CursorHoldI => "CursorHoldI",
            Self::CompleteDone => "CompleteDone",
            Self::LspAttach => "LspAttach",
            Self::LspDetach => "LspDetach",
            Self::User => "User",
        }
    }

    /// Parse event from string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "BufNew" => Some(Self::BufNew),
            "BufRead" => Some(Self::BufRead),
            "BufReadPost" => Some(Self::BufReadPost),
            "BufWrite" => Some(Self::BufWrite),
            "BufWritePost" => Some(Self::BufWritePost),
            "BufEnter" => Some(Self::BufEnter),
            "BufLeave" => Some(Self::BufLeave),
            "BufDelete" => Some(Self::BufDelete),
            "BufHidden" => Some(Self::BufHidden),
            "BufUnload" => Some(Self::BufUnload),
            "BufWinEnter" => Some(Self::BufWinEnter),
            "BufWinLeave" => Some(Self::BufWinLeave),
            "FileType" => Some(Self::FileType),
            "FileReadPre" => Some(Self::FileReadPre),
            "FileReadPost" => Some(Self::FileReadPost),
            "FileWritePre" => Some(Self::FileWritePre),
            "FileWritePost" => Some(Self::FileWritePost),
            "WinEnter" => Some(Self::WinEnter),
            "WinLeave" => Some(Self::WinLeave),
            "WinNew" => Some(Self::WinNew),
            "WinClosed" => Some(Self::WinClosed),
            "WinResized" => Some(Self::WinResized),
            "VimEnter" => Some(Self::VimEnter),
            "VimLeave" => Some(Self::VimLeave),
            "FocusGained" => Some(Self::FocusGained),
            "FocusLost" => Some(Self::FocusLost),
            "InsertEnter" => Some(Self::InsertEnter),
            "InsertLeave" => Some(Self::InsertLeave),
            "VisualEnter" => Some(Self::VisualEnter),
            "VisualLeave" => Some(Self::VisualLeave),
            "CmdlineEnter" => Some(Self::CmdlineEnter),
            "CmdlineLeave" => Some(Self::CmdlineLeave),
            "TextChanged" => Some(Self::TextChanged),
            "TextChangedI" => Some(Self::TextChangedI),
            "TextYankPost" => Some(Self::TextYankPost),
            "CursorMoved" => Some(Self::CursorMoved),
            "CursorMovedI" => Some(Self::CursorMovedI),
            "CursorHold" => Some(Self::CursorHold),
            "CursorHoldI" => Some(Self::CursorHoldI),
            "CompleteDone" => Some(Self::CompleteDone),
            "LspAttach" => Some(Self::LspAttach),
            "LspDetach" => Some(Self::LspDetach),
            "User" => Some(Self::User),
            _ => None,
        }
    }
}

/// A pattern for matching autocommand triggers.
#[derive(Debug, Clone)]
pub enum AutocmdPattern {
    /// Match any file.
    All,
    /// Glob pattern (e.g., "*.rs").
    Glob(String),
    /// Filetype pattern (e.g., "rust").
    Filetype(String),
    /// Buffer number.
    Buffer(u64),
}

impl AutocmdPattern {
    /// Check if pattern matches a filetype.
    pub fn matches_filetype(&self, ft: &str) -> bool {
        match self {
            Self::All => true,
            Self::Filetype(pattern) => pattern == ft,
            _ => false,
        }
    }

    /// Check if pattern matches a filename.
    pub fn matches_filename(&self, filename: &str) -> bool {
        match self {
            Self::All => true,
            Self::Glob(pattern) => glob_match(pattern, filename),
            _ => false,
        }
    }
}

/// Simple glob matching (supports * only).
fn glob_match(pattern: &str, text: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(suffix) = pattern.strip_prefix("*.") {
        return text.ends_with(&format!(".{}", suffix));
    }
    if let Some(prefix) = pattern.strip_suffix("*") {
        return text.starts_with(prefix);
    }
    pattern == text
}

/// An autocommand action.
#[derive(Debug, Clone)]
pub enum AutocmdAction {
    /// Run a command string.
    Command(String),
    /// Call a named function.
    Function(String),
    /// Multiple actions.
    Group(Vec<AutocmdAction>),
}

/// A single autocommand.
#[derive(Debug, Clone)]
pub struct Autocommand {
    /// Unique ID.
    pub id: u64,
    /// Event that triggers this autocommand.
    pub event: AutocmdEvent,
    /// Pattern to match.
    pub pattern: AutocmdPattern,
    /// Action to execute.
    pub action: AutocmdAction,
    /// Optional group name.
    pub group: Option<String>,
    /// Whether this autocommand is enabled.
    pub enabled: bool,
    /// Run only once then delete.
    pub once: bool,
    /// Allow nested autocommand triggers.
    pub nested: bool,
    /// Optional description.
    pub desc: Option<String>,
}

impl Autocommand {
    /// Create a new autocommand.
    pub fn new(event: AutocmdEvent, pattern: AutocmdPattern, action: AutocmdAction) -> Self {
        Self {
            id: 0,
            event,
            pattern,
            action,
            group: None,
            enabled: true,
            once: false,
            nested: false,
            desc: None,
        }
    }

    /// Set group.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Set once flag.
    pub fn with_once(mut self, once: bool) -> Self {
        self.once = once;
        self
    }

    /// Set nested flag.
    pub fn with_nested(mut self, nested: bool) -> Self {
        self.nested = nested;
        self
    }

    /// Set description.
    pub fn with_desc(mut self, desc: impl Into<String>) -> Self {
        self.desc = Some(desc.into());
        self
    }
}

/// Autocommand group.
#[derive(Debug, Clone)]
pub struct AutocmdGroup {
    /// Group name.
    pub name: String,
    /// Autocommands in this group.
    pub autocmds: Vec<u64>,
}

impl AutocmdGroup {
    /// Create a new group.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            autocmds: Vec::new(),
        }
    }
}

/// Autocommand manager.
#[derive(Debug, Default)]
pub struct AutocmdManager {
    /// All autocommands by ID.
    autocmds: HashMap<u64, Autocommand>,
    /// Autocommands by event.
    by_event: HashMap<AutocmdEvent, Vec<u64>>,
    /// Groups.
    groups: HashMap<String, AutocmdGroup>,
    /// Next ID.
    next_id: u64,
    /// Whether currently executing (to prevent infinite recursion).
    executing: bool,
}

impl AutocmdManager {
    /// Create a new autocommand manager.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Check if currently executing autocommands.
    pub fn is_executing(&self) -> bool {
        self.executing
    }

    /// Set executing state.
    pub fn set_executing(&mut self, executing: bool) {
        self.executing = executing;
    }

    /// Create a group.
    pub fn create_group(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.groups.contains_key(&name) {
            self.groups.insert(name.clone(), AutocmdGroup::new(&name));
        }
    }

    /// Clear a group (remove all autocmds in it).
    pub fn clear_group(&mut self, name: &str) {
        if let Some(group) = self.groups.get_mut(name) {
            for id in group.autocmds.drain(..) {
                if let Some(ac) = self.autocmds.remove(&id) {
                    if let Some(ids) = self.by_event.get_mut(&ac.event) {
                        ids.retain(|&i| i != id);
                    }
                }
            }
        }
    }

    /// Delete a group.
    pub fn delete_group(&mut self, name: &str) {
        self.clear_group(name);
        self.groups.remove(name);
    }

    /// Add an autocommand.
    pub fn add(&mut self, mut autocmd: Autocommand) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        autocmd.id = id;

        let event = autocmd.event;
        if let Some(ref group_name) = autocmd.group {
            if let Some(group) = self.groups.get_mut(group_name) {
                group.autocmds.push(id);
            }
        }

        self.autocmds.insert(id, autocmd);
        self.by_event.entry(event).or_default().push(id);
        id
    }

    /// Remove an autocommand by ID.
    pub fn remove(&mut self, id: u64) -> Option<Autocommand> {
        if let Some(ac) = self.autocmds.remove(&id) {
            if let Some(ids) = self.by_event.get_mut(&ac.event) {
                ids.retain(|&i| i != id);
            }
            if let Some(ref group_name) = ac.group {
                if let Some(group) = self.groups.get_mut(group_name) {
                    group.autocmds.retain(|&i| i != id);
                }
            }
            Some(ac)
        } else {
            None
        }
    }

    /// Get autocommands for an event.
    pub fn get_for_event(&self, event: AutocmdEvent) -> Vec<&Autocommand> {
        self.by_event
            .get(&event)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.autocmds.get(id))
                    .filter(|ac| ac.enabled)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get matching autocommands for an event and filetype.
    pub fn get_matching(
        &self,
        event: AutocmdEvent,
        filetype: Option<&str>,
        filename: Option<&str>,
    ) -> Vec<&Autocommand> {
        self.get_for_event(event)
            .into_iter()
            .filter(|ac| {
                match &ac.pattern {
                    AutocmdPattern::All => true,
                    AutocmdPattern::Filetype(pat) => {
                        filetype.map(|ft| ft == pat).unwrap_or(false)
                    }
                    AutocmdPattern::Glob(pat) => {
                        filename.map(|f| glob_match(pat, f)).unwrap_or(false)
                    }
                    AutocmdPattern::Buffer(buf_id) => {
                        // Buffer matching would need buffer ID passed in
                        // For now, just return false unless we add buffer param
                        let _ = buf_id;
                        false
                    }
                }
            })
            .collect()
    }

    /// List all autocommands.
    pub fn list(&self) -> Vec<&Autocommand> {
        self.autocmds.values().collect()
    }

    /// List autocommands in a group.
    pub fn list_group(&self, name: &str) -> Vec<&Autocommand> {
        self.groups
            .get(name)
            .map(|g| {
                g.autocmds
                    .iter()
                    .filter_map(|id| self.autocmds.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_name_roundtrip() {
        let event = AutocmdEvent::BufEnter;
        let name = event.name();
        assert_eq!(AutocmdEvent::from_name(name), Some(event));
    }

    #[test]
    fn test_pattern_all() {
        let pattern = AutocmdPattern::All;
        assert!(pattern.matches_filetype("rust"));
        assert!(pattern.matches_filename("test.rs"));
    }

    #[test]
    fn test_pattern_glob() {
        let pattern = AutocmdPattern::Glob("*.rs".to_string());
        assert!(pattern.matches_filename("test.rs"));
        assert!(!pattern.matches_filename("test.py"));
    }

    #[test]
    fn test_pattern_filetype() {
        let pattern = AutocmdPattern::Filetype("rust".to_string());
        assert!(pattern.matches_filetype("rust"));
        assert!(!pattern.matches_filetype("python"));
    }

    #[test]
    fn test_glob_match() {
        assert!(glob_match("*", "anything"));
        assert!(glob_match("*.rs", "main.rs"));
        assert!(!glob_match("*.rs", "main.py"));
        assert!(glob_match("test*", "testing"));
        assert!(glob_match("exact", "exact"));
    }

    #[test]
    fn test_autocommand_new() {
        let ac = Autocommand::new(
            AutocmdEvent::BufWrite,
            AutocmdPattern::Glob("*.rs".to_string()),
            AutocmdAction::Command("rustfmt %".to_string()),
        )
        .with_desc("Format Rust files");

        assert_eq!(ac.event, AutocmdEvent::BufWrite);
        assert_eq!(ac.desc, Some("Format Rust files".to_string()));
        assert!(ac.enabled);
    }

    #[test]
    fn test_manager_add_remove() {
        let mut mgr = AutocmdManager::new();
        let ac = Autocommand::new(
            AutocmdEvent::BufEnter,
            AutocmdPattern::All,
            AutocmdAction::Command("echo entered".to_string()),
        );

        let id = mgr.add(ac);
        assert!(mgr.autocmds.contains_key(&id));

        let removed = mgr.remove(id);
        assert!(removed.is_some());
        assert!(!mgr.autocmds.contains_key(&id));
    }

    #[test]
    fn test_manager_get_for_event() {
        let mut mgr = AutocmdManager::new();

        mgr.add(Autocommand::new(
            AutocmdEvent::BufEnter,
            AutocmdPattern::All,
            AutocmdAction::Command("cmd1".to_string()),
        ));
        mgr.add(Autocommand::new(
            AutocmdEvent::BufEnter,
            AutocmdPattern::All,
            AutocmdAction::Command("cmd2".to_string()),
        ));
        mgr.add(Autocommand::new(
            AutocmdEvent::BufLeave,
            AutocmdPattern::All,
            AutocmdAction::Command("cmd3".to_string()),
        ));

        let enter_cmds = mgr.get_for_event(AutocmdEvent::BufEnter);
        assert_eq!(enter_cmds.len(), 2);

        let leave_cmds = mgr.get_for_event(AutocmdEvent::BufLeave);
        assert_eq!(leave_cmds.len(), 1);
    }

    #[test]
    fn test_manager_groups() {
        let mut mgr = AutocmdManager::new();
        mgr.create_group("format");

        let id = mgr.add(
            Autocommand::new(
                AutocmdEvent::BufWrite,
                AutocmdPattern::All,
                AutocmdAction::Command("format".to_string()),
            )
            .with_group("format"),
        );

        let group_cmds = mgr.list_group("format");
        assert_eq!(group_cmds.len(), 1);
        assert_eq!(group_cmds[0].id, id);

        mgr.clear_group("format");
        assert!(mgr.list_group("format").is_empty());
        assert!(!mgr.autocmds.contains_key(&id));
    }

    #[test]
    fn test_get_matching() {
        let mut mgr = AutocmdManager::new();

        mgr.add(Autocommand::new(
            AutocmdEvent::FileType,
            AutocmdPattern::Filetype("rust".to_string()),
            AutocmdAction::Command("rust setup".to_string()),
        ));
        mgr.add(Autocommand::new(
            AutocmdEvent::FileType,
            AutocmdPattern::Filetype("python".to_string()),
            AutocmdAction::Command("python setup".to_string()),
        ));

        let rust_cmds = mgr.get_matching(AutocmdEvent::FileType, Some("rust"), None);
        assert_eq!(rust_cmds.len(), 1);

        let py_cmds = mgr.get_matching(AutocmdEvent::FileType, Some("python"), None);
        assert_eq!(py_cmds.len(), 1);
    }
}
