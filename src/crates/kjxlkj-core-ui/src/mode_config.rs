//! Mode configuration options — cursor shape, line numbers, status indicator.
//!
//! Mode configuration affects presentation only, not editing semantics.
//! Keybindings execute identically regardless of cursor/line-number style.

use kjxlkj_core_types::{CursorShape, Mode};

/// Per-mode cursor configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorConfig {
    pub shape: CursorShape,
    pub blink: bool,
}

impl CursorConfig {
    pub fn for_mode(mode: Mode) -> Self {
        match mode {
            Mode::Normal | Mode::Visual | Mode::VisualLine | Mode::VisualBlock
            | Mode::OperatorPending => {
                Self { shape: CursorShape::Block, blink: false }
            }
            Mode::Insert | Mode::InsertNormal | Mode::Command => {
                Self { shape: CursorShape::Bar, blink: true }
            }
            Mode::Replace => {
                Self { shape: CursorShape::Underline, blink: false }
            }
            Mode::Terminal => Self { shape: CursorShape::Bar, blink: true },
        }
    }
}

/// Line number display style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineNumberStyle {
    /// No line numbers.
    None,
    /// Absolute line numbers.
    Absolute,
    /// Relative line numbers (distance from cursor).
    Relative,
    /// Hybrid: current line absolute, others relative.
    Hybrid,
}

/// Status line mode indicator format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeIndicatorFormat {
    /// Uppercase mode name: "NORMAL", "INSERT", etc.
    Uppercase,
    /// Short abbreviation: "NOR", "INS", etc.
    Short,
    /// Single character: "N", "I", etc.
    Char,
    /// Hidden (no mode indicator).
    Hidden,
}

/// Complete mode display configuration.
#[derive(Debug, Clone)]
pub struct ModeDisplayConfig {
    pub cursor: CursorConfig,
    pub line_numbers: LineNumberStyle,
    pub mode_indicator: ModeIndicatorFormat,
}

impl ModeDisplayConfig {
    pub fn default_for(mode: Mode) -> Self {
        Self {
            cursor: CursorConfig::for_mode(mode),
            line_numbers: LineNumberStyle::Absolute,
            mode_indicator: ModeIndicatorFormat::Uppercase,
        }
    }
}

/// Format the mode name for a status line given the format preference.
pub fn mode_indicator_text(mode: Mode, format: ModeIndicatorFormat) -> &'static str {
    match format {
        ModeIndicatorFormat::Uppercase => match mode {
            Mode::Normal => "NORMAL", Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL", Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK", Mode::Command => "COMMAND",
            Mode::Replace => "REPLACE", Mode::OperatorPending => "O-PENDING",
            Mode::InsertNormal => "INSERT", Mode::Terminal => "TERMINAL",
        },
        ModeIndicatorFormat::Short => match mode {
            Mode::Normal => "NOR", Mode::Insert => "INS",
            Mode::Visual => "VIS", Mode::VisualLine => "V-L",
            Mode::VisualBlock => "V-B", Mode::Command => "CMD",
            Mode::Replace => "REP", Mode::OperatorPending => "OPR",
            Mode::InsertNormal => "INS", Mode::Terminal => "TRM",
        },
        ModeIndicatorFormat::Char => match mode {
            Mode::Normal => "N", Mode::Insert => "I",
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => "V",
            Mode::Command => "C", Mode::Replace => "R",
            Mode::InsertNormal => "I", Mode::Terminal => "T",
            Mode::OperatorPending => "O",
        },
        ModeIndicatorFormat::Hidden => "",
    }
}

/// Compute the relative line number for display.
pub fn relative_line_number(current_line: usize, display_line: usize) -> usize {
    if current_line >= display_line {
        current_line - display_line
    } else {
        display_line - current_line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_config_for_modes() {
        assert_eq!(CursorConfig::for_mode(Mode::Normal).shape, CursorShape::Block);
        assert_eq!(CursorConfig::for_mode(Mode::Insert).shape, CursorShape::Bar);
        assert_eq!(CursorConfig::for_mode(Mode::Replace).shape, CursorShape::Underline);
        assert!(CursorConfig::for_mode(Mode::Insert).blink);
        assert!(!CursorConfig::for_mode(Mode::Normal).blink);
    }

    #[test]
    fn mode_indicator_formats() {
        assert_eq!(mode_indicator_text(Mode::Normal, ModeIndicatorFormat::Uppercase), "NORMAL");
        assert_eq!(mode_indicator_text(Mode::Insert, ModeIndicatorFormat::Short), "INS");
        assert_eq!(mode_indicator_text(Mode::Visual, ModeIndicatorFormat::Char), "V");
        assert_eq!(mode_indicator_text(Mode::Command, ModeIndicatorFormat::Hidden), "");
    }

    #[test]
    fn relative_line_numbers() {
        assert_eq!(relative_line_number(10, 10), 0); // cursor line
        assert_eq!(relative_line_number(10, 7), 3);
        assert_eq!(relative_line_number(10, 13), 3);
        assert_eq!(relative_line_number(0, 5), 5);
    }

    #[test]
    fn line_number_styles_distinct() {
        assert_ne!(LineNumberStyle::None, LineNumberStyle::Absolute);
        assert_ne!(LineNumberStyle::Relative, LineNumberStyle::Hybrid);
    }

    #[test]
    fn default_display_config() {
        let cfg = ModeDisplayConfig::default_for(Mode::Insert);
        assert_eq!(cfg.cursor.shape, CursorShape::Bar);
        assert_eq!(cfg.line_numbers, LineNumberStyle::Absolute);
        assert_eq!(cfg.mode_indicator, ModeIndicatorFormat::Uppercase);
    }
}
