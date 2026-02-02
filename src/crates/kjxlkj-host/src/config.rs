//! Configuration management for kjxlkj.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Editor options.
    pub editor: EditorConfig,
    /// UI configuration.
    pub ui: UiConfig,
    /// File handling options.
    pub files: FilesConfig,
}

/// Editor options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EditorConfig {
    /// Tab width in spaces.
    pub tab_width: usize,
    /// Use spaces instead of tabs.
    pub expand_tabs: bool,
    /// Show line numbers.
    pub line_numbers: bool,
    /// Relative line numbers.
    pub relative_numbers: bool,
    /// Scroll offset (lines before cursor).
    pub scroll_off: usize,
    /// Highlight current line.
    pub cursor_line: bool,
    /// Wrap long lines.
    pub wrap: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_width: 4,
            expand_tabs: true,
            line_numbers: true,
            relative_numbers: false,
            scroll_off: 5,
            cursor_line: true,
            wrap: false,
        }
    }
}

/// UI configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    /// Color theme name.
    pub theme: String,
    /// Show statusline.
    pub statusline: bool,
    /// Show tabline.
    pub tabline: bool,
    /// Show sign column.
    pub signcolumn: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            statusline: true,
            tabline: true,
            signcolumn: true,
        }
    }
}

/// Files configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FilesConfig {
    /// Auto-save on focus lost.
    pub auto_save: bool,
    /// Backup before save.
    pub backup: bool,
    /// Encoding.
    pub encoding: String,
    /// Line endings (lf, crlf).
    pub line_ending: String,
}

impl Default for FilesConfig {
    fn default() -> Self {
        Self {
            auto_save: false,
            backup: false,
            encoding: "utf-8".to_string(),
            line_ending: "lf".to_string(),
        }
    }
}

impl Config {
    /// Loads config from the default path.
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(contents) = std::fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str(&contents) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    /// Returns the config file path.
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("kjxlkj").join("config.toml"))
    }

    /// Saves config to the default path.
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
            std::fs::write(path, content)?;
        }
        Ok(())
    }
}
