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

    #[test]
    fn test_mode_is_insert_like() {
        assert!(Mode::Insert.is_insert_like());
        assert!(Mode::Replace.is_insert_like());
        assert!(!Mode::Normal.is_insert_like());
        assert!(!Mode::Visual.is_insert_like());
    }

    #[test]
    fn test_mode_as_str() {
        assert_eq!(Mode::Normal.as_str(), "normal");
        assert_eq!(Mode::Insert.as_str(), "insert");
        assert_eq!(Mode::Visual.as_str(), "visual");
        assert_eq!(Mode::VisualLine.as_str(), "visual_line");
        assert_eq!(Mode::VisualBlock.as_str(), "visual_block");
        assert_eq!(Mode::Command.as_str(), "command");
        assert_eq!(Mode::Replace.as_str(), "replace");
    }

    #[test]
    fn test_mode_from_str_loose_shortcuts() {
        assert_eq!(Mode::from_str_loose("n"), Some(Mode::Normal));
        assert_eq!(Mode::from_str_loose("i"), Some(Mode::Insert));
        assert_eq!(Mode::from_str_loose("v"), Some(Mode::Visual));
        assert_eq!(Mode::from_str_loose("vl"), Some(Mode::VisualLine));
        assert_eq!(Mode::from_str_loose("vb"), Some(Mode::VisualBlock));
        assert_eq!(Mode::from_str_loose("cmd"), Some(Mode::Command));
        assert_eq!(Mode::from_str_loose("r"), Some(Mode::Replace));
    }

    #[test]
    fn test_mode_clone() {
        let mode = Mode::Visual;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_mode_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Mode::Normal);
        assert!(set.contains(&Mode::Normal));
        assert!(!set.contains(&Mode::Insert));
    }

    #[test]
    fn test_mode_debug() {
        let mode = Mode::Command;
        let debug = format!("{:?}", mode);
        assert!(debug.contains("Command"));
    }

    #[test]
    fn test_mode_all_variants() {
        let modes = [
            Mode::Normal,
            Mode::Insert,
            Mode::Visual,
            Mode::VisualLine,
            Mode::VisualBlock,
            Mode::Command,
            Mode::Replace,
        ];
        assert_eq!(modes.len(), 7);
    }

    #[test]
    fn test_mode_visual_block_is_visual() {
        assert!(Mode::VisualBlock.is_visual());
    }

    #[test]
    fn test_mode_command_not_visual() {
        assert!(!Mode::Command.is_visual());
    }

    #[test]
    fn test_mode_replace_is_insert_like() {
        assert!(Mode::Replace.is_insert_like());
    }

    #[test]
    fn test_mode_not_insert_like() {
        assert!(!Mode::Command.is_insert_like());
    }

    #[test]
    fn test_mode_copy() {
        let mode = Mode::Normal;
        let copied = mode;
        assert_eq!(mode, copied);
    }

    #[test]
    fn test_mode_inequality() {
        assert_ne!(Mode::Normal, Mode::Insert);
        assert_ne!(Mode::Visual, Mode::VisualLine);
        assert_ne!(Mode::Command, Mode::Replace);
    }

    #[test]
    fn test_mode_as_str_roundtrip() {
        for mode in [Mode::Normal, Mode::Insert, Mode::Visual, Mode::VisualLine, Mode::VisualBlock, Mode::Command, Mode::Replace] {
            let s = mode.as_str();
            let parsed = Mode::from_str_loose(s);
            assert_eq!(parsed, Some(mode));
        }
    }

    #[test]
    fn test_mode_from_str_loose_case_insensitive() {
        assert_eq!(Mode::from_str_loose("NORMAL"), Some(Mode::Normal));
        assert_eq!(Mode::from_str_loose("InSeRt"), Some(Mode::Insert));
        assert_eq!(Mode::from_str_loose("VISUAL"), Some(Mode::Visual));
    }

    #[test]
    fn test_mode_from_str_loose_unknown() {
        assert_eq!(Mode::from_str_loose("unknown"), None);
        assert_eq!(Mode::from_str_loose(""), None);
        assert_eq!(Mode::from_str_loose("xyz"), None);
    }

    #[test]
    fn test_mode_eq_reflexive() {
        assert_eq!(Mode::Normal, Mode::Normal);
        assert_eq!(Mode::Insert, Mode::Insert);
        assert_eq!(Mode::Visual, Mode::Visual);
    }

    #[test]
    fn test_mode_visual_line_not_insert_like() {
        assert!(!Mode::VisualLine.is_insert_like());
    }

    #[test]
    fn test_mode_visual_block_not_insert_like() {
        assert!(!Mode::VisualBlock.is_insert_like());
    }

    #[test]
    fn test_mode_hash_all_modes() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Mode::Normal);
        set.insert(Mode::Insert);
        set.insert(Mode::Visual);
        set.insert(Mode::VisualLine);
        set.insert(Mode::VisualBlock);
        set.insert(Mode::Command);
        set.insert(Mode::Replace);
        assert_eq!(set.len(), 7);
    }

    #[test]
    fn test_mode_from_str_visualline_alt() {
        assert_eq!(Mode::from_str_loose("visualline"), Some(Mode::VisualLine));
    }

    #[test]
    fn test_mode_from_str_visualblock_alt() {
        assert_eq!(Mode::from_str_loose("visualblock"), Some(Mode::VisualBlock));
    }

    #[test]
    fn test_mode_from_str_c() {
        assert_eq!(Mode::from_str_loose("c"), Some(Mode::Command));
    }
}
