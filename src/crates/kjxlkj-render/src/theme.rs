//! Theme system for the editor.
//!
//! Implements theme models as specified in `/docs/spec/ui/themes.md`.

use std::collections::HashMap;

/// Theme color palette.
#[derive(Debug, Clone)]
pub struct Palette {
    /// Named colors.
    colors: HashMap<String, ThemeColor>,
}

impl Default for Palette {
    fn default() -> Self {
        let mut colors = HashMap::new();
        // Default dark palette
        colors.insert("bg".to_string(), ThemeColor::Rgb(30, 30, 46));
        colors.insert("fg".to_string(), ThemeColor::Rgb(205, 214, 244));
        colors.insert("surface".to_string(), ThemeColor::Rgb(49, 50, 68));
        colors.insert("overlay".to_string(), ThemeColor::Rgb(69, 71, 90));
        colors.insert("red".to_string(), ThemeColor::Rgb(243, 139, 168));
        colors.insert("green".to_string(), ThemeColor::Rgb(166, 227, 161));
        colors.insert("yellow".to_string(), ThemeColor::Rgb(249, 226, 175));
        colors.insert("blue".to_string(), ThemeColor::Rgb(137, 180, 250));
        colors.insert("magenta".to_string(), ThemeColor::Rgb(203, 166, 247));
        colors.insert("cyan".to_string(), ThemeColor::Rgb(148, 226, 213));
        colors.insert("comment".to_string(), ThemeColor::Rgb(108, 112, 134));
        Self { colors }
    }
}

impl Palette {
    /// Create a new palette.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a light palette.
    pub fn light() -> Self {
        let mut colors = HashMap::new();
        colors.insert("bg".to_string(), ThemeColor::Rgb(239, 241, 245));
        colors.insert("fg".to_string(), ThemeColor::Rgb(76, 79, 105));
        colors.insert("surface".to_string(), ThemeColor::Rgb(220, 224, 232));
        colors.insert("overlay".to_string(), ThemeColor::Rgb(188, 192, 204));
        colors.insert("red".to_string(), ThemeColor::Rgb(210, 15, 57));
        colors.insert("green".to_string(), ThemeColor::Rgb(64, 160, 43));
        colors.insert("yellow".to_string(), ThemeColor::Rgb(223, 142, 29));
        colors.insert("blue".to_string(), ThemeColor::Rgb(30, 102, 245));
        colors.insert("magenta".to_string(), ThemeColor::Rgb(136, 57, 239));
        colors.insert("cyan".to_string(), ThemeColor::Rgb(23, 146, 153));
        colors.insert("comment".to_string(), ThemeColor::Rgb(140, 143, 161));
        Self { colors }
    }

    /// Get a color by name.
    pub fn get(&self, name: &str) -> Option<&ThemeColor> {
        self.colors.get(name)
    }

    /// Set a color.
    pub fn set(&mut self, name: &str, color: ThemeColor) {
        self.colors.insert(name.to_string(), color);
    }
}

/// Theme color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeColor {
    /// RGB color.
    Rgb(u8, u8, u8),
    /// Indexed color (0-255).
    Indexed(u8),
    /// Named terminal color.
    Named(NamedColor),
    /// No color (inherit).
    #[default]
    None,
}

impl ThemeColor {
    /// Convert to crossterm Color.
    pub fn to_crossterm(&self) -> Option<crossterm::style::Color> {
        use crossterm::style::Color;
        match self {
            ThemeColor::Rgb(r, g, b) => Some(Color::Rgb { r: *r, g: *g, b: *b }),
            ThemeColor::Indexed(i) => Some(Color::AnsiValue(*i)),
            ThemeColor::Named(n) => Some(n.to_crossterm()),
            ThemeColor::None => None,
        }
    }
}

/// Named terminal colors.
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

impl NamedColor {
    /// Convert to crossterm Color.
    pub fn to_crossterm(&self) -> crossterm::style::Color {
        use crossterm::style::Color;
        match self {
            Self::Black => Color::Black,
            Self::Red => Color::DarkRed,
            Self::Green => Color::DarkGreen,
            Self::Yellow => Color::DarkYellow,
            Self::Blue => Color::DarkBlue,
            Self::Magenta => Color::DarkMagenta,
            Self::Cyan => Color::DarkCyan,
            Self::White => Color::Grey,
            Self::BrightBlack => Color::DarkGrey,
            Self::BrightRed => Color::Red,
            Self::BrightGreen => Color::Green,
            Self::BrightYellow => Color::Yellow,
            Self::BrightBlue => Color::Blue,
            Self::BrightMagenta => Color::Magenta,
            Self::BrightCyan => Color::Cyan,
            Self::BrightWhite => Color::White,
        }
    }
}

/// Theme style definition.
#[derive(Debug, Clone, Default)]
pub struct ThemeStyle {
    /// Foreground color.
    pub fg: ThemeColor,
    /// Background color.
    pub bg: ThemeColor,
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
}

