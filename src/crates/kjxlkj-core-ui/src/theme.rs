//! Theme model types: palettes, styles, and registry.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Color palette for a theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePalette {
    pub fg: String,
    pub bg: String,
    pub cursor: String,
    pub selection: String,
    pub keyword: String,
    pub string: String,
    pub comment: String,
    pub function: String,
    pub type_color: String,
    pub number: String,
    pub operator: String,
    pub special: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    pub hint: String,
    pub line_nr: String,
}

/// A named theme with a palette and optional style overrides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub palette: ThemePalette,
    pub overrides: HashMap<String, ThemeStyle>,
}

/// Style descriptor for a highlight group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// Registry of available themes.
#[derive(Debug, Clone)]
pub struct ThemeRegistry {
    pub themes: HashMap<String, Theme>,
    pub active: String,
}

impl ThemeRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            active: String::new(),
        }
    }

    /// Register a theme. If it is the first theme, it becomes active.
    pub fn register(&mut self, theme: Theme) {
        let name = theme.name.clone();
        self.themes.insert(name.clone(), theme);
        if self.active.is_empty() {
            self.active = name;
        }
    }

    /// Set the active theme by name. Returns `false` if the theme is not registered.
    pub fn set_active(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.active = name.to_string();
            true
        } else {
            false
        }
    }

    /// Get a reference to the currently active theme.
    pub fn active_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.active)
    }

    /// Get a theme by name.
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme_builtin::{theme_dark, theme_light};

    #[test]
    fn register_and_get() {
        let mut reg = ThemeRegistry::new();
        reg.register(theme_dark());
        reg.register(theme_light());
        assert_eq!(reg.active, "dark");
        assert!(reg.set_active("light"));
        assert_eq!(reg.active_theme().unwrap().name, "light");
    }

    #[test]
    fn set_active_missing() {
        let mut reg = ThemeRegistry::new();
        assert!(!reg.set_active("nonexistent"));
    }
}
