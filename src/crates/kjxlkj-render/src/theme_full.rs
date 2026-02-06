/// Full theme rendering — palette, face resolution, terminal color mapping.

/// 256-color index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorIndex(pub u8);

/// RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb { pub r: u8, pub g: u8, pub b: u8 }

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn hex(&self) -> String { format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b) }
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 { return None; }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }
    pub fn luminance(&self) -> f64 { 0.2126 * (self.r as f64 / 255.0) + 0.7152 * (self.g as f64 / 255.0) + 0.0722 * (self.b as f64 / 255.0) }
    pub fn is_dark(&self) -> bool { self.luminance() < 0.5 }
}

/// Theme color reference — resolved at render time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeColor { Named(String), Rgb(Rgb), Index(ColorIndex), Default }

/// A face (foreground + background + attributes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face { pub fg: ThemeColor, pub bg: ThemeColor, pub bold: bool, pub italic: bool, pub underline: bool, pub strikethrough: bool }

impl Default for Face {
    fn default() -> Self {
        Self { fg: ThemeColor::Default, bg: ThemeColor::Default, bold: false, italic: false, underline: false, strikethrough: false }
    }
}

impl Face {
    pub fn with_fg(mut self, fg: ThemeColor) -> Self { self.fg = fg; self }
    pub fn with_bg(mut self, bg: ThemeColor) -> Self { self.bg = bg; self }
    pub fn bold(mut self) -> Self { self.bold = true; self }
    pub fn italic(mut self) -> Self { self.italic = true; self }
    pub fn underline(mut self) -> Self { self.underline = true; self }
}

/// Map a 256-color index to approximate RGB.
pub fn index_to_rgb(idx: u8) -> Rgb {
    match idx {
        0 => Rgb::new(0, 0, 0), 1 => Rgb::new(128, 0, 0), 2 => Rgb::new(0, 128, 0),
        3 => Rgb::new(128, 128, 0), 4 => Rgb::new(0, 0, 128), 5 => Rgb::new(128, 0, 128),
        6 => Rgb::new(0, 128, 128), 7 => Rgb::new(192, 192, 192),
        16..=231 => {
            let n = idx - 16;
            let b = (n % 6) * 51; let g = ((n / 6) % 6) * 51; let r = (n / 36) * 51;
            Rgb::new(r, g, b)
        }
        232..=255 => { let v = 8 + (idx - 232) * 10; Rgb::new(v, v, v) }
        _ => Rgb::new(128, 128, 128),
    }
}

/// Resolve a ThemeColor to RGB (with default fallback).
pub fn resolve_color(color: &ThemeColor, default: Rgb) -> Rgb {
    match color {
        ThemeColor::Rgb(rgb) => *rgb,
        ThemeColor::Index(ColorIndex(idx)) => index_to_rgb(*idx),
        ThemeColor::Default | ThemeColor::Named(_) => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hex() {
        let c = Rgb::new(255, 128, 0);
        assert_eq!(c.hex(), "#ff8000");
    }

    #[test]
    fn rgb_from_hex() {
        let c = Rgb::from_hex("#ff8000").unwrap();
        assert_eq!(c.r, 255); assert_eq!(c.g, 128); assert_eq!(c.b, 0);
    }

    #[test]
    fn luminance_dark() { assert!(Rgb::new(0, 0, 0).is_dark()); }
    #[test]
    fn luminance_light() { assert!(!Rgb::new(255, 255, 255).is_dark()); }

    #[test]
    fn face_builder() {
        let f = Face::default().with_fg(ThemeColor::Rgb(Rgb::new(255, 0, 0))).bold();
        assert!(f.bold); assert!(!f.italic);
    }

    #[test]
    fn index_to_rgb_black() { assert_eq!(index_to_rgb(0), Rgb::new(0, 0, 0)); }

    #[test]
    fn index_to_rgb_grayscale() {
        let c = index_to_rgb(232); assert_eq!(c.r, 8);
    }

    #[test]
    fn resolve_default() {
        let d = Rgb::new(200, 200, 200);
        assert_eq!(resolve_color(&ThemeColor::Default, d), d);
    }

    #[test]
    fn resolve_rgb() {
        let c = Rgb::new(10, 20, 30);
        assert_eq!(resolve_color(&ThemeColor::Rgb(c), Rgb::new(0, 0, 0)), c);
    }
}
