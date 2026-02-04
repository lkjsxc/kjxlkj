//! Editor mode definitions.

use serde::{Deserialize, Serialize};

/// The current editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Mode {
    /// Normal mode - navigation and commands.
    #[default]
    Normal,
    /// Insert mode - text entry.
    Insert,
    /// Visual mode - character-wise selection.
    Visual,
    /// Visual line mode - line-wise selection.
    VisualLine,
    /// Visual block mode - rectangular selection.
    VisualBlock,
    /// Command mode - Ex command entry.
    Command,
    /// Replace mode - overwrite characters.
    Replace,
}

impl Mode {
    /// Returns true if this is a visual selection mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Returns true if this mode allows text input.
    pub fn is_insert_like(&self) -> bool {
        matches!(self, Mode::Insert | Mode::Replace)
    }

    /// Parse a mode from a string (case-insensitive).
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "normal" | "n" => Some(Mode::Normal),
            "insert" | "i" => Some(Mode::Insert),
            "visual" | "v" => Some(Mode::Visual),
            "visual_line" | "visualline" | "vl" => Some(Mode::VisualLine),
            "visual_block" | "visualblock" | "vb" => Some(Mode::VisualBlock),
            "command" | "cmd" | "c" => Some(Mode::Command),
            "replace" | "r" => Some(Mode::Replace),
            _ => None,
        }
    }

    /// Get the mode name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Normal => "normal",
            Mode::Insert => "insert",
            Mode::Visual => "visual",
            Mode::VisualLine => "visual_line",
            Mode::VisualBlock => "visual_block",
            Mode::Command => "command",
            Mode::Replace => "replace",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_default() {
        assert_eq!(Mode::default(), Mode::Normal);
    }

    #[test]
    fn test_mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
        assert!(!Mode::Insert.is_visual());
    }

    #[test]
    fn test_mode_from_str_loose() {
        assert_eq!(Mode::from_str_loose("normal"), Some(Mode::Normal));
        assert_eq!(Mode::from_str_loose("NORMAL"), Some(Mode::Normal));
        assert_eq!(Mode::from_str_loose("insert"), Some(Mode::Insert));
        assert_eq!(Mode::from_str_loose("visual_line"), Some(Mode::VisualLine));
        assert_eq!(Mode::from_str_loose("unknown"), None);
    }
}
