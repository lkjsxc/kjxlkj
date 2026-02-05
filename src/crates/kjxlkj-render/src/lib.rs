//! Terminal rendering for kjxlkj editor.
//!
//! This crate provides rendering of editor snapshots to the terminal.

mod renderer;
mod diff;
pub mod syntax;

pub use renderer::TerminalRenderer;
pub use diff::RenderDiff;
pub use syntax::{
    Color as SyntaxColor, Colorscheme, HighlightGroup, HighlightSource, HighlightSpan,
    LineHighlights, Style as SyntaxStyle, SyntaxState,
};

use crossterm::style::Color;

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

    #[test]
    fn test_style_bg() {
        let style = Style::new().bg(Color::Blue);
        assert_eq!(style.bg, Some(Color::Blue));
    }

    #[test]
    fn test_style_reverse() {
        let style = Style::new().reverse();
        assert!(style.reverse);
    }

    #[test]
    fn test_style_default() {
        let style = Style::default();
        assert!(style.fg.is_none());
        assert!(style.bg.is_none());
        assert!(!style.bold);
        assert!(!style.italic);
        assert!(!style.underline);
        assert!(!style.reverse);
    }

    #[test]
    fn test_style_chain() {
        let style = Style::new()
            .fg(Color::Green)
            .bg(Color::Black)
            .bold()
            .reverse();
        assert_eq!(style.fg, Some(Color::Green));
        assert_eq!(style.bg, Some(Color::Black));
        assert!(style.bold);
        assert!(style.reverse);
    }

    #[test]
    fn test_style_equality() {
        let style1 = Style::new().fg(Color::Red).bold();
        let style2 = Style::new().fg(Color::Red).bold();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_style_clone() {
        let style = Style::new().fg(Color::Yellow).bg(Color::Blue);
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_style_debug() {
        let style = Style::new().bold();
        let debug = format!("{:?}", style);
        assert!(debug.contains("bold"));
    }

    #[test]
    fn test_style_inequality() {
        let style1 = Style::new().fg(Color::Red);
        let style2 = Style::new().fg(Color::Blue);
        assert_ne!(style1, style2);
    }

    #[test]
    fn test_style_italic() {
        let mut style = Style::new();
        style.italic = true;
        assert!(style.italic);
    }

    #[test]
    fn test_style_underline() {
        let mut style = Style::new();
        style.underline = true;
        assert!(style.underline);
    }

    #[test]
    fn test_style_copy() {
        let style = Style::new().bold();
        let copied = style; // Copy trait
        assert_eq!(style, copied);
    }

    #[test]
    fn test_style_fg_rgb() {
        let style = Style::new().fg(Color::Rgb { r: 255, g: 0, b: 0 });
        assert!(style.fg.is_some());
    }

    #[test]
    fn test_style_bg_rgb() {
        let style = Style::new().bg(Color::Rgb { r: 0, g: 255, b: 0 });
        assert!(style.bg.is_some());
    }

    #[test]
    fn test_style_all_attrs() {
        let mut style = Style::new();
        style.bold = true;
        style.italic = true;
        style.underline = true;
        style.reverse = true;
        assert!(style.bold && style.italic && style.underline && style.reverse);
    }

    #[test]
    fn test_style_fg_none_initially() {
        let style = Style::new();
        assert!(style.fg.is_none());
        assert!(style.bg.is_none());
    }

    #[test]
    fn test_style_chain_multiple() {
        let style = Style::new()
            .fg(Color::White)
            .bg(Color::Black)
            .bold()
            .reverse();
        assert_eq!(style.fg, Some(Color::White));
        assert_eq!(style.bg, Some(Color::Black));
        assert!(style.bold);
        assert!(style.reverse);
        assert!(!style.italic);
        assert!(!style.underline);
    }

    #[test]
    fn test_style_eq_with_all_fields() {
        let mut s1 = Style::new();
        s1.fg = Some(Color::Red);
        s1.bg = Some(Color::Blue);
        s1.bold = true;
        s1.italic = true;
        s1.underline = true;
        s1.reverse = true;

        let mut s2 = Style::new();
        s2.fg = Some(Color::Red);
        s2.bg = Some(Color::Blue);
        s2.bold = true;
        s2.italic = true;
        s2.underline = true;
        s2.reverse = true;

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_style_ansi_colors() {
        let style = Style::new()
            .fg(Color::AnsiValue(42))
            .bg(Color::AnsiValue(100));
        assert!(style.fg.is_some());
        assert!(style.bg.is_some());
    }

    #[test]
    fn test_style_grey_color() {
        let style = Style::new().fg(Color::Grey);
        assert_eq!(style.fg, Some(Color::Grey));
    }

    #[test]
    fn test_style_dark_grey() {
        let style = Style::new().fg(Color::DarkGrey);
        assert_eq!(style.fg, Some(Color::DarkGrey));
    }

    #[test]
    fn test_style_dark_red() {
        let style = Style::new().fg(Color::DarkRed);
        assert_eq!(style.fg, Some(Color::DarkRed));
    }

    #[test]
    fn test_style_dark_green() {
        let style = Style::new().fg(Color::DarkGreen);
        assert_eq!(style.fg, Some(Color::DarkGreen));
    }

    #[test]
    fn test_style_dark_yellow() {
        let style = Style::new().fg(Color::DarkYellow);
        assert_eq!(style.fg, Some(Color::DarkYellow));
    }

    #[test]
    fn test_style_dark_blue() {
        let style = Style::new().fg(Color::DarkBlue);
        assert_eq!(style.fg, Some(Color::DarkBlue));
    }

    #[test]
    fn test_style_dark_magenta() {
        let style = Style::new().fg(Color::DarkMagenta);
        assert_eq!(style.fg, Some(Color::DarkMagenta));
    }

    #[test]
    fn test_style_dark_cyan() {
        let style = Style::new().fg(Color::DarkCyan);
        assert_eq!(style.fg, Some(Color::DarkCyan));
    }

    #[test]
    fn test_style_magenta() {
        let style = Style::new().fg(Color::Magenta);
        assert_eq!(style.fg, Some(Color::Magenta));
    }

    #[test]
    fn test_style_cyan() {
        let style = Style::new().fg(Color::Cyan);
        assert_eq!(style.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_style_white() {
        let style = Style::new().fg(Color::White);
        assert_eq!(style.fg, Some(Color::White));
    }

    #[test]
    fn test_style_black() {
        let style = Style::new().bg(Color::Black);
        assert_eq!(style.bg, Some(Color::Black));
    }

    #[test]
    fn test_style_yellow() {
        let style = Style::new().fg(Color::Yellow);
        assert_eq!(style.fg, Some(Color::Yellow));
    }

    #[test]
    fn test_style_blue() {
        let style = Style::new().fg(Color::Blue);
        assert_eq!(style.fg, Some(Color::Blue));
    }

    #[test]
    fn test_style_green() {
        let style = Style::new().fg(Color::Green);
        assert_eq!(style.fg, Some(Color::Green));
    }

    #[test]
    fn test_style_red() {
        let style = Style::new().fg(Color::Red);
        assert_eq!(style.fg, Some(Color::Red));
    }
}
