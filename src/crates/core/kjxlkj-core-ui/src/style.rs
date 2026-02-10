//! Style types for rendering.

/// Color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    /// Default terminal color.
    #[default]
    Default,
    /// Named color.
    Named(NamedColor),
    /// 256-color palette index.
    Indexed(u8),
    /// True color RGB.
    Rgb(u8, u8, u8),
}

/// Named ANSI colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Text attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Attributes {
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
    pub hidden: bool,
    pub strikethrough: bool,
}

impl Attributes {
    /// No attributes.
    pub const NONE: Self = Self {
        bold: false,
        dim: false,
        italic: false,
        underline: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Bold attribute.
    pub const BOLD: Self = Self {
        bold: true,
        dim: false,
        italic: false,
        underline: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };
}

/// Complete style for a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    /// Foreground color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Text attributes.
    pub attrs: Attributes,
}

impl Style {
    /// Default style.
    pub fn default_style() -> Self {
        Self::default()
    }

    /// Create style with colors.
    pub fn new(fg: Color, bg: Color) -> Self {
        Self {
            fg,
            bg,
            attrs: Attributes::NONE,
        }
    }

    /// Add bold attribute.
    pub fn bold(mut self) -> Self {
        self.attrs.bold = true;
        self
    }

    /// Add italic attribute.
    pub fn italic(mut self) -> Self {
        self.attrs.italic = true;
        self
    }

    /// Add underline attribute.
    pub fn underline(mut self) -> Self {
        self.attrs.underline = true;
        self
    }

    /// Add reverse attribute.
    pub fn reverse(mut self) -> Self {
        self.attrs.reverse = true;
        self
    }
}
