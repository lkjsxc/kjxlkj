//! Style types.

/// A color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Reset to default.
    Reset,
    /// Black.
    Black,
    /// Red.
    Red,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Blue.
    Blue,
    /// Magenta.
    Magenta,
    /// Cyan.
    Cyan,
    /// White.
    White,
    /// Bright black (gray).
    BrightBlack,
    /// Bright red.
    BrightRed,
    /// Bright green.
    BrightGreen,
    /// Bright yellow.
    BrightYellow,
    /// Bright blue.
    BrightBlue,
    /// Bright magenta.
    BrightMagenta,
    /// Bright cyan.
    BrightCyan,
    /// Bright white.
    BrightWhite,
    /// 256-color index.
    Indexed(u8),
    /// RGB color.
    Rgb(u8, u8, u8),
}

impl Color {
    /// Converts to crossterm color.
    pub fn to_crossterm(self) -> crossterm::style::Color {
        match self {
            Self::Reset => crossterm::style::Color::Reset,
            Self::Black => crossterm::style::Color::Black,
            Self::Red => crossterm::style::Color::DarkRed,
            Self::Green => crossterm::style::Color::DarkGreen,
            Self::Yellow => crossterm::style::Color::DarkYellow,
            Self::Blue => crossterm::style::Color::DarkBlue,
            Self::Magenta => crossterm::style::Color::DarkMagenta,
            Self::Cyan => crossterm::style::Color::DarkCyan,
            Self::White => crossterm::style::Color::Grey,
            Self::BrightBlack => crossterm::style::Color::DarkGrey,
            Self::BrightRed => crossterm::style::Color::Red,
            Self::BrightGreen => crossterm::style::Color::Green,
            Self::BrightYellow => crossterm::style::Color::Yellow,
            Self::BrightBlue => crossterm::style::Color::Blue,
            Self::BrightMagenta => crossterm::style::Color::Magenta,
            Self::BrightCyan => crossterm::style::Color::Cyan,
            Self::BrightWhite => crossterm::style::Color::White,
            Self::Indexed(i) => crossterm::style::Color::AnsiValue(i),
            Self::Rgb(r, g, b) => crossterm::style::Color::Rgb { r, g, b },
        }
    }
}

/// Text style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    /// Strikethrough.
    pub strikethrough: bool,
    /// Reverse video.
    pub reverse: bool,
}

impl Style {
    /// Creates a new empty style.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets bold.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Sets italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Sets underline.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
}
