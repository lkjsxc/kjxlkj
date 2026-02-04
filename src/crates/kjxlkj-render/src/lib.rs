//! Terminal rendering for kjxlkj editor.
//!
//! This crate provides rendering of editor snapshots to the terminal.

mod renderer;
mod diff;

pub use renderer::TerminalRenderer;
pub use diff::RenderDiff;

use crossterm::style::Color;
use kjxlkj_core_ui::EditorSnapshot;

/// Style for rendered content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    /// Foreground color.
    pub fg: Option<Color>,
    /// Background color.
    pub bg: Option<Color>,
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
    /// Reverse.
    pub reverse: bool,
}

impl Style {
    /// Default style.
    pub fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            reverse: false,
        }
    }

    /// Set foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set bold.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set reverse.
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Render output trait for abstracting terminal output.
pub trait RenderOutput {
    /// Clear the screen.
    fn clear(&mut self) -> std::io::Result<()>;

    /// Move cursor to position.
    fn move_to(&mut self, x: u16, y: u16) -> std::io::Result<()>;

    /// Write styled text.
    fn write_styled(&mut self, text: &str, style: Style) -> std::io::Result<()>;

    /// Show cursor.
    fn show_cursor(&mut self) -> std::io::Result<()>;

    /// Hide cursor.
    fn hide_cursor(&mut self) -> std::io::Result<()>;

    /// Flush output.
    fn flush(&mut self) -> std::io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_new() {
        let style = Style::new();
        assert!(style.fg.is_none());
        assert!(!style.bold);
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new().fg(Color::Red).bold();
        assert_eq!(style.fg, Some(Color::Red));
        assert!(style.bold);
    }
}
