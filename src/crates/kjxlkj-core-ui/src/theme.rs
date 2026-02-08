//! Theme and color definitions for rendering.

use serde::{Deserialize, Serialize};

use kjxlkj_core_types::Color;

/// A themed color pair (foreground + background).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeColor {
    pub fg: Color,
    pub bg: Color,
}

impl Default for ThemeColor {
    fn default() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
        }
    }
}

/// Active color theme.
///
/// Per /docs/spec/architecture/render-pipeline.md, the theme is included
/// in the EditorSnapshot so the render task can use it directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Normal text.
    pub normal: ThemeColor,
    /// Line numbers.
    pub line_nr: ThemeColor,
    /// Current line number.
    pub cursor_line_nr: ThemeColor,
    /// Mode indicator.
    pub mode_indicator: ThemeColor,
    /// Statusline active window.
    pub statusline: ThemeColor,
    /// Statusline inactive window.
    pub statusline_nc: ThemeColor,
    /// Visual selection.
    pub visual: ThemeColor,
    /// Search highlight.
    pub search: ThemeColor,
    /// Current search match.
    pub inc_search: ThemeColor,
    /// Command line.
    pub cmdline: ThemeColor,
    /// Error messages.
    pub error_msg: ThemeColor,
    /// Warning messages.
    pub warning_msg: ThemeColor,
    /// Tab line active.
    pub tab_line_sel: ThemeColor,
    /// Tab line inactive.
    pub tab_line: ThemeColor,
    /// Tab line fill.
    pub tab_line_fill: ThemeColor,
    /// Window separator.
    pub win_separator: ThemeColor,
    /// Cursor color.
    pub cursor: ThemeColor,
    /// Pmenu (popup menu).
    pub pmenu: ThemeColor,
    /// Pmenu selected.
    pub pmenu_sel: ThemeColor,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            normal: ThemeColor::default(),
            line_nr: ThemeColor {
                fg: Color::Indexed(8),
                bg: Color::Default,
            },
            cursor_line_nr: ThemeColor {
                fg: Color::Indexed(3),
                bg: Color::Default,
            },
            mode_indicator: ThemeColor {
                fg: Color::Indexed(0),
                bg: Color::Indexed(2),
            },
            statusline: ThemeColor {
                fg: Color::Indexed(15),
                bg: Color::Indexed(8),
            },
            statusline_nc: ThemeColor {
                fg: Color::Indexed(7),
                bg: Color::Indexed(0),
            },
            visual: ThemeColor {
                fg: Color::Default,
                bg: Color::Indexed(4),
            },
            search: ThemeColor {
                fg: Color::Indexed(0),
                bg: Color::Indexed(3),
            },
            inc_search: ThemeColor {
                fg: Color::Indexed(0),
                bg: Color::Indexed(6),
            },
            cmdline: ThemeColor::default(),
            error_msg: ThemeColor {
                fg: Color::Indexed(1),
                bg: Color::Default,
            },
            warning_msg: ThemeColor {
                fg: Color::Indexed(3),
                bg: Color::Default,
            },
            tab_line_sel: ThemeColor {
                fg: Color::Indexed(15),
                bg: Color::Indexed(4),
            },
            tab_line: ThemeColor {
                fg: Color::Indexed(7),
                bg: Color::Indexed(8),
            },
            tab_line_fill: ThemeColor {
                fg: Color::Default,
                bg: Color::Indexed(0),
            },
            win_separator: ThemeColor {
                fg: Color::Indexed(8),
                bg: Color::Default,
            },
            cursor: ThemeColor {
                fg: Color::Indexed(0),
                bg: Color::Indexed(15),
            },
            pmenu: ThemeColor {
                fg: Color::Indexed(15),
                bg: Color::Indexed(8),
            },
            pmenu_sel: ThemeColor {
                fg: Color::Indexed(0),
                bg: Color::Indexed(6),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme() {
        let t = Theme::default();
        assert_eq!(t.normal.fg, Color::Default);
    }
}
