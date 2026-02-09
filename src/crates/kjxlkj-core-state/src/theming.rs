//! Theming system per /docs/spec/ux/theming.md.

use std::collections::HashMap;

use kjxlkj_core_types::Color;

#[derive(Debug, Clone, Default)]
#[rustfmt::skip]
pub struct ThemeStyle {
    pub fg: Option<Color>, pub bg: Option<Color>, pub bold: bool, pub italic: bool, pub underline: bool,
    pub strikethrough: bool, pub link: Option<String>,
}

#[derive(Debug, Clone)]
#[rustfmt::skip]
pub struct Theme {
    pub name: String, pub dark: bool, pub default_fg: Color, pub default_bg: Color,
    pub groups: HashMap<String, ThemeStyle>,
}

impl Theme {
    pub fn dark_default() -> Self {
        let mut groups = HashMap::new();
        groups.insert(
            "Normal".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(248, 248, 242)),
                bg: Some(Color::Rgb(40, 42, 54)),
                ..Default::default()
            },
        );
        groups.insert(
            "Comment".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(98, 114, 164)),
                italic: true,
                ..Default::default()
            },
        );
        groups.insert(
            "Keyword".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(255, 121, 198)),
                bold: true,
                ..Default::default()
            },
        );
        groups.insert(
            "String".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(241, 250, 140)),
                ..Default::default()
            },
        );
        groups.insert(
            "Function".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(80, 250, 123)),
                ..Default::default()
            },
        );
        groups.insert(
            "Number".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(189, 147, 249)),
                ..Default::default()
            },
        );
        groups.insert(
            "Error".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(255, 85, 85)),
                bold: true,
                ..Default::default()
            },
        );
        groups.insert(
            "Warning".into(),
            ThemeStyle {
                fg: Some(Color::Rgb(255, 184, 108)),
                ..Default::default()
            },
        );
        Self {
            name: "default-dark".into(),
            dark: true,
            default_fg: Color::Rgb(248, 248, 242),
            default_bg: Color::Rgb(40, 42, 54),
            groups,
        }
    }

    pub fn resolve(&self, group: &str) -> ThemeStyle {
        if let Some(style) = self.groups.get(group) {
            if let Some(ref link) = style.link {
                return self.resolve(link);
            }
            return style.clone();
        }
        ThemeStyle {
            fg: Some(self.default_fg),
            bg: Some(self.default_bg),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
#[rustfmt::skip]
pub struct ThemeRegistry {
    pub themes: Vec<Theme>, pub active: usize,
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self {
            themes: vec![Theme::dark_default()],
            active: 0,
        }
    }
}

impl ThemeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active_theme(&self) -> &Theme {
        &self.themes[self.active]
    }

    pub fn set_theme(&mut self, name: &str) -> bool {
        if let Some(idx) = self.themes.iter().position(|t| t.name == name) {
            self.active = idx;
            true
        } else {
            false
        }
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_default_has_groups() {
        let theme = Theme::dark_default();
        assert!(theme.groups.contains_key("Normal"));
        assert!(theme.groups.contains_key("Comment"));
        assert!(theme.dark);
    }

    #[test]
    fn resolve_known_group() {
        let theme = Theme::dark_default();
        let style = theme.resolve("Keyword");
        assert!(style.bold);
        assert!(style.fg.is_some());
    }

    #[test]
    fn resolve_unknown_uses_default() {
        let theme = Theme::dark_default();
        let style = theme.resolve("Unknown");
        assert_eq!(style.fg, Some(theme.default_fg));
    }
}
