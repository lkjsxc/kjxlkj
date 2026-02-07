//! Full theme implementation with RGB colours and face descriptors.

use serde::{Deserialize, Serialize};

/// RGB triplet.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgb { pub r: u8, pub g: u8, pub b: u8 }

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }

    /// Parse `#RRGGBB` hex string.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 { return None; }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }

    /// Convert to `#RRGGBB` hex string.
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Relative luminance (sRGB approximation, 0.0–1.0).
    pub fn luminance(self) -> f64 {
        0.2126 * (self.r as f64 / 255.0)
            + 0.7152 * (self.g as f64 / 255.0)
            + 0.0722 * (self.b as f64 / 255.0)
    }
}

/// Abstract colour reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeColor {
    Named(String),
    Rgb(Rgb),
    Index(u8),
    Default,
}

/// A face: combined foreground, background, and text attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Face {
    pub fg: ThemeColor,
    pub bg: ThemeColor,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

impl Default for Face {
    fn default() -> Self {
        Self {
            fg: ThemeColor::Default, bg: ThemeColor::Default,
            bold: false, italic: false, underline: false, strikethrough: false,
        }
    }
}

/// Convert a 256-colour index to approximate RGB.
pub fn index_to_rgb(idx: u8) -> Rgb {
    match idx {
        0..=15 => {
            // Standard 16 ANSI colours (rough approximation).
            let table: [(u8,u8,u8); 16] = [
                (0,0,0),(128,0,0),(0,128,0),(128,128,0),(0,0,128),(128,0,128),
                (0,128,128),(192,192,192),(128,128,128),(255,0,0),(0,255,0),
                (255,255,0),(0,0,255),(255,0,255),(0,255,255),(255,255,255),
            ];
            let (r,g,b) = table[idx as usize];
            Rgb::new(r,g,b)
        }
        16..=231 => {
            let n = idx - 16;
            let ri = n / 36;
            let gi = (n % 36) / 6;
            let bi = n % 6;
            let to_val = |v: u8| if v == 0 { 0u8 } else { 55 + 40 * v };
            Rgb::new(to_val(ri), to_val(gi), to_val(bi))
        }
        232..=255 => {
            let v = 8 + 10 * (idx - 232);
            Rgb::new(v, v, v)
        }
    }
}

/// Resolve a [`ThemeColor`] to an RGB value.
pub fn resolve_color(color: &ThemeColor) -> Option<Rgb> {
    match color {
        ThemeColor::Rgb(rgb) => Some(*rgb),
        ThemeColor::Index(idx) => Some(index_to_rgb(*idx)),
        ThemeColor::Named(n) => Rgb::from_hex(n),
        ThemeColor::Default => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        let c = Rgb::new(0xab, 0xcd, 0xef);
        let hex = c.to_hex();
        assert_eq!(Rgb::from_hex(&hex), Some(c));
    }

    #[test]
    fn from_hex_with_hash() {
        assert_eq!(Rgb::from_hex("#ff0000"), Some(Rgb::new(255, 0, 0)));
    }

    #[test]
    fn luminance_black_white() {
        assert!(Rgb::new(0,0,0).luminance() < 0.01);
        assert!(Rgb::new(255,255,255).luminance() > 0.99);
    }

    #[test]
    fn index_to_rgb_standard() {
        assert_eq!(index_to_rgb(0), Rgb::new(0, 0, 0));
        assert_eq!(index_to_rgb(15), Rgb::new(255, 255, 255));
    }

    #[test]
    fn index_to_rgb_cube() {
        let c = index_to_rgb(196); // should be reddish
        assert!(c.r > 100);
    }

    #[test]
    fn index_to_rgb_grey() {
        let c = index_to_rgb(240);
        assert_eq!(c.r, c.g);
        assert_eq!(c.g, c.b);
    }

    #[test]
    fn resolve_default_none() {
        assert!(resolve_color(&ThemeColor::Default).is_none());
    }

    #[test]
    fn resolve_rgb_some() {
        let rgb = Rgb::new(10, 20, 30);
        assert_eq!(resolve_color(&ThemeColor::Rgb(rgb)), Some(rgb));
    }

    #[test]
    fn face_default() {
        let f = Face::default();
        assert!(!f.bold);
        assert_eq!(f.fg, ThemeColor::Default);
    }
}
