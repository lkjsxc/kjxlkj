//! Theme engine — color resolution, theme loading, customization.

/// An RGBA color value for theme styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeColor { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

impl ThemeColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, a: 255 } }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    /// Convert to crossterm-compatible 256-color index (nearest).
    pub fn to_ansi256(&self) -> u8 {
        if self.r == self.g && self.g == self.b {
            let gray = self.r as u16;
            if gray < 8 { return 16; }
            if gray > 248 { return 231; }
            return (((gray - 8) * 24 / 240) + 232) as u8;
        }
        let r = (self.r as u16 * 5 / 255) as u8;
        let g = (self.g as u16 * 5 / 255) as u8;
        let b = (self.b as u16 * 5 / 255) as u8;
        16 + 36 * r + 6 * g + b
    }
}

/// A style rule mapping a scope to colors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRule {
    pub scope: String,
    pub fg: Option<ThemeColor>,
    pub bg: Option<ThemeColor>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// A complete theme definition.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub is_dark: bool,
    pub rules: Vec<StyleRule>,
    pub editor_bg: ThemeColor,
    pub editor_fg: ThemeColor,
    pub cursor_color: ThemeColor,
    pub selection_bg: ThemeColor,
    pub statusline_bg: ThemeColor,
    pub statusline_fg: ThemeColor,
    pub line_number_fg: ThemeColor,
}

/// Build the default dark theme.
pub fn default_dark_theme() -> Theme {
    Theme {
        name: "dark-default".into(),
        is_dark: true,
        rules: vec![
            StyleRule { scope: "comment".into(), fg: Some(ThemeColor::rgb(106, 153, 85)), bg: None, bold: false, italic: true, underline: false },
            StyleRule { scope: "keyword".into(), fg: Some(ThemeColor::rgb(86, 156, 214)), bg: None, bold: true, italic: false, underline: false },
            StyleRule { scope: "string".into(), fg: Some(ThemeColor::rgb(206, 145, 120)), bg: None, bold: false, italic: false, underline: false },
            StyleRule { scope: "number".into(), fg: Some(ThemeColor::rgb(181, 206, 168)), bg: None, bold: false, italic: false, underline: false },
            StyleRule { scope: "function".into(), fg: Some(ThemeColor::rgb(220, 220, 170)), bg: None, bold: false, italic: false, underline: false },
            StyleRule { scope: "type".into(), fg: Some(ThemeColor::rgb(78, 201, 176)), bg: None, bold: false, italic: false, underline: false },
        ],
        editor_bg: ThemeColor::rgb(30, 30, 30), editor_fg: ThemeColor::rgb(212, 212, 212),
        cursor_color: ThemeColor::rgb(255, 255, 255), selection_bg: ThemeColor::rgb(38, 79, 120),
        statusline_bg: ThemeColor::rgb(0, 122, 204), statusline_fg: ThemeColor::rgb(255, 255, 255),
        line_number_fg: ThemeColor::rgb(133, 133, 133),
    }
}

/// Build a default light theme.
pub fn default_light_theme() -> Theme {
    Theme {
        name: "light-default".into(),
        is_dark: false,
        rules: vec![
            StyleRule { scope: "comment".into(), fg: Some(ThemeColor::rgb(0, 128, 0)), bg: None, bold: false, italic: true, underline: false },
            StyleRule { scope: "keyword".into(), fg: Some(ThemeColor::rgb(0, 0, 255)), bg: None, bold: true, italic: false, underline: false },
            StyleRule { scope: "string".into(), fg: Some(ThemeColor::rgb(163, 21, 21)), bg: None, bold: false, italic: false, underline: false },
        ],
        editor_bg: ThemeColor::rgb(255, 255, 255), editor_fg: ThemeColor::rgb(0, 0, 0),
        cursor_color: ThemeColor::rgb(0, 0, 0), selection_bg: ThemeColor::rgb(173, 214, 255),
        statusline_bg: ThemeColor::rgb(0, 122, 204), statusline_fg: ThemeColor::rgb(255, 255, 255),
        line_number_fg: ThemeColor::rgb(37, 127, 125),
    }
}

/// Resolve a scope name to its style from the theme rules.
pub fn resolve_scope<'a>(theme: &'a Theme, scope: &str) -> Option<&'a StyleRule> {
    theme.rules.iter().find(|r| r.scope == scope)
}

/// Merge a user override rule into a copy of a theme.
pub fn apply_override(theme: &mut Theme, rule: StyleRule) {
    if let Some(existing) = theme.rules.iter_mut().find(|r| r.scope == rule.scope) {
        *existing = rule;
    } else {
        theme.rules.push(rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_theme_basics() {
        let t = default_dark_theme();
        assert!(t.is_dark);
        assert!(t.rules.len() >= 5);
    }

    #[test]
    fn light_theme_basics() {
        let t = default_light_theme();
        assert!(!t.is_dark);
        assert!(t.rules.len() >= 3);
    }

    #[test]
    fn resolve_keyword_scope() {
        let t = default_dark_theme();
        let r = resolve_scope(&t, "keyword").unwrap();
        assert!(r.bold);
    }

    #[test]
    fn resolve_missing_scope() {
        let t = default_dark_theme();
        assert!(resolve_scope(&t, "nonexistent").is_none());
    }

    #[test]
    fn apply_override_existing() {
        let mut t = default_dark_theme();
        let rule = StyleRule { scope: "keyword".into(), fg: Some(ThemeColor::rgb(255, 0, 0)), bg: None, bold: false, italic: false, underline: false };
        apply_override(&mut t, rule);
        let r = resolve_scope(&t, "keyword").unwrap();
        assert_eq!(r.fg.unwrap().r, 255);
        assert!(!r.bold);
    }

    #[test]
    fn apply_override_new() {
        let mut t = default_dark_theme();
        let len_before = t.rules.len();
        let rule = StyleRule { scope: "custom".into(), fg: None, bg: None, bold: true, italic: false, underline: false };
        apply_override(&mut t, rule);
        assert_eq!(t.rules.len(), len_before + 1);
    }

    #[test]
    fn ansi256_pure_red() {
        let c = ThemeColor::rgb(255, 0, 0);
        let idx = c.to_ansi256();
        assert_eq!(idx, 196); // 16 + 36*5 + 6*0 + 0
    }

    #[test]
    fn ansi256_grayscale() {
        let c = ThemeColor::rgb(128, 128, 128);
        let idx = c.to_ansi256();
        assert!(idx >= 232 && idx <= 255);
    }
}
