//! Gutter types.
//!
//! Types for gutter configuration and rendering.

use crate::DiffKind;

/// Configuration for the gutter.
#[derive(Debug, Clone)]
pub struct GutterConfig {
    /// Whether to show placed signs.
    pub show_signs: bool,
    /// Whether to show diff markers.
    pub show_diff: bool,
    /// Render width (characters).
    pub width: usize,
}

impl Default for GutterConfig {
    fn default() -> Self {
        Self {
            show_signs: true,
            show_diff: true,
            width: 2,
        }
    }
}

/// A renderable gutter cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GutterCell {
    /// Display text (already padded/truncated to config width).
    pub text: String,
    /// Highlight group name for the text.
    pub text_highlight: String,
}

impl GutterCell {
    /// Pads text to width.
    pub fn pad_to_width(mut text: String, width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        if text.chars().count() > width {
            text = text.chars().take(width).collect();
        }
        while text.chars().count() < width {
            text.push(' ');
        }
        text
    }

    /// Creates a blank cell.
    pub fn blank(width: usize) -> Self {
        Self {
            text: Self::pad_to_width(String::new(), width),
            text_highlight: "SignColumn".to_string(),
        }
    }
}

/// Returns symbol and highlight for diff kind.
pub fn diff_symbol(kind: DiffKind) -> (&'static str, &'static str) {
    match kind {
        DiffKind::Added => ("+", "DiffAdd"),
        DiffKind::Changed => ("~", "DiffChange"),
        DiffKind::Deleted => ("_", "DiffDelete"),
        DiffKind::DeletedTop => ("^", "DiffDelete"),
    }
}
