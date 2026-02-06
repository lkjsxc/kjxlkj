//! Rendering style types.

use serde::{Deserialize, Serialize};

/// A color definition for terminal rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DarkGrey,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

/// Text styling attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub reverse: bool,
}

impl Style {
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Merge another style on top (non-None fields override).
    pub fn merge(self, other: Self) -> Self {
        Self {
            fg: other.fg.or(self.fg),
            bg: other.bg.or(self.bg),
            bold: other.bold || self.bold,
            italic: other.italic || self.italic,
            underline: other.underline || self.underline,
            strikethrough: other.strikethrough || self.strikethrough,
            dim: other.dim || self.dim,
            reverse: other.reverse || self.reverse,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_builder() {
        let s = Style::default()
            .fg(Color::Red)
            .bg(Color::Black)
            .bold();
        assert_eq!(s.fg, Some(Color::Red));
        assert_eq!(s.bg, Some(Color::Black));
        assert!(s.bold);
        assert!(!s.italic);
    }

    #[test]
    fn style_merge() {
        let base = Style::default().fg(Color::White).bold();
        let over = Style::default().fg(Color::Red).italic();
        let merged = base.merge(over);
        assert_eq!(merged.fg, Some(Color::Red));
        assert!(merged.bold);
        assert!(merged.italic);
    }
}
