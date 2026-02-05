//! Mode configuration options.
//!
//! Implements configurable elements for mode presentation.
//! Note: Configuration affects presentation only, not editing semantics.

use kjxlkj_core_types::Mode;
use std::collections::HashMap;

/// Cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorShape {
    /// Block cursor.
    #[default]
    Block,
    /// Vertical bar cursor.
    Bar,
    /// Underline cursor.
    Underline,
}

/// Line number display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineNumberMode {
    /// No line numbers.
    None,
    /// Absolute line numbers.
    #[default]
    Absolute,
    /// Relative line numbers.
    Relative,
    /// Hybrid (current absolute, others relative).
    Hybrid,
}

/// Mode indicator format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModeIndicator {
    /// Uppercase mode name (NORMAL, INSERT, etc.).
    #[default]
    Uppercase,
    /// Lowercase mode name.
    Lowercase,
    /// Short form (N, I, V, etc.).
    Short,
    /// Custom format.
    Custom,
}

/// Per-mode cursor configuration.
#[derive(Debug, Clone, Copy)]
pub struct CursorConfig {
    /// Cursor shape.
    pub shape: CursorShape,
    /// Whether cursor blinks.
    pub blink: bool,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            shape: CursorShape::Block,
            blink: false,
        }
    }
}

impl CursorConfig {
    /// Default for Normal mode.
    pub fn normal() -> Self {
        Self {
            shape: CursorShape::Block,
            blink: false,
        }
    }

    /// Default for Insert mode.
    pub fn insert() -> Self {
        Self {
            shape: CursorShape::Bar,
            blink: false,
        }
    }

    /// Default for Replace mode.
    pub fn replace() -> Self {
        Self {
            shape: CursorShape::Underline,
            blink: false,
        }
    }

    /// Default for Visual mode.
    pub fn visual() -> Self {
        Self {
            shape: CursorShape::Block,
            blink: false,
        }
    }

    /// Default for Command mode.
    pub fn command() -> Self {
        Self {
            shape: CursorShape::Bar,
            blink: false,
        }
    }
}

/// Mode configuration.
#[derive(Debug, Clone)]
pub struct ModeConfig {
    /// Cursor configs per mode.
    cursors: HashMap<Mode, CursorConfig>,
    /// Line number mode.
    pub line_numbers: LineNumberMode,
    /// Mode indicator format.
    pub mode_indicator: ModeIndicator,
    /// Show mode in statusline.
    pub showmode: bool,
}

impl Default for ModeConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeConfig {
    /// Create new mode configuration with defaults.
    pub fn new() -> Self {
        let mut cursors = HashMap::new();
        cursors.insert(Mode::Normal, CursorConfig::normal());
        cursors.insert(Mode::Insert, CursorConfig::insert());
        cursors.insert(Mode::Replace, CursorConfig::replace());
        cursors.insert(Mode::Visual, CursorConfig::visual());
        cursors.insert(Mode::VisualLine, CursorConfig::visual());
        cursors.insert(Mode::VisualBlock, CursorConfig::visual());
        cursors.insert(Mode::Command, CursorConfig::command());

        Self {
            cursors,
            line_numbers: LineNumberMode::Absolute,
            mode_indicator: ModeIndicator::Uppercase,
            showmode: true,
        }
    }

    /// Get cursor config for a mode.
    pub fn cursor(&self, mode: Mode) -> CursorConfig {
        self.cursors.get(&mode).copied().unwrap_or_default()
    }

    /// Set cursor config for a mode.
    pub fn set_cursor(&mut self, mode: Mode, config: CursorConfig) {
        self.cursors.insert(mode, config);
    }

    /// Get mode indicator text.
    pub fn mode_text(&self, mode: Mode) -> String {
        match self.mode_indicator {
            ModeIndicator::Uppercase => mode_to_string(mode).to_uppercase(),
            ModeIndicator::Lowercase => mode_to_string(mode).to_lowercase(),
            ModeIndicator::Short => mode_to_short(mode).to_string(),
            ModeIndicator::Custom => mode_to_string(mode).to_uppercase(),
        }
    }

    /// Format line number based on configuration.
    pub fn format_line_number(&self, line: usize, current: usize) -> String {
        match self.line_numbers {
            LineNumberMode::None => String::new(),
            LineNumberMode::Absolute => format!("{}", line),
            LineNumberMode::Relative => {
                if line == current {
                    format!("{}", line)
                } else {
                    format!("{}", (line as isize - current as isize).unsigned_abs())
                }
            }
            LineNumberMode::Hybrid => {
                if line == current {
                    format!("{}", line)
                } else {
                    format!("{}", (line as isize - current as isize).unsigned_abs())
                }
            }
        }
    }
}

fn mode_to_string(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "Normal",
        Mode::Insert => "Insert",
        Mode::Visual => "Visual",
        Mode::VisualLine => "V-Line",
        Mode::VisualBlock => "V-Block",
        Mode::Replace => "Replace",
        Mode::Command => "Command",
    }
}

fn mode_to_short(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "N",
        Mode::Insert => "I",
        Mode::Visual => "V",
        Mode::VisualLine => "VL",
        Mode::VisualBlock => "VB",
        Mode::Replace => "R",
        Mode::Command => "C",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_shape_default() {
        assert_eq!(CursorShape::default(), CursorShape::Block);
    }

    #[test]
    fn test_cursor_config_modes() {
        assert_eq!(CursorConfig::normal().shape, CursorShape::Block);
        assert_eq!(CursorConfig::insert().shape, CursorShape::Bar);
        assert_eq!(CursorConfig::replace().shape, CursorShape::Underline);
    }

    #[test]
    fn test_mode_config_cursor() {
        let config = ModeConfig::new();
        assert_eq!(config.cursor(Mode::Normal).shape, CursorShape::Block);
        assert_eq!(config.cursor(Mode::Insert).shape, CursorShape::Bar);
    }

    #[test]
    fn test_mode_config_set_cursor() {
        let mut config = ModeConfig::new();
        config.set_cursor(Mode::Normal, CursorConfig::insert());
        assert_eq!(config.cursor(Mode::Normal).shape, CursorShape::Bar);
    }

    #[test]
    fn test_mode_text_uppercase() {
        let config = ModeConfig::new();
        assert_eq!(config.mode_text(Mode::Normal), "NORMAL");
        assert_eq!(config.mode_text(Mode::Insert), "INSERT");
    }

    #[test]
    fn test_mode_text_short() {
        let mut config = ModeConfig::new();
        config.mode_indicator = ModeIndicator::Short;
        assert_eq!(config.mode_text(Mode::Normal), "N");
        assert_eq!(config.mode_text(Mode::Visual), "V");
    }

    #[test]
    fn test_line_number_absolute() {
        let config = ModeConfig::new();
        assert_eq!(config.format_line_number(5, 3), "5");
        assert_eq!(config.format_line_number(3, 3), "3");
    }

    #[test]
    fn test_line_number_relative() {
        let mut config = ModeConfig::new();
        config.line_numbers = LineNumberMode::Relative;
        assert_eq!(config.format_line_number(5, 3), "2");
        assert_eq!(config.format_line_number(1, 3), "2");
        assert_eq!(config.format_line_number(3, 3), "3");
    }

    #[test]
    fn test_line_number_hybrid() {
        let mut config = ModeConfig::new();
        config.line_numbers = LineNumberMode::Hybrid;
        assert_eq!(config.format_line_number(5, 3), "2");
        assert_eq!(config.format_line_number(3, 3), "3");
    }

    #[test]
    fn test_showmode_default() {
        let config = ModeConfig::new();
        assert!(config.showmode);
    }
}
