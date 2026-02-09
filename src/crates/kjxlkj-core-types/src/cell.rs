//! Cell model for the terminal grid.

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use bitflags::bitflags;

use crate::Color;

bitflags! {
    /// Text decoration attributes on a cell.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct CellAttrs: u8 {
        /// No attributes.
        const NONE          = 0b0000_0000;
        /// Bold weight.
        const BOLD          = 0b0000_0001;
        /// Dimmed weight.
        const DIM           = 0b0000_0010;
        /// Italic style.
        const ITALIC        = 0b0000_0100;
        /// Underlined.
        const UNDERLINE     = 0b0000_1000;
        /// Strikethrough.
        const STRIKETHROUGH = 0b0001_0000;
        /// Reversed fg/bg.
        const REVERSE       = 0b0010_0000;
    }
}

/// A single cell in the terminal grid.
///
/// Each cell holds a grapheme cluster, its display width, and styling.
/// Width-2 graphemes produce a continuation cell in the next position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    /// The grapheme cluster displayed in this cell.
    /// Empty string for continuation cells of wide characters.
    pub grapheme: CompactString,
    /// Display width: 1 for normal, 2 for wide (CJK), 0 for continuation.
    pub width: u8,
    /// Foreground color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Text decoration attributes.
    pub attrs: CellAttrs,
    /// True if this cell is the continuation of a width-2 grapheme.
    pub is_wide_continuation: bool,
}

impl Cell {
    /// Create a blank (space) cell with default colors.
    pub fn blank() -> Self {
        Self {
            grapheme: CompactString::const_new(" "),
            width: 1,
            fg: Color::Default,
            bg: Color::Default,
            attrs: CellAttrs::NONE,
            is_wide_continuation: false,
        }
    }

    /// Create a cell for a regular printable grapheme.
    pub fn new(grapheme: &str, width: u8) -> Self {
        Self {
            grapheme: CompactString::from(grapheme),
            width,
            fg: Color::Default,
            bg: Color::Default,
            attrs: CellAttrs::NONE,
            is_wide_continuation: false,
        }
    }

    /// Create a continuation cell for the second column of a wide character.
    pub fn wide_continuation() -> Self {
        Self {
            grapheme: CompactString::const_new(""),
            width: 0,
            fg: Color::Default,
            bg: Color::Default,
            attrs: CellAttrs::NONE,
            is_wide_continuation: true,
        }
    }

    /// Apply foreground and background colors.
    pub fn with_colors(mut self, fg: Color, bg: Color) -> Self {
        self.fg = fg;
        self.bg = bg;
        self
    }

    /// Apply attributes.
    pub fn with_attrs(mut self, attrs: CellAttrs) -> Self {
        self.attrs = attrs;
        self
    }

    /// Reset this cell to a blank space.
    pub fn reset(&mut self) {
        self.grapheme = CompactString::const_new(" ");
        self.width = 1;
        self.fg = Color::Default;
        self.bg = Color::Default;
        self.attrs = CellAttrs::NONE;
        self.is_wide_continuation = false;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::blank()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_cell_properties() {
        let c = Cell::blank();
        assert_eq!(c.grapheme.as_str(), " ");
        assert_eq!(c.width, 1);
        assert!(!c.is_wide_continuation);
    }

    #[test]
    fn wide_continuation_properties() {
        let c = Cell::wide_continuation();
        assert_eq!(c.width, 0);
        assert!(c.is_wide_continuation);
    }

    #[test]
    fn cell_reset() {
        let mut c = Cell::new("あ", 2);
        c.fg = Color::RED;
        c.reset();
        assert_eq!(c.grapheme.as_str(), " ");
        assert_eq!(c.fg, Color::Default);
    }
}
