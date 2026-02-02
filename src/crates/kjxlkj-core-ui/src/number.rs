//! Line number column.
//!
//! Line number display options.

/// Number display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NumberMode {
    /// No line numbers.
    #[default]
    None,
    /// Absolute line numbers.
    Absolute,
    /// Relative line numbers.
    Relative,
    /// Hybrid (current = absolute, others = relative).
    Hybrid,
}

/// Number column configuration.
#[derive(Debug, Clone)]
pub struct NumberColumn {
    /// Display mode.
    pub mode: NumberMode,
    /// Minimum width.
    pub min_width: usize,
    /// Whether to right-align numbers.
    pub right_align: bool,
}

impl Default for NumberColumn {
    fn default() -> Self {
        Self {
            mode: NumberMode::None,
            min_width: 4,
            right_align: true,
        }
    }
}

impl NumberColumn {
    /// Creates a new number column.
    pub fn new(mode: NumberMode) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }

    /// Calculates required width for line count.
    pub fn width_for_lines(&self, line_count: usize) -> usize {
        if self.mode == NumberMode::None {
            return 0;
        }
        let digits = if line_count == 0 {
            1
        } else {
            (line_count as f64).log10().floor() as usize + 1
        };
        // Add 1 for padding
        digits.max(self.min_width) + 1
    }

    /// Formats a line number.
    pub fn format(&self, line: usize, current: usize, width: usize) -> String {
        let num = match self.mode {
            NumberMode::None => return String::new(),
            NumberMode::Absolute => line,
            NumberMode::Relative => {
                if line == current {
                    line
                } else {
                    line.abs_diff(current)
                }
            }
            NumberMode::Hybrid => {
                if line == current {
                    line
                } else {
                    line.abs_diff(current)
                }
            }
        };

        if self.right_align {
            format!("{:>width$} ", num, width = width - 1)
        } else {
            format!("{:<width$} ", num, width = width - 1)
        }
    }

    /// Returns whether numbers are enabled.
    pub fn is_enabled(&self) -> bool {
        self.mode != NumberMode::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_mode_default() {
        assert_eq!(NumberMode::default(), NumberMode::None);
    }

    #[test]
    fn test_number_column_width() {
        let col = NumberColumn::new(NumberMode::Absolute);
        assert_eq!(col.width_for_lines(100), 5);
        assert_eq!(col.width_for_lines(1000), 5);
        assert_eq!(col.width_for_lines(10000), 6);
    }

    #[test]
    fn test_number_column_format_absolute() {
        let col = NumberColumn::new(NumberMode::Absolute);
        let s = col.format(42, 10, 5);
        assert!(s.contains("42"));
    }

    #[test]
    fn test_number_column_format_relative() {
        let col = NumberColumn::new(NumberMode::Relative);
        let s = col.format(15, 10, 5);
        assert!(s.contains("5"));
    }

    #[test]
    fn test_number_column_none() {
        let col = NumberColumn::new(NumberMode::None);
        assert_eq!(col.width_for_lines(100), 0);
    }

    #[test]
    fn test_number_column_is_enabled() {
        assert!(!NumberColumn::new(NumberMode::None).is_enabled());
        assert!(NumberColumn::new(NumberMode::Absolute).is_enabled());
    }
}
