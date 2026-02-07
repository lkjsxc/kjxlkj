//! Line number display types and mode indicator formatting.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

/// Style used for rendering line numbers in the gutter.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum LineNumberStyle {
    None,
    #[default]
    Absolute,
    Relative,
    Hybrid,
}


/// Format a line number for display.
///
/// - `current_line`: the line the cursor is on (0-indexed).
/// - `display_line`: the line being rendered (0-indexed).
/// - `width`: minimum display width (right-aligned, space-padded).
pub fn format_line_number(
    style: LineNumberStyle,
    current_line: usize,
    display_line: usize,
    width: usize,
) -> String {
    match style {
        LineNumberStyle::None => " ".repeat(width),
        LineNumberStyle::Absolute => {
            format!("{:>w$}", display_line + 1, w = width)
        }
        LineNumberStyle::Relative => {
            let rel = display_line.abs_diff(current_line);
            format!("{:>w$}", rel, w = width)
        }
        LineNumberStyle::Hybrid => {
            if display_line == current_line {
                format!("{:>w$}", display_line + 1, w = width)
            } else {
                let rel = display_line.abs_diff(current_line);
                format!("{:>w$}", rel, w = width)
            }
        }
    }
}

/// Format style for the mode indicator in the status line.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModeIndicatorFormat {
    Uppercase,
    Short,
    Char,
    Hidden,
}

/// Format the mode indicator string for the status line.
pub fn format_mode_indicator(mode: &Mode, format: ModeIndicatorFormat) -> String {
    match format {
        ModeIndicatorFormat::Hidden => String::new(),
        ModeIndicatorFormat::Uppercase => format!("{mode}"),
        ModeIndicatorFormat::Short => match mode {
            Mode::Normal => "NOR".into(),
            Mode::Insert => "INS".into(),
            Mode::Visual => "VIS".into(),
            Mode::VisualLine => "V-L".into(),
            Mode::VisualBlock => "V-B".into(),
            Mode::Command => "CMD".into(),
            Mode::Replace => "REP".into(),
            Mode::Terminal => "TRM".into(),
            Mode::OperatorPending => "OPD".into(),
        },
        ModeIndicatorFormat::Char => match mode {
            Mode::Normal => "N".into(),
            Mode::Insert => "I".into(),
            Mode::Visual => "V".into(),
            Mode::VisualLine => "L".into(),
            Mode::VisualBlock => "B".into(),
            Mode::Command => "C".into(),
            Mode::Replace => "R".into(),
            Mode::Terminal => "T".into(),
            Mode::OperatorPending => "O".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_numbers() {
        assert_eq!(
            format_line_number(LineNumberStyle::Absolute, 0, 0, 4),
            "   1"
        );
        assert_eq!(
            format_line_number(LineNumberStyle::Absolute, 0, 9, 4),
            "  10"
        );
    }

    #[test]
    fn relative_numbers() {
        assert_eq!(
            format_line_number(LineNumberStyle::Relative, 5, 5, 3),
            "  0"
        );
        assert_eq!(
            format_line_number(LineNumberStyle::Relative, 5, 8, 3),
            "  3"
        );
        assert_eq!(
            format_line_number(LineNumberStyle::Relative, 5, 2, 3),
            "  3"
        );
    }

    #[test]
    fn hybrid_numbers() {
        // Current line shows absolute
        assert_eq!(format_line_number(LineNumberStyle::Hybrid, 5, 5, 4), "   6");
        // Other lines show relative
        assert_eq!(format_line_number(LineNumberStyle::Hybrid, 5, 7, 4), "   2");
    }

    #[test]
    fn none_numbers() {
        assert_eq!(format_line_number(LineNumberStyle::None, 0, 0, 4), "    ");
    }

    #[test]
    fn mode_indicators() {
        assert_eq!(
            format_mode_indicator(&Mode::Normal, ModeIndicatorFormat::Uppercase),
            "NORMAL"
        );
        assert_eq!(
            format_mode_indicator(&Mode::Insert, ModeIndicatorFormat::Short),
            "INS"
        );
        assert_eq!(
            format_mode_indicator(&Mode::Visual, ModeIndicatorFormat::Char),
            "V"
        );
        assert_eq!(
            format_mode_indicator(&Mode::Normal, ModeIndicatorFormat::Hidden),
            ""
        );
    }
}
