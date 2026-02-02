//! Gutter rendering model.
//!
//! Combines sign column and diff highlighting into a single renderable cell.

use crate::{DiffKind, DiffState, SignColumn, SignDefinition};

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
    fn pad_to_width(mut text: String, width: usize) -> String {
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

    fn blank(width: usize) -> Self {
        Self {
            text: Self::pad_to_width(String::new(), width),
            text_highlight: "SignColumn".to_string(),
        }
    }
}

fn diff_symbol(kind: DiffKind) -> (&'static str, &'static str) {
    match kind {
        DiffKind::Added => ("+", "DiffAdd"),
        DiffKind::Changed => ("~", "DiffChange"),
        DiffKind::Deleted => ("_", "DiffDelete"),
        DiffKind::DeletedTop => ("^", "DiffDelete"),
    }
}

/// A combined gutter state for a single buffer.
#[derive(Debug, Default)]
pub struct GutterState {
    config: GutterConfig,
    signs: SignColumn,
    diff: DiffState,
}

impl GutterState {
    /// Creates a new gutter state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates with config.
    pub fn with_config(config: GutterConfig) -> Self {
        Self {
            config,
            signs: SignColumn::new(),
            diff: DiffState::new(),
        }
    }

    /// Gets config.
    pub fn config(&self) -> &GutterConfig {
        &self.config
    }

    /// Gets sign column state.
    pub fn signs(&self) -> &SignColumn {
        &self.signs
    }

    /// Gets mutable sign column state.
    pub fn signs_mut(&mut self) -> &mut SignColumn {
        &mut self.signs
    }

    /// Gets diff state.
    pub fn diff(&self) -> &DiffState {
        &self.diff
    }

    /// Gets mutable diff state.
    pub fn diff_mut(&mut self) -> &mut DiffState {
        &mut self.diff
    }

    /// Renders the gutter cell for a 1-based line number.
    pub fn cell_for_line(&self, line_1based: usize) -> GutterCell {
        let width = self.config.width;
        if width == 0 {
            return GutterCell::blank(0);
        }

        if self.config.show_signs {
            if let Some(sign) = self.signs.top_sign_at(line_1based) {
                if let Some(def) = self.signs.definition(&sign.name) {
                    return render_sign(def, width);
                }
            }
        }

        if self.config.show_diff {
            if let Some(marker) = self.diff.marker(line_1based.saturating_sub(1)) {
                let (sym, hl) = diff_symbol(marker.kind);
                return GutterCell {
                    text: GutterCell::pad_to_width(sym.to_string(), width),
                    text_highlight: hl.to_string(),
                };
            }
        }

        GutterCell::blank(width)
    }
}

fn render_sign(def: &SignDefinition, width: usize) -> GutterCell {
    GutterCell {
        text: GutterCell::pad_to_width(def.text.clone(), width),
        text_highlight: def.text_highlight.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiffMarker;

    #[test]
    fn test_gutter_cell_padding() {
        assert_eq!(GutterCell::pad_to_width("".to_string(), 2), "  ");
        assert_eq!(GutterCell::pad_to_width("X".to_string(), 2), "X ");
        assert_eq!(GutterCell::pad_to_width("XYZ".to_string(), 2), "XY");
    }

    #[test]
    fn test_gutter_default_blank() {
        let gutter = GutterState::new();
        let cell = gutter.cell_for_line(1);
        assert_eq!(cell.text, "  ");
        assert_eq!(cell.text_highlight, "SignColumn");
    }

    #[test]
    fn test_gutter_renders_sign_over_diff() {
        let mut gutter = GutterState::new();
        gutter
            .signs_mut()
            .define(SignDefinition::new("Error", "E").with_text_highlight("ErrorSign"));
        let id = gutter.signs_mut().place("Error", 10).unwrap();
        assert!(gutter.signs_mut().set_priority(id, 50));

        gutter
            .diff_mut()
            .set_markers(vec![DiffMarker::new(9, DiffKind::Added)]);

        let cell = gutter.cell_for_line(10);
        assert_eq!(cell.text, "E ");
        assert_eq!(cell.text_highlight, "ErrorSign");
    }

    #[test]
    fn test_gutter_renders_diff_when_no_sign() {
        let mut gutter = GutterState::new();
        gutter
            .diff_mut()
            .set_markers(vec![DiffMarker::new(0, DiffKind::Changed)]);

        let cell = gutter.cell_for_line(1);
        assert_eq!(cell.text, "~ ");
        assert_eq!(cell.text_highlight, "DiffChange");
    }

    #[test]
    fn test_gutter_respects_config_flags() {
        let mut gutter = GutterState::with_config(GutterConfig {
            show_signs: false,
            show_diff: false,
            width: 2,
        });
        gutter
            .signs_mut()
            .define(SignDefinition::new("Info", "i").with_text_highlight("InfoSign"));
        gutter.signs_mut().place("Info", 1);
        gutter
            .diff_mut()
            .set_markers(vec![DiffMarker::new(0, DiffKind::Added)]);

        let cell = gutter.cell_for_line(1);
        assert_eq!(cell.text, "  ");
        assert_eq!(cell.text_highlight, "SignColumn");
    }

    #[test]
    fn test_gutter_zero_width() {
        let gutter = GutterState::with_config(GutterConfig {
            show_signs: true,
            show_diff: true,
            width: 0,
        });
        let cell = gutter.cell_for_line(1);
        assert_eq!(cell.text, "");
    }
}
