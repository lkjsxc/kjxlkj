//! Editor mode types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The editing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
    VisualLine,
    VisualBlock,
    Command,
    Search,
    Replace,
}

impl Mode {
    /// Check if mode uses end-exclusive cursor semantics.
    pub fn is_end_exclusive(self) -> bool {
        matches!(
            self,
            Mode::Normal | Mode::Visual | Mode::VisualLine | Mode::VisualBlock | Mode::Replace
        )
    }

    /// Check if mode uses end-inclusive cursor semantics.
    pub fn is_end_inclusive(self) -> bool {
        matches!(self, Mode::Insert)
    }

    /// Check if this is a visual mode variant.
    pub fn is_visual(self) -> bool {
        matches!(self, Mode::Visual | Mode::VisualLine | Mode::VisualBlock)
    }

    /// Get the mode indicator for status line.
    pub fn indicator(self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Command => "COMMAND",
            Mode::Search => "SEARCH",
            Mode::Replace => "REPLACE",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.indicator())
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
    fn mode_end_exclusive() {
        assert!(Mode::Normal.is_end_exclusive());
        assert!(Mode::Visual.is_end_exclusive());
        assert!(!Mode::Insert.is_end_exclusive());
    }

    #[test]
    fn mode_end_inclusive() {
        assert!(Mode::Insert.is_end_inclusive());
        assert!(!Mode::Normal.is_end_inclusive());
    }

    #[test]
    fn mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn mode_indicator() {
        assert_eq!(Mode::Normal.indicator(), "NORMAL");
        assert_eq!(Mode::Insert.indicator(), "INSERT");
    }
}
