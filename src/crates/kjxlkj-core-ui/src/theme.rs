//! Theme data model and built-in theme definitions.

use kjxlkj_core_types::{Color, HighlightGroup, Style};
use std::collections::HashMap;

/// A named color theme containing styles for all highlight groups.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub palette: ThemePalette,
    overrides: HashMap<HighlightGroup, Style>,
}

/// Core semantic colors for a theme.
#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub fg: Color, pub bg: Color, pub cursor: Color, pub selection: Color,
    pub comment: Color, pub keyword: Color, pub string: Color, pub number: Color,
    pub r#type: Color, pub function: Color, pub error: Color, pub warning: Color,
    pub info: Color, pub hint: Color, pub line_nr: Color,
    pub status_fg: Color, pub status_bg: Color,
}

impl Theme {
    pub fn new(name: impl Into<String>, palette: ThemePalette) -> Self {
        Self { name: name.into(), palette, overrides: HashMap::new() }
    }
    /// Resolve a style for a highlight group: override first, then palette, then default.
    pub fn resolve(&self, group: HighlightGroup) -> Style {
        if let Some(s) = self.overrides.get(&group) { return *s; }
        self.palette.style_for(group)
    }
    pub fn set_override(&mut self, group: HighlightGroup, style: Style) {
        self.overrides.insert(group, style);
    }
}

impl ThemePalette {
    pub fn style_for(&self, group: HighlightGroup) -> Style {
        match group {
            HighlightGroup::Normal => Style::default().fg(self.fg).bg(self.bg),
            HighlightGroup::Comment => Style::default().fg(self.comment).italic(),
            HighlightGroup::Keyword => Style::default().fg(self.keyword).bold(),
            HighlightGroup::String => Style::default().fg(self.string),
            HighlightGroup::Number => Style::default().fg(self.number),
            HighlightGroup::Type => Style::default().fg(self.r#type),
            HighlightGroup::Function => Style::default().fg(self.function),
            HighlightGroup::Error => Style::default().fg(self.error).bold(),
            HighlightGroup::Warning => Style::default().fg(self.warning),
            HighlightGroup::LineNr => Style::default().fg(self.line_nr),
            HighlightGroup::StatusLine => Style::default().fg(self.status_fg).bg(self.status_bg),
            HighlightGroup::Visual => Style::default().bg(self.selection),
            HighlightGroup::Search => Style::default().fg(self.bg).bg(self.warning),
            _ => group.default_style(),
        }
    }
}

/// Built-in dark theme (default).
pub fn theme_dark() -> Theme {
    Theme::new("dark", ThemePalette {
        fg: Color::White, bg: Color::Black, cursor: Color::White,
        selection: Color::Blue, comment: Color::DarkGrey, keyword: Color::Blue,
        string: Color::Green, number: Color::Magenta, r#type: Color::Cyan,
        function: Color::Yellow, error: Color::Red, warning: Color::Yellow,
        info: Color::Cyan, hint: Color::DarkGrey, line_nr: Color::DarkGrey,
        status_fg: Color::Black, status_bg: Color::White,
    })
}

/// Built-in light theme.
pub fn theme_light() -> Theme {
    Theme::new("light", ThemePalette {
        fg: Color::Black, bg: Color::White, cursor: Color::Black,
        selection: Color::LightBlue, comment: Color::DarkGrey, keyword: Color::Blue,
        string: Color::Green, number: Color::Magenta, r#type: Color::Cyan,
        function: Color::Red, error: Color::Red, warning: Color::Yellow,
        info: Color::Cyan, hint: Color::DarkGrey, line_nr: Color::DarkGrey,
        status_fg: Color::White, status_bg: Color::Black,
    })
}

/// Built-in gruvbox theme.
pub fn theme_gruvbox() -> Theme {
    Theme::new("gruvbox", ThemePalette {
        fg: Color::Rgb(235, 219, 178), bg: Color::Rgb(40, 40, 40),
        cursor: Color::Rgb(235, 219, 178), selection: Color::Rgb(60, 56, 54),
        comment: Color::Rgb(146, 131, 116), keyword: Color::Rgb(251, 73, 52),
        string: Color::Rgb(184, 187, 38), number: Color::Rgb(211, 134, 155),
        r#type: Color::Rgb(250, 189, 47), function: Color::Rgb(131, 165, 152),
        error: Color::Rgb(251, 73, 52), warning: Color::Rgb(250, 189, 47),
        info: Color::Rgb(131, 165, 152), hint: Color::Rgb(146, 131, 116),
        line_nr: Color::Rgb(124, 111, 100), status_fg: Color::Rgb(235, 219, 178),
        status_bg: Color::Rgb(80, 73, 69),
    })
}

/// Registry of available themes.
pub struct ThemeRegistry { themes: HashMap<String, Theme>, active: String }

impl ThemeRegistry {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        for t in [theme_dark(), theme_light(), theme_gruvbox()] {
            themes.insert(t.name.clone(), t);
        }
        Self { themes, active: "dark".into() }
    }
    pub fn active(&self) -> &Theme { &self.themes[&self.active] }
    pub fn set_active(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) { self.active = name.into(); true } else { false }
    }
    pub fn register(&mut self, theme: Theme) { self.themes.insert(theme.name.clone(), theme); }
    pub fn names(&self) -> Vec<&str> { self.themes.keys().map(|s| s.as_str()).collect() }
}

impl Default for ThemeRegistry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_theme_normal() {
        let t = theme_dark();
        let s = t.resolve(HighlightGroup::Normal);
        assert_eq!(s.fg, Some(Color::White));
        assert_eq!(s.bg, Some(Color::Black));
    }

    #[test]
    fn light_theme_inverted() {
        let t = theme_light();
        let s = t.resolve(HighlightGroup::Normal);
        assert_eq!(s.fg, Some(Color::Black));
        assert_eq!(s.bg, Some(Color::White));
    }

    #[test]
    fn gruvbox_uses_rgb() {
        let t = theme_gruvbox();
        let s = t.resolve(HighlightGroup::Normal);
        assert!(matches!(s.fg, Some(Color::Rgb(_, _, _))));
    }

    #[test]
    fn override_takes_precedence() {
        let mut t = theme_dark();
        let custom = Style::default().fg(Color::Magenta).bold();
        t.set_override(HighlightGroup::Keyword, custom);
        let s = t.resolve(HighlightGroup::Keyword);
        assert_eq!(s.fg, Some(Color::Magenta));
        assert!(s.bold);
    }

    #[test]
    fn registry_switch_theme() {
        let mut reg = ThemeRegistry::new();
        assert_eq!(reg.active().name, "dark");
        assert!(reg.set_active("light"));
        assert_eq!(reg.active().name, "light");
        assert!(!reg.set_active("nonexistent"));
    }

    #[test]
    fn registry_names() {
        let reg = ThemeRegistry::new();
        let mut names = reg.names();
        names.sort();
        assert!(names.contains(&"dark"));
        assert!(names.contains(&"light"));
        assert!(names.contains(&"gruvbox"));
    }

    #[test]
    fn comment_is_italic() {
        let t = theme_dark();
        let s = t.resolve(HighlightGroup::Comment);
        assert!(s.italic);
    }

    #[test]
    fn keyword_is_bold() {
        let t = theme_dark();
        let s = t.resolve(HighlightGroup::Keyword);
        assert!(s.bold);
    }
}
