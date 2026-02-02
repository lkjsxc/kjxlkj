//! Theme integration for rendering.
//!
//! Provides conversion from kjxlkj-host theme definitions to render styles.

use crate::{Color as RenderColor, Style as RenderStyle};

/// A resolved render theme ready for use.
#[derive(Debug, Clone)]
pub struct RenderTheme {
    /// Normal text.
    pub normal: RenderStyle,
    /// Keywords.
    pub keyword: RenderStyle,
    /// Functions.
    pub function: RenderStyle,
    /// Strings.
    pub string: RenderStyle,
    /// Numbers.
    pub number: RenderStyle,
    /// Types.
    pub r#type: RenderStyle,
    /// Comments.
    pub comment: RenderStyle,
    /// Line numbers in gutter.
    pub line_number: RenderStyle,
    /// Current line number.
    pub line_number_current: RenderStyle,
    /// Status line.
    pub statusline: RenderStyle,
    /// Mode indicator.
    pub mode_normal: RenderStyle,
    pub mode_insert: RenderStyle,
    pub mode_visual: RenderStyle,
    pub mode_command: RenderStyle,
    /// Selection highlight.
    pub selection: RenderStyle,
    /// Search match highlight.
    pub search: RenderStyle,
    /// Cursor.
    pub cursor: RenderStyle,
}

impl Default for RenderTheme {
    fn default() -> Self {
        Self::monokai()
    }
}

impl RenderTheme {
    /// Creates a default Monokai-inspired theme.
    pub fn monokai() -> Self {
        Self {
            normal: RenderStyle::new().fg(RenderColor::White),
            keyword: RenderStyle::new().fg(RenderColor::Magenta).bold(),
            function: RenderStyle::new().fg(RenderColor::Green),
            string: RenderStyle::new().fg(RenderColor::Yellow),
            number: RenderStyle::new().fg(RenderColor::Magenta),
            r#type: RenderStyle::new().fg(RenderColor::Cyan),
            comment: RenderStyle::new().fg(RenderColor::BrightBlack).italic(),
            line_number: RenderStyle::new().fg(RenderColor::BrightBlack),
            line_number_current: RenderStyle::new()
                .fg(RenderColor::Yellow)
                .bold(),
            statusline: RenderStyle::new()
                .fg(RenderColor::Black)
                .bg(RenderColor::White),
            mode_normal: RenderStyle::new()
                .fg(RenderColor::Black)
                .bg(RenderColor::Green)
                .bold(),
            mode_insert: RenderStyle::new()
                .fg(RenderColor::Black)
                .bg(RenderColor::Blue)
                .bold(),
            mode_visual: RenderStyle::new()
                .fg(RenderColor::Black)
                .bg(RenderColor::Magenta)
                .bold(),
            mode_command: RenderStyle::new()
                .fg(RenderColor::Black)
                .bg(RenderColor::Yellow)
                .bold(),
            selection: RenderStyle::new().bg(RenderColor::BrightBlack),
            search: RenderStyle::new()
                .fg(RenderColor::Black)
                .bg(RenderColor::Yellow),
            cursor: RenderStyle::new().reverse(true),
        }
    }

