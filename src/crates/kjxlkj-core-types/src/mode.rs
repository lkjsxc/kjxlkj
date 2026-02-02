//! Mode definitions for kjxlkj editor.

use serde::{Deserialize, Serialize};

/// Editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode - navigation and commands.
    #[default]
    Normal,
    /// Insert mode - text entry.
    Insert,
    /// Visual mode - selection.
    Visual(VisualMode),
    /// Command mode - ex commands and search.
    Command(CommandKind),
    /// Replace mode - overwrite text.
    Replace,
    /// Operator-pending mode - waiting for motion/object.
    OperatorPending,
}

impl Mode {
    /// Returns true if this is an insert-like mode.
    pub fn is_insert_like(self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Returns true if this is a visual mode.
    pub fn is_visual(self) -> bool {
        matches!(self, Mode::Visual(_))
    }

    /// Returns the display name for the mode.
    pub fn display_name(self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual(VisualMode::Char) => "VISUAL",
            Mode::Visual(VisualMode::Line) => "V-LINE",
            Mode::Visual(VisualMode::Block) => "V-BLOCK",
            Mode::Command(CommandKind::Ex) => "COMMAND",
            Mode::Command(CommandKind::Search) => "SEARCH",
            Mode::Replace => "REPLACE",
            Mode::OperatorPending => "O-PENDING",
        }
    }
}

/// Visual mode variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum VisualMode {
    /// Character-wise selection (v).
    #[default]
    Char,
    /// Line-wise selection (V).
    Line,
    /// Block/column selection (Ctrl-v).
    Block,
}

/// Command mode variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum CommandKind {
    /// Ex command (:).
    #[default]
    Ex,
    /// Search (/ or ?).
    Search,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_normal() {
        let mode = Mode::default();
        assert_eq!(mode, Mode::Normal);
    }

    #[test]
    fn test_is_insert_like() {
        assert!(Mode::Insert.is_insert_like());
        assert!(Mode::Replace.is_insert_like());
        assert!(!Mode::Normal.is_insert_like());
        assert!(!Mode::Visual(VisualMode::Char).is_insert_like());
    }

    #[test]
    fn test_is_visual() {
        assert!(Mode::Visual(VisualMode::Char).is_visual());
        assert!(Mode::Visual(VisualMode::Line).is_visual());
        assert!(Mode::Visual(VisualMode::Block).is_visual());
        assert!(!Mode::Normal.is_visual());
        assert!(!Mode::Insert.is_visual());
    }

    #[test]
    fn test_display_names() {
        assert_eq!(Mode::Normal.display_name(), "NORMAL");
        assert_eq!(Mode::Insert.display_name(), "INSERT");
        assert_eq!(Mode::Visual(VisualMode::Char).display_name(), "VISUAL");
        assert_eq!(Mode::Visual(VisualMode::Line).display_name(), "V-LINE");
        assert_eq!(Mode::Visual(VisualMode::Block).display_name(), "V-BLOCK");
        assert_eq!(Mode::Command(CommandKind::Ex).display_name(), "COMMAND");
        assert_eq!(Mode::Replace.display_name(), "REPLACE");
    }
}
