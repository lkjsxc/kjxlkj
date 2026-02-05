//! Color picker types.
//!
//! Implements color picker as specified in `/docs/spec/features/ui/color-picker.md`.

/// Color representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0.0-1.0).
    pub a: f32,
}

impl Color {
    /// Create a new RGB color.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a new RGBA color.
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a: a.clamp(0.0, 1.0) }
    }

    /// Parse a hex color string.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::rgba(r, g, b, a as f32 / 255.0))
            }
            3 => {
                // Short form (#RGB)
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::rgb(r, g, b))
            }
            _ => None,
        }
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            let a = (self.a * 255.0) as u8;
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, a)
        }
    }

    /// Convert to RGB string.
    pub fn to_rgb_string(&self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            format!("rgb({}, {}, {})", self.r, self.g, self.b)
        } else {
            format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        }
    }

    /// Convert to HSL.
    pub fn to_hsl(&self) -> (u16, u8, u8) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if (max - min).abs() < f32::EPSILON {
            return (0, 0, (l * 100.0) as u8);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if (max - r).abs() < f32::EPSILON {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if (max - g).abs() < f32::EPSILON {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };

        ((h * 60.0) as u16, (s * 100.0) as u8, (l * 100.0) as u8)
    }

    /// Create from HSL values.
    pub fn from_hsl(h: u16, s: u8, l: u8) -> Self {
        let h = (h % 360) as f32 / 360.0;
        let s = (s.min(100)) as f32 / 100.0;
        let l = (l.min(100)) as f32 / 100.0;

        if s == 0.0 {
            let v = (l * 255.0) as u8;
            return Self::rgb(v, v, v);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let hue_to_rgb = |p: f32, q: f32, mut t: f32| -> f32 {
            if t < 0.0 { t += 1.0; }
            if t > 1.0 { t -= 1.0; }
            if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
            if t < 1.0/2.0 { return q; }
            if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
            p
        };

        let r = (hue_to_rgb(p, q, h + 1.0/3.0) * 255.0) as u8;
        let g = (hue_to_rgb(p, q, h) * 255.0) as u8;
        let b = (hue_to_rgb(p, q, h - 1.0/3.0) * 255.0) as u8;

        Self::rgb(r, g, b)
    }

    /// Get named color.
    pub fn named(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "red" => Some(Self::rgb(255, 0, 0)),
            "green" => Some(Self::rgb(0, 128, 0)),
            "blue" => Some(Self::rgb(0, 0, 255)),
            "white" => Some(Self::rgb(255, 255, 255)),
            "black" => Some(Self::rgb(0, 0, 0)),
            "yellow" => Some(Self::rgb(255, 255, 0)),
            "cyan" => Some(Self::rgb(0, 255, 255)),
            "magenta" => Some(Self::rgb(255, 0, 255)),
            "orange" => Some(Self::rgb(255, 165, 0)),
            "purple" => Some(Self::rgb(128, 0, 128)),
            "gray" | "grey" => Some(Self::rgb(128, 128, 128)),
            _ => None,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

/// Color format detected in text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    /// Hex format (#RRGGBB).
    Hex,
    /// Short hex format (#RGB).
    HexShort,
    /// RGB function.
    Rgb,
    /// RGBA function.
    Rgba,
    /// HSL function.
    Hsl,
    /// Named color.
    Named,
}

/// A detected color in a document.
#[derive(Debug, Clone)]
pub struct ColorMatch {
    /// Start offset in the line.
    pub start: usize,
    /// End offset in the line.
    pub end: usize,
    /// Detected format.
    pub format: ColorFormat,
    /// Parsed color value.
    pub color: Color,
    /// Original text.
    pub text: String,
}

impl ColorMatch {
    /// Create a new color match.
    pub fn new(start: usize, end: usize, format: ColorFormat, color: Color, text: String) -> Self {
        Self { start, end, format, color, text }
    }

    /// Get the span length.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the match is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Color picker state.
#[derive(Debug, Clone)]
pub struct ColorPicker {
    /// Currently selected color.
    pub color: Color,
    /// Current component being edited.
    pub component: ColorComponent,
    /// Whether picker is open.
    pub open: bool,
    /// Target match in buffer.
    pub target: Option<ColorMatch>,
}

/// Color component for editing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorComponent {
    /// Red component.
    #[default]
    Red,
    /// Green component.
    Green,
    /// Blue component.
    Blue,
    /// Alpha component.
    Alpha,
    /// Hue (HSL).
    Hue,
    /// Saturation (HSL).
    Saturation,
    /// Lightness (HSL).
    Lightness,
}

impl ColorPicker {
    /// Create a new color picker.
    pub fn new(color: Color) -> Self {
        Self {
            color,
            component: ColorComponent::Red,
            open: false,
            target: None,
        }
    }

    /// Open the picker with a target match.
    pub fn open(&mut self, target: ColorMatch) {
        self.color = target.color;
        self.target = Some(target);
        self.open = true;
    }

    /// Close the picker.
    pub fn close(&mut self) {
        self.open = false;
        self.target = None;
    }

    /// Move to next component.
    pub fn next_component(&mut self) {
        self.component = match self.component {
            ColorComponent::Red => ColorComponent::Green,
            ColorComponent::Green => ColorComponent::Blue,
            ColorComponent::Blue => ColorComponent::Alpha,
            ColorComponent::Alpha => ColorComponent::Red,
            ColorComponent::Hue => ColorComponent::Saturation,
            ColorComponent::Saturation => ColorComponent::Lightness,
            ColorComponent::Lightness => ColorComponent::Hue,
        };
    }

    /// Move to previous component.
    pub fn prev_component(&mut self) {
        self.component = match self.component {
            ColorComponent::Red => ColorComponent::Alpha,
            ColorComponent::Green => ColorComponent::Red,
            ColorComponent::Blue => ColorComponent::Green,
            ColorComponent::Alpha => ColorComponent::Blue,
            ColorComponent::Hue => ColorComponent::Lightness,
            ColorComponent::Saturation => ColorComponent::Hue,
            ColorComponent::Lightness => ColorComponent::Saturation,
        };
    }

    /// Increase current component.
    pub fn increase(&mut self, amount: u8) {
        match self.component {
            ColorComponent::Red => self.color.r = self.color.r.saturating_add(amount),
            ColorComponent::Green => self.color.g = self.color.g.saturating_add(amount),
            ColorComponent::Blue => self.color.b = self.color.b.saturating_add(amount),
            ColorComponent::Alpha => {
                self.color.a = (self.color.a + amount as f32 / 255.0).min(1.0);
            }
            _ => {} // HSL handled separately
        }
    }

    /// Decrease current component.
    pub fn decrease(&mut self, amount: u8) {
        match self.component {
            ColorComponent::Red => self.color.r = self.color.r.saturating_sub(amount),
            ColorComponent::Green => self.color.g = self.color.g.saturating_sub(amount),
            ColorComponent::Blue => self.color.b = self.color.b.saturating_sub(amount),
            ColorComponent::Alpha => {
                self.color.a = (self.color.a - amount as f32 / 255.0).max(0.0);
            }
            _ => {} // HSL handled separately
        }
    }
}

/// Color picker configuration.
#[derive(Debug, Clone)]
pub struct ColorPickerConfig {
    /// Enable inline color preview.
    pub preview: bool,
    /// Preview style.
    pub preview_style: ColorPreviewStyle,
    /// Enable color picker.
    pub picker: bool,
    /// Enabled file types.
    pub filetypes: Vec<String>,
}

/// Color preview style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPreviewStyle {
    /// Square block.
    #[default]
    Square,
    /// Background highlight.
    Background,
    /// Foreground (text) color.
    Foreground,
}

impl Default for ColorPickerConfig {
    fn default() -> Self {
        Self {
            preview: true,
            preview_style: ColorPreviewStyle::Square,
            picker: true,
            filetypes: vec![
                "css".to_string(),
                "scss".to_string(),
                "less".to_string(),
                "html".to_string(),
                "javascript".to_string(),
                "typescript".to_string(),
            ],
        }
    }
}

impl ColorPickerConfig {
    /// Check if colors should be shown for a filetype.
    pub fn enabled_for(&self, filetype: &str) -> bool {
        self.filetypes.iter().any(|f| f == filetype)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_rgb() {
        let c = Color::rgb(255, 128, 64);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 64);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_color_rgba() {
        let c = Color::rgba(255, 128, 64, 0.5);
        assert_eq!(c.a, 0.5);
    }

    #[test]
    fn test_color_from_hex() {
        let c = Color::from_hex("#ff5370").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 83);
        assert_eq!(c.b, 112);
    }

    #[test]
    fn test_color_from_hex_short() {
        let c = Color::from_hex("#f00").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_color_from_hex_with_alpha() {
        let c = Color::from_hex("#ff000080").unwrap();
        assert_eq!(c.a, 128.0 / 255.0);
    }

    #[test]
    fn test_color_to_hex() {
        let c = Color::rgb(255, 83, 112);
        assert_eq!(c.to_hex(), "#ff5370");
    }

    #[test]
    fn test_color_to_rgb_string() {
        let c = Color::rgb(255, 128, 64);
        assert_eq!(c.to_rgb_string(), "rgb(255, 128, 64)");
    }

    #[test]
    fn test_color_to_rgba_string() {
        let c = Color::rgba(255, 128, 64, 0.5);
        assert!(c.to_rgb_string().starts_with("rgba"));
    }

    #[test]
    fn test_color_named() {
        let c = Color::named("red").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_color_named_case_insensitive() {
        assert!(Color::named("RED").is_some());
        assert!(Color::named("Red").is_some());
    }

    #[test]
    fn test_color_named_unknown() {
        assert!(Color::named("unknowncolor").is_none());
    }

    #[test]
    fn test_color_hsl_conversion() {
        let c = Color::rgb(255, 0, 0);
        let (h, s, l) = c.to_hsl();
        assert_eq!(h, 0);
        assert!(s > 90);
    }

    #[test]
    fn test_color_from_hsl() {
        let c = Color::from_hsl(0, 100, 50);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_color_match_len() {
        let m = ColorMatch::new(0, 7, ColorFormat::Hex, Color::rgb(0, 0, 0), "#000000".to_string());
        assert_eq!(m.len(), 7);
    }

    #[test]
    fn test_color_picker_new() {
        let p = ColorPicker::new(Color::rgb(255, 0, 0));
        assert_eq!(p.color.r, 255);
        assert!(!p.open);
    }

    #[test]
    fn test_color_picker_next_component() {
        let mut p = ColorPicker::new(Color::rgb(0, 0, 0));
        assert_eq!(p.component, ColorComponent::Red);
        p.next_component();
        assert_eq!(p.component, ColorComponent::Green);
    }

    #[test]
    fn test_color_picker_increase() {
        let mut p = ColorPicker::new(Color::rgb(100, 0, 0));
        p.increase(10);
        assert_eq!(p.color.r, 110);
    }

    #[test]
    fn test_color_picker_decrease() {
        let mut p = ColorPicker::new(Color::rgb(100, 0, 0));
        p.decrease(10);
        assert_eq!(p.color.r, 90);
    }

    #[test]
    fn test_color_picker_config_default() {
        let c = ColorPickerConfig::default();
        assert!(c.preview);
        assert!(c.picker);
    }

    #[test]
    fn test_color_picker_config_enabled_for() {
        let c = ColorPickerConfig::default();
        assert!(c.enabled_for("css"));
        assert!(!c.enabled_for("rust"));
    }
}
