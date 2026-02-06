//! Editor modes as deterministic state machines.

use serde::{Deserialize, Serialize};

/// The primary editor modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    VisualBlock,
    Replace,
    Command,
    OperatorPending,
    /// One normal command from insert mode (Ctrl-o).
    InsertNormal,
    /// Terminal mode — input goes to embedded terminal.
    Terminal,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Visual => write!(f, "VISUAL"),
            Self::VisualLine => write!(f, "V-LINE"),
            Self::VisualBlock => write!(f, "V-BLOCK"),
            Self::Replace => write!(f, "REPLACE"),
            Self::Command => write!(f, "COMMAND"),
            Self::OperatorPending => write!(f, "O-PENDING"),
            Self::InsertNormal => write!(f, "(insert)"),
            Self::Terminal => write!(f, "TERMINAL"),
        }
    }
}

impl Mode {
    /// Parse a mode name from a string (case-insensitive).
    pub fn from_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "normal" | "n" => Some(Self::Normal),
            "insert" | "i" => Some(Self::Insert),
            "visual" | "v" => Some(Self::Visual),
            "visual_line" | "vl" => Some(Self::VisualLine),
            "visual_block" | "vb" => Some(Self::VisualBlock),
            "replace" | "r" => Some(Self::Replace),
            "command" | "c" | "cmdline" => Some(Self::Command),
            "operator_pending" | "op" => Some(Self::OperatorPending),
            "terminal" | "t" => Some(Self::Terminal),
            _ => None,
        }
    }

    /// Whether this mode allows text insertion.
    pub fn is_insert_like(&self) -> bool {
        matches!(self, Self::Insert | Self::Replace)
    }

    /// Whether this mode is the terminal passthrough mode.
    pub fn is_terminal(&self) -> bool { matches!(self, Self::Terminal) }

    /// Whether this mode is a visual selection mode.
    pub fn is_visual(&self) -> bool {
        matches!(self, Self::Visual | Self::VisualLine | Self::VisualBlock)
    }
}

/// Cursor shape driven by mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Bar,
    Underline,
}

impl Mode {
    pub fn cursor_shape(&self) -> CursorShape {
        match self {
            Self::Normal | Self::OperatorPending | Self::InsertNormal => CursorShape::Block,
            Self::Insert | Self::Command => CursorShape::Bar,
            Self::Visual | Self::VisualLine | Self::VisualBlock => CursorShape::Block,
            Self::Replace => CursorShape::Underline,
            Self::Terminal => CursorShape::Bar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_default_is_normal() {
        assert_eq!(Mode::default(), Mode::Normal);
    }

    #[test]
    fn mode_from_name() {
        assert_eq!(Mode::from_name("normal"), Some(Mode::Normal));
        assert_eq!(Mode::from_name("INSERT"), Some(Mode::Insert));
        assert_eq!(Mode::from_name("visual_line"), Some(Mode::VisualLine));
        assert_eq!(Mode::from_name("unknown"), None);
    }

    #[test]
    fn mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn mode_cursor_shapes() {
        assert_eq!(Mode::Normal.cursor_shape(), CursorShape::Block);
        assert_eq!(Mode::Insert.cursor_shape(), CursorShape::Bar);
        assert_eq!(Mode::Replace.cursor_shape(), CursorShape::Underline);
    }

    #[test]
    fn mode_display() {
        assert_eq!(format!("{}", Mode::Normal), "NORMAL");
        assert_eq!(format!("{}", Mode::Insert), "INSERT");
    }
}
