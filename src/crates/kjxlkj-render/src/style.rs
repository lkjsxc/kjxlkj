//! Style definitions.

use crossterm::style::Color;

/// Text style for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub reverse: bool,
}

impl Style {
    /// Default style.
    pub const fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            reverse: false,
        }
    }

    /// Create a style with foreground color.
    pub const fn fg(color: Color) -> Self {
        Self {
            fg: Some(color),
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            reverse: false,
        }
    }

    /// Set foreground color.
    pub const fn with_fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color.
    pub const fn with_bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set bold.
    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set reverse video.
    pub const fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::default()
    }
}

/// Mode-specific colors.
pub mod mode_colors {
    use crossterm::style::Color;

    pub const NORMAL: Color = Color::Blue;
    pub const INSERT: Color = Color::Green;
    pub const VISUAL: Color = Color::Magenta;
    pub const REPLACE: Color = Color::Red;
    pub const COMMAND: Color = Color::Yellow;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_building() {
        let style = Style::fg(Color::Red).with_bg(Color::Black).bold();
        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, Some(Color::Black));
        assert!(style.bold);
    }
}