    /// Creates a Dracula-inspired theme.
    pub fn dracula() -> Self {
        Self {
            normal: RenderStyle::new().fg(RenderColor::Rgb(248, 248, 242)),
            keyword: RenderStyle::new()
                .fg(RenderColor::Rgb(255, 121, 198))
                .bold(),
            function: RenderStyle::new().fg(RenderColor::Rgb(80, 250, 123)),
            string: RenderStyle::new().fg(RenderColor::Rgb(241, 250, 140)),
            number: RenderStyle::new().fg(RenderColor::Rgb(189, 147, 249)),
            r#type: RenderStyle::new().fg(RenderColor::Rgb(139, 233, 253)),
            comment: RenderStyle::new()
                .fg(RenderColor::Rgb(98, 114, 164))
                .italic(),
            line_number: RenderStyle::new().fg(RenderColor::Rgb(68, 71, 90)),
            line_number_current: RenderStyle::new()
                .fg(RenderColor::Rgb(248, 248, 242))
                .bold(),
            statusline: RenderStyle::new()
                .fg(RenderColor::Rgb(248, 248, 242))
                .bg(RenderColor::Rgb(68, 71, 90)),
            mode_normal: RenderStyle::new()
                .fg(RenderColor::Rgb(40, 42, 54))
                .bg(RenderColor::Rgb(80, 250, 123))
                .bold(),
            mode_insert: RenderStyle::new()
                .fg(RenderColor::Rgb(40, 42, 54))
                .bg(RenderColor::Rgb(139, 233, 253))
                .bold(),
            mode_visual: RenderStyle::new()
                .fg(RenderColor::Rgb(40, 42, 54))
                .bg(RenderColor::Rgb(189, 147, 249))
                .bold(),
            mode_command: RenderStyle::new()
                .fg(RenderColor::Rgb(40, 42, 54))
                .bg(RenderColor::Rgb(241, 250, 140))
                .bold(),
            selection: RenderStyle::new().bg(RenderColor::Rgb(68, 71, 90)),
            search: RenderStyle::new()
                .fg(RenderColor::Rgb(40, 42, 54))
                .bg(RenderColor::Rgb(241, 250, 140)),
            cursor: RenderStyle::new().reverse(true),
        }
    }

    /// Creates a Nord-inspired theme.
    pub fn nord() -> Self {
        Self {
            normal: RenderStyle::new().fg(RenderColor::Rgb(216, 222, 233)),
            keyword: RenderStyle::new()
                .fg(RenderColor::Rgb(129, 161, 193))
                .bold(),
            function: RenderStyle::new().fg(RenderColor::Rgb(136, 192, 208)),
            string: RenderStyle::new().fg(RenderColor::Rgb(163, 190, 140)),
            number: RenderStyle::new().fg(RenderColor::Rgb(180, 142, 173)),
            r#type: RenderStyle::new().fg(RenderColor::Rgb(143, 188, 187)),
            comment: RenderStyle::new()
                .fg(RenderColor::Rgb(76, 86, 106))
                .italic(),
            line_number: RenderStyle::new().fg(RenderColor::Rgb(76, 86, 106)),
            line_number_current: RenderStyle::new()
                .fg(RenderColor::Rgb(216, 222, 233))
                .bold(),
            statusline: RenderStyle::new()
                .fg(RenderColor::Rgb(216, 222, 233))
                .bg(RenderColor::Rgb(59, 66, 82)),
            mode_normal: RenderStyle::new()
                .fg(RenderColor::Rgb(46, 52, 64))
                .bg(RenderColor::Rgb(163, 190, 140))
                .bold(),
            mode_insert: RenderStyle::new()
                .fg(RenderColor::Rgb(46, 52, 64))
                .bg(RenderColor::Rgb(136, 192, 208))
                .bold(),
            mode_visual: RenderStyle::new()
                .fg(RenderColor::Rgb(46, 52, 64))
                .bg(RenderColor::Rgb(180, 142, 173))
                .bold(),
            mode_command: RenderStyle::new()
                .fg(RenderColor::Rgb(46, 52, 64))
                .bg(RenderColor::Rgb(235, 203, 139))
                .bold(),
            selection: RenderStyle::new().bg(RenderColor::Rgb(67, 76, 94)),
            search: RenderStyle::new()
                .fg(RenderColor::Rgb(46, 52, 64))
                .bg(RenderColor::Rgb(235, 203, 139)),
            cursor: RenderStyle::new().reverse(true),
        }
    }
}

/// Extension trait for RenderStyle to add reverse.
trait StyleExt {
    fn reverse(self, val: bool) -> Self;
}

impl StyleExt for RenderStyle {
    fn reverse(mut self, val: bool) -> Self {
        self.reverse = val;
        self
    }
}
