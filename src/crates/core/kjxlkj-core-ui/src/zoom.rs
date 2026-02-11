//! Window zoom state. See /docs/spec/features/window/window-zoom.md.
//!
//! Zoom temporarily maximizes one window. The original layout is saved
//! and restored when zoom is toggled off.

use crate::layout::{LayoutNode, LayoutTree};
use kjxlkj_core_types::WindowId;

/// Zoom state for the editor.
#[derive(Debug, Clone)]
pub struct ZoomState {
    /// Saved layout before zoom. None when not zoomed.
    saved_layout: Option<LayoutNode>,
    /// The window that was zoomed.
    zoomed_window: Option<WindowId>,
}

impl ZoomState {
    pub fn new() -> Self { Self { saved_layout: None, zoomed_window: None } }
    pub fn is_zoomed(&self) -> bool { self.saved_layout.is_some() }
    pub fn zoomed_window(&self) -> Option<WindowId> { self.zoomed_window }

    /// Toggle zoom for the given window. Returns true if zoom changed.
    pub fn toggle(&mut self, tree: &mut LayoutTree, focused: WindowId,
                  content: kjxlkj_core_types::ContentKind) -> bool {
        if self.is_zoomed() { self.restore(tree) } else { self.zoom_in(tree, focused, content) }
    }

    /// Zoom in: save current layout, replace with single-window layout.
    pub fn zoom_in(&mut self, tree: &mut LayoutTree, focused: WindowId,
                   content: kjxlkj_core_types::ContentKind) -> bool {
        if self.is_zoomed() { return false; }
        self.saved_layout = Some(tree.root.clone());
        self.zoomed_window = Some(focused);
        tree.root = LayoutNode::Leaf { window_id: focused, content };
        true
    }

    /// Zoom out: restore saved layout if available.
    pub fn restore(&mut self, tree: &mut LayoutTree) -> bool {
        if let Some(saved) = self.saved_layout.take() {
            // Remove windows that were closed while zoomed.
            tree.root = saved;
            self.zoomed_window = None;
            true
        } else {
            false
        }
    }

    /// Statusline indicator text.
    pub fn indicator(&self) -> &str {
        if self.is_zoomed() { "[Z]" } else { "" }
    }

    /// Called when a window is closed while zoomed.
    /// Removes that window from the saved layout so restore won't reference it.
    pub fn on_window_closed(&mut self, closed: WindowId) {
        if let Some(ref mut saved) = self.saved_layout {
            remove_window_from_node(saved, closed);
            collapse_unary(saved);
        }
    }
}

fn remove_window_from_node(node: &mut LayoutNode, target: WindowId) {
    match node {
        LayoutNode::Leaf { .. } => {}
        LayoutNode::Horizontal { children } | LayoutNode::Vertical { children } => {
            children.retain(|c| !matches!(c, LayoutNode::Leaf { window_id, .. } if *window_id == target));
            for child in children.iter_mut() { remove_window_from_node(child, target); }
        }
    }
}

fn collapse_unary(node: &mut LayoutNode) {
    match node {
        LayoutNode::Horizontal { children } | LayoutNode::Vertical { children } => {
            for child in children.iter_mut() { collapse_unary(child); }
            if children.len() == 1 { *node = children.remove(0); }
        }
        LayoutNode::Leaf { .. } => {}
    }
}

#[cfg(test)]
#[path = "zoom_tests.rs"]
mod tests;
