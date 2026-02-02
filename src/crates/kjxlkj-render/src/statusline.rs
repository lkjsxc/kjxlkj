//! Statusline rendering component.

use crossterm::style::Color;
use crate::buffer::ScreenBuffer;

/// Statusline configuration.
#[derive(Debug, Clone)]
pub struct StatuslineConfig {
    /// Format string for left section.
    pub left: String,
    /// Format string for center section.
    pub center: String,
    /// Format string for right section.
    pub right: String,
    /// Background color.
    pub bg: Color,
    /// Foreground color.
    pub fg: Color,
}

impl Default for StatuslineConfig {
    fn default() -> Self {
        Self {
            left: "{mode}".to_string(),
            center: "{filename}".to_string(),
            right: "{line}:{col}".to_string(),
            bg: Color::DarkGrey,
            fg: Color::White,
        }
    }
}

/// Context for rendering the statusline.
pub struct StatuslineContext<'a> {
    /// Current mode name.
    pub mode: &'a str,
    /// File name.
    pub filename: &'a str,
    /// Whether file is modified.
    pub modified: bool,
    /// Current line (1-based).
    pub line: usize,
    /// Current column (1-based).
    pub col: usize,
    /// Total lines.
    pub total_lines: usize,
    /// File type.
    pub filetype: &'a str,
}

/// Renders the statusline to a buffer.
pub fn render_statusline(
    buffer: &mut ScreenBuffer,
    y: u16,
    ctx: &StatuslineContext,
    config: &StatuslineConfig,
) {
    let width = buffer.width() as usize;
    
    // Build left section
    let modified_indicator = if ctx.modified { " [+]" } else { "" };
    let left = format!(" {} | {}{} ", ctx.mode, ctx.filename, modified_indicator);
    
    // Build right section
    let right = format!(" {}:{} | {} ", ctx.line, ctx.col, ctx.total_lines);
    
    // Build center/fill
    let fill_len = width.saturating_sub(left.len() + right.len());
    let fill = " ".repeat(fill_len);
    
    // Combine
    let line = format!("{}{}{}", left, fill, right);
    
    // Write to buffer
    buffer.write_str(0, y, &line, config.fg, config.bg);
}

/// Renders a minimal statusline.
pub fn render_minimal_statusline(
    buffer: &mut ScreenBuffer,
    y: u16,
    mode: &str,
    filename: &str,
    line: usize,
    col: usize,
) {
    let config = StatuslineConfig::default();
    let ctx = StatuslineContext {
        mode,
        filename,
        modified: false,
        line,
        col,
        total_lines: 0,
        filetype: "",
    };
    render_statusline(buffer, y, &ctx, &config);
}
