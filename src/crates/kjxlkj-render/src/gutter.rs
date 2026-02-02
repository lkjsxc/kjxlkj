//! Gutter rendering for line numbers and signs.

use crate::buffer::{Cell, ScreenBuffer};
use crossterm::style::Color;

/// Gutter component for line numbers and signs.
#[derive(Debug, Clone)]
pub struct GutterConfig {
    /// Whether to show line numbers.
    pub show_numbers: bool,
    /// Whether to use relative line numbers.
    pub relative: bool,
    /// Minimum width for numbers column.
    pub min_width: u16,
    /// Show signs column.
    pub show_signs: bool,
    /// Show fold column.
    pub show_folds: bool,
    /// Foreground color for numbers.
    pub number_fg: Color,
    /// Foreground for current line number.
    pub cursorline_fg: Color,
    /// Background color.
    pub bg: Color,
}

impl Default for GutterConfig {
    fn default() -> Self {
        Self {
            show_numbers: true,
            relative: false,
            min_width: 4,
            show_signs: true,
            show_folds: false,
            number_fg: Color::DarkGrey,
            cursorline_fg: Color::Yellow,
            bg: Color::Reset,
        }
    }
}

/// Calculates the width of the gutter.
pub fn gutter_width(config: &GutterConfig, total_lines: usize) -> u16 {
    let mut width = 0;

    if config.show_signs {
        width += 2; // Sign column
    }

    if config.show_numbers {
        let num_width = total_lines.to_string().len().max(config.min_width as usize);
        width += num_width + 1; // Numbers + padding
    }

    if config.show_folds {
        width += 1; // Fold column
    }

    width as u16
}

/// Renders the gutter for a single line.
pub fn render_gutter_line(
    buffer: &mut ScreenBuffer,
    y: u16,
    line_num: usize,
    cursor_line: usize,
    total_lines: usize,
    config: &GutterConfig,
) {
    let mut x = 0u16;

    // Sign column
    if config.show_signs {
        buffer.set(x, y, Cell::new(' ').bg(config.bg));
        buffer.set(x + 1, y, Cell::new(' ').bg(config.bg));
        x += 2;
    }

    // Line number
    if config.show_numbers {
        let num_width = total_lines.to_string().len().max(config.min_width as usize);
        let display_num = if config.relative && line_num != cursor_line {
            (line_num as isize - cursor_line as isize).unsigned_abs()
        } else {
            line_num + 1
        };

        let fg = if line_num == cursor_line {
            config.cursorline_fg
        } else {
            config.number_fg
        };

        let num_str = format!("{:>width$} ", display_num, width = num_width);
        buffer.write_str(x, y, &num_str, fg, config.bg);
    }
}
