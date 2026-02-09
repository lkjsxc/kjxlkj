/// Color and style types for theming.
/// A color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    /// 24-bit RGB.
    Rgb(u8, u8, u8),
    /// ANSI 256 color index.
    Indexed(u8),
    /// Default terminal color.
    #[default]
    Default,
}

/// Text style attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub reverse: bool,
}

/// Theme definition.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub default_style: Style,
    pub cursor_style: Style,
    pub statusline_style: Style,
    pub statusline_nc_style: Style,
    pub line_number_style: Style,
    pub visual_style: Style,
    pub search_style: Style,
    pub cmdline_style: Style,
    pub tab_line_style: Style,
    pub tab_line_sel_style: Style,
    pub tab_line_fill_style: Style,
    pub win_separator_style: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            default_style: Style::default(),
            cursor_style: Style {
                reverse: true,
                ..Style::default()
            },
            statusline_style: Style {
                bold: true,
                reverse: true,
                ..Style::default()
            },
            statusline_nc_style: Style {
                reverse: true,
                ..Style::default()
            },
            line_number_style: Style {
                fg: Color::Indexed(242),
                ..Style::default()
            },
            visual_style: Style {
                bg: Color::Indexed(238),
                ..Style::default()
            },
            search_style: Style {
                bg: Color::Indexed(220),
                fg: Color::Indexed(0),
                ..Style::default()
            },
            cmdline_style: Style::default(),
            tab_line_style: Style::default(),
            tab_line_sel_style: Style {
                bold: true,
                ..Style::default()
            },
            tab_line_fill_style: Style::default(),
            win_separator_style: Style {
                fg: Color::Indexed(242),
                ..Style::default()
            },
        }
    }
}
