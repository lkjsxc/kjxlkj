//! Color theme definitions.

use serde::{Deserialize, Serialize};

/// A color theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name.
    pub name: String,
    /// Color palette.
    pub palette: Palette,
    /// Highlight styles.
    pub highlights: Highlights,
}

/// Color palette.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Palette {
    /// Background color.
    pub bg: Color,
    /// Foreground color.
    pub fg: Color,
    /// Selection color.
    pub selection: Color,
    /// Cursor color.
    pub cursor: Color,
    /// Comment color.
    pub comment: Color,
    /// Red.
    pub red: Color,
    /// Green.
    pub green: Color,
    /// Yellow.
    pub yellow: Color,
    /// Blue.
    pub blue: Color,
    /// Magenta.
    pub magenta: Color,
    /// Cyan.
    pub cyan: Color,
}

/// A color value (RGB or named).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum Color {
    /// RGB values.
    Rgb { r: u8, g: u8, b: u8 },
    /// Hex string (e.g., "#ff0000").
    Hex(String),
    /// Named color.
    #[default]
    Default,
}

impl Color {
    /// Creates from hex string.
    pub fn from_hex(hex: &str) -> Self {
        Color::Hex(hex.to_string())
    }

    /// Creates from RGB values.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb { r, g, b }
    }
}

/// Highlight styles for syntax elements.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Highlights {
    /// Keywords.
    pub keyword: Style,
    /// Functions.
    pub function: Style,
    /// Strings.
    pub string: Style,
    /// Numbers.
    pub number: Style,
    /// Types.
    pub r#type: Style,
    /// Variables.
    pub variable: Style,
    /// Comments.
    pub comment: Style,
    /// Operators.
    pub operator: Style,
}

/// A text style.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Style {
    /// Foreground color.
    pub fg: Option<String>,
    /// Background color.
    pub bg: Option<String>,
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
}

impl Theme {
    /// Creates a default dark theme.
    pub fn default_dark() -> Self {
        Self {
            name: "default".to_string(),
            palette: Palette::default(),
            highlights: Highlights::default(),
        }
    }
}