impl ThemeStyle {
    /// Create a new theme style.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set foreground.
    pub fn fg(mut self, color: ThemeColor) -> Self {
        self.fg = color;
        self
    }

    /// Set background.
    pub fn bg(mut self, color: ThemeColor) -> Self {
        self.bg = color;
        self
    }

    /// Set bold.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set underline.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
}

/// UI element styles.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name.
    pub name: String,
    /// Color palette.
    pub palette: Palette,
    /// UI element styles.
    styles: HashMap<String, ThemeStyle>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create a dark theme.
    pub fn dark() -> Self {
        let palette = Palette::default();
        let mut styles = HashMap::new();

        // Editor styles
        styles.insert(
            "editor.bg".to_string(),
            ThemeStyle::new().bg(ThemeColor::Rgb(30, 30, 46)),
        );
        styles.insert(
            "editor.fg".to_string(),
            ThemeStyle::new().fg(ThemeColor::Rgb(205, 214, 244)),
        );
        styles.insert(
            "cursor".to_string(),
            ThemeStyle::new()
                .fg(ThemeColor::Rgb(30, 30, 46))
                .bg(ThemeColor::Rgb(205, 214, 244)),
        );
        styles.insert(
            "selection".to_string(),
            ThemeStyle::new().bg(ThemeColor::Rgb(69, 71, 90)),
        );
        styles.insert(
            "line_number".to_string(),
            ThemeStyle::new().fg(ThemeColor::Rgb(108, 112, 134)),
        );
        styles.insert(
            "line_number_active".to_string(),
            ThemeStyle::new().fg(ThemeColor::Rgb(205, 214, 244)),
        );

        // Statusline styles
        styles.insert(
            "statusline".to_string(),
            ThemeStyle::new()
                .fg(ThemeColor::Rgb(205, 214, 244))
                .bg(ThemeColor::Rgb(49, 50, 68)),
        );
        styles.insert(
            "statusline.mode.normal".to_string(),
            ThemeStyle::new()
                .fg(ThemeColor::Rgb(30, 30, 46))
                .bg(ThemeColor::Rgb(137, 180, 250))
                .bold(),
        );
        styles.insert(
            "statusline.mode.insert".to_string(),
            ThemeStyle::new()
                .fg(ThemeColor::Rgb(30, 30, 46))
                .bg(ThemeColor::Rgb(166, 227, 161))
                .bold(),
        );
        styles.insert(
            "statusline.mode.visual".to_string(),
            ThemeStyle::new()
                .fg(ThemeColor::Rgb(30, 30, 46))
                .bg(ThemeColor::Rgb(203, 166, 247))
                .bold(),
        );

        Self {
            name: "dark".to_string(),
            palette,
            styles,
        }
    }

    /// Create a light theme.
    pub fn light() -> Self {
        let palette = Palette::light();
        let mut styles = HashMap::new();

        styles.insert(
            "editor.bg".to_string(),
            ThemeStyle::new().bg(ThemeColor::Rgb(239, 241, 245)),
        );
        styles.insert(
            "editor.fg".to_string(),
            ThemeStyle::new().fg(ThemeColor::Rgb(76, 79, 105)),
        );

        Self {
            name: "light".to_string(),
            palette,
            styles,
        }
    }

    /// Get a style by key.
    pub fn get_style(&self, key: &str) -> Option<&ThemeStyle> {
        self.styles.get(key)
    }

    /// Set a style.
    pub fn set_style(&mut self, key: &str, style: ThemeStyle) {
        self.styles.insert(key.to_string(), style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_default() {
        let palette = Palette::default();
        assert!(palette.get("bg").is_some());
        assert!(palette.get("fg").is_some());
    }

    #[test]
    fn test_palette_light() {
        let palette = Palette::light();
        assert!(palette.get("bg").is_some());
    }

    #[test]
    fn test_theme_color_rgb() {
        let color = ThemeColor::Rgb(255, 128, 64);
        assert!(color.to_crossterm().is_some());
    }

    #[test]
    fn test_theme_color_none() {
        let color = ThemeColor::None;
        assert!(color.to_crossterm().is_none());
    }

    #[test]
    fn test_named_color_crossterm() {
        let color = NamedColor::Red;
        let ct = color.to_crossterm();
        assert_eq!(ct, crossterm::style::Color::DarkRed);
    }

    #[test]
    fn test_theme_style_builder() {
        let style = ThemeStyle::new()
            .fg(ThemeColor::Rgb(255, 0, 0))
            .bold()
            .italic();
        assert!(style.bold);
        assert!(style.italic);
    }

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "dark");
        assert!(theme.get_style("cursor").is_some());
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name, "light");
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.name, "dark");
    }

    #[test]
    fn test_theme_set_style() {
        let mut theme = Theme::dark();
        theme.set_style("custom", ThemeStyle::new().bold());
        assert!(theme.get_style("custom").is_some());
    }
}
