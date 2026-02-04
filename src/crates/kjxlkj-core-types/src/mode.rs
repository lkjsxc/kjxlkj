//! Editor mode types.

use serde::{Deserialize, Serialize};

/// The current editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode (navigation and commands).
    #[default]
    Normal,
    /// Insert mode (text entry).
    Insert,
    /// Visual mode (character selection).
    Visual,
    /// Visual line mode (line selection).
    VisualLine,
    /// Visual block mode (rectangular selection).
    VisualBlock,
    /// Command mode (Ex commands).
    Command,
    /// Replace mode (overwrite text).
    Replace,
}

impl Mode {
    /// Check if this is any visual mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Check if this is an insert-like mode.
    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Get the display name of the mode.
    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Command => "COMMAND",
            Mode::Replace => "REPLACE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_default() {
        assert_eq!(Mode::default(), Mode::Normal);
    }

    #[test]
    fn mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn mode_is_insert() {
        assert!(Mode::Insert.is_insert());
        assert!(Mode::Replace.is_insert());
        assert!(!Mode::Normal.is_insert());
    }

    #[test]
    fn mode_display_name() {
        assert_eq!(Mode::Normal.display_name(), "NORMAL");
        assert_eq!(Mode::Insert.display_name(), "INSERT");
        assert_eq!(Mode::VisualLine.display_name(), "V-LINE");
        assert_eq!(Mode::VisualBlock.display_name(), "V-BLOCK");
    }

    #[test]
    fn mode_equality() {
        assert_eq!(Mode::Normal, Mode::Normal);
        assert_ne!(Mode::Normal, Mode::Insert);
    }

    #[test]
    fn mode_command_display() {
        assert_eq!(Mode::Command.display_name(), "COMMAND");
    }

    #[test]
    fn mode_replace_display() {
        assert_eq!(Mode::Replace.display_name(), "REPLACE");
    }

    #[test]
    fn mode_visual_display() {
        assert_eq!(Mode::Visual.display_name(), "VISUAL");
    }

    #[test]
    fn mode_command_not_visual() {
        assert!(!Mode::Command.is_visual());
    }

    #[test]
    fn mode_visual_not_insert() {
        assert!(!Mode::Visual.is_insert());
    }

    #[test]
    fn mode_clone() {
        let m = Mode::Insert;
        let cloned = m.clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn mode_copy() {
        let m = Mode::Visual;
        let copied: Mode = m;
        assert_eq!(m, copied);
    }

    #[test]
    fn mode_replace_is_insert_like() {
        assert!(Mode::Replace.is_insert());
        assert!(!Mode::Replace.is_visual());
    }
}
