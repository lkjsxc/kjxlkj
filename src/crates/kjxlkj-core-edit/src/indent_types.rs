//! Indentation types.
//!
//! Types for indent style and configuration.

use serde::{Deserialize, Serialize};

/// Indentation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndentStyle {
    /// Use tabs for indentation.
    Tabs,
    /// Use spaces for indentation.
    Spaces(u8),
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self::Spaces(4)
    }
}

impl IndentStyle {
    /// Returns the string for one level of indentation.
    pub fn indent_str(&self) -> String {
        match self {
            Self::Tabs => "\t".to_string(),
            Self::Spaces(n) => " ".repeat(*n as usize),
        }
    }

    /// Returns the display width of one indent level.
    pub fn width(&self) -> usize {
        match self {
            Self::Tabs => 8, // Default tab width
            Self::Spaces(n) => *n as usize,
        }
    }
}

/// Auto-indent configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndentConfig {
    /// Indent style.
    pub style: IndentStyle,
    /// Enable smart indent.
    pub smart_indent: bool,
    /// Increase indent chars (e.g., '{').
    pub increase_indent_on: Vec<char>,
    /// Decrease indent chars (e.g., '}').
    pub decrease_indent_on: Vec<char>,
}

impl Default for IndentConfig {
    fn default() -> Self {
        Self {
            style: IndentStyle::default(),
            smart_indent: true,
            increase_indent_on: vec!['{', '(', '['],
            decrease_indent_on: vec!['}', ')', ']'],
        }
    }
}

impl IndentConfig {
    /// Creates a new default indent config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a config with tabs.
    pub fn with_tabs() -> Self {
        Self {
            style: IndentStyle::Tabs,
            ..Self::default()
        }
    }

    /// Creates a config with spaces.
    pub fn with_spaces(n: u8) -> Self {
        Self {
            style: IndentStyle::Spaces(n),
            ..Self::default()
        }
    }
}
