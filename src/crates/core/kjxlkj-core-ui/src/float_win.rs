//! Floating window model.
//!
//! See /docs/spec/features/window/floating-windows.md.

use kjxlkj_core_types::{BufferId, WindowId};

/// Anchor point for float positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatAnchor {
    Editor, Cursor, Window, NW, NE, SW, SE,
}

/// Border style for floating windows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorderStyle {
    None, Single, Double, Rounded, Solid, Shadow,
    Custom([String; 8]),
}

impl Default for BorderStyle {
    fn default() -> Self { Self::Rounded }
}

/// Title/footer alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align { Left, Center, Right }

/// Type classification for floating windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatKind { Dialog, Tooltip, Preview, Completion }

/// Configuration for creating a floating window.
#[derive(Debug, Clone)]
pub struct FloatConfig {
    pub width: u16,
    pub height: u16,
    pub row: u16,
    pub col: u16,
    pub anchor: FloatAnchor,
    pub center: bool,
    pub border: BorderStyle,
    pub focusable: bool,
    pub enter: bool,
    pub zindex: u32,
    pub title: Option<String>,
    pub title_align: Align,
    pub footer: Option<String>,
    pub footer_align: Align,
    pub kind: FloatKind,
    pub close_on_focus_loss: bool,
}

impl FloatConfig {
    /// Centered dialog config.
    pub fn dialog(w: u16, h: u16) -> Self {
        Self {
            width: w, height: h, row: 0, col: 0,
            anchor: FloatAnchor::Editor, center: true,
            border: BorderStyle::Rounded,
            focusable: true, enter: true, zindex: 50,
            title: None, title_align: Align::Left,
            footer: None, footer_align: Align::Left,
            kind: FloatKind::Dialog,
            close_on_focus_loss: false,
        }
    }

    /// Tooltip config near cursor.
    pub fn tooltip(w: u16, h: u16) -> Self {
        Self {
            width: w, height: h, row: 1, col: 0,
            anchor: FloatAnchor::Cursor, center: false,
            border: BorderStyle::Single,
            focusable: false, enter: false, zindex: 60,
            title: None, title_align: Align::Left,
            footer: None, footer_align: Align::Left,
            kind: FloatKind::Tooltip,
            close_on_focus_loss: true,
        }
    }
}

/// A live floating window instance.
#[derive(Debug, Clone)]
pub struct FloatWindow {
    pub window_id: WindowId,
    pub buffer_id: BufferId,
    pub config: FloatConfig,
    pub creation_order: u64,
}

/// Manages all floating windows.
#[derive(Debug, Clone)]
pub struct FloatLayer {
    pub floats: Vec<FloatWindow>,
    next_order: u64,
}

impl FloatLayer {
    pub fn new() -> Self {
        Self { floats: Vec::new(), next_order: 0 }
    }

    /// Open a new floating window.
    pub fn open(
        &mut self, window_id: WindowId,
        buffer_id: BufferId, config: FloatConfig,
    ) -> &FloatWindow {
        let fw = FloatWindow {
            window_id, buffer_id, config,
            creation_order: self.next_order,
        };
        self.next_order += 1;
        self.floats.push(fw);
        self.floats.last().unwrap()
    }

    /// Close a floating window by id.
    pub fn close(&mut self, wid: WindowId) -> bool {
        let before = self.floats.len();
        self.floats.retain(|f| f.window_id != wid);
        self.floats.len() < before
    }

    /// Sorted render order (ascending zindex, then creation).
    pub fn render_order(&self) -> Vec<&FloatWindow> {
        let mut sorted: Vec<_> = self.floats.iter().collect();
        sorted.sort_by(|a, b| {
            a.config.zindex.cmp(&b.config.zindex)
                .then(a.creation_order.cmp(&b.creation_order))
        });
        sorted
    }

    /// Find a focusable float by id.
    pub fn focusable(&self, wid: WindowId) -> Option<&FloatWindow> {
        self.floats.iter().find(|f| f.window_id == wid && f.config.focusable)
    }

    pub fn count(&self) -> usize { self.floats.len() }
    pub fn is_empty(&self) -> bool { self.floats.is_empty() }
}

#[cfg(test)]
#[path = "float_win_tests.rs"]
mod tests;
