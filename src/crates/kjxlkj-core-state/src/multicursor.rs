//! Multi-cursor support per /docs/spec/features/editing/multicursor.md.
//!
//! Provides multiple simultaneous cursors for editing.

use kjxlkj_core_edit::CursorPosition;

/// A secondary cursor with optional selection.
#[derive(Debug, Clone)]
pub struct SecondaryCursor {
    /// Cursor position.
    pub position: CursorPosition,
    /// Optional anchor for selection.
    pub anchor: Option<CursorPosition>,
}

/// Multi-cursor state.
#[derive(Debug, Clone, Default)]
pub struct MultiCursorState {
    /// Whether multi-cursor mode is active.
    pub active: bool,
    /// Secondary cursors (primary is EditorState.cursor).
    pub cursors: Vec<SecondaryCursor>,
    /// Pattern for "select all" matching.
    pub match_pattern: Option<String>,
}

impl MultiCursorState {
    /// Create new empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a cursor at position.
    pub fn add_cursor(&mut self, pos: CursorPosition) {
        self.active = true;
        self.cursors.push(SecondaryCursor {
            position: pos,
            anchor: None,
        });
    }

    /// Remove cursor nearest to position.
    pub fn remove_nearest(&mut self, pos: CursorPosition) {
        if self.cursors.is_empty() {
            return;
        }
        let mut best = 0usize;
        let mut best_dist = usize::MAX;
        for (i, c) in self.cursors.iter().enumerate() {
            let dist = pos.line.abs_diff(c.position.line)
                + pos.grapheme_offset.abs_diff(c.position.grapheme_offset);
            if dist < best_dist {
                best_dist = dist;
                best = i;
            }
        }
        self.cursors.remove(best);
        if self.cursors.is_empty() {
            self.active = false;
        }
    }

    /// Clear all secondary cursors.
    pub fn clear(&mut self) {
        self.cursors.clear();
        self.active = false;
        self.match_pattern = None;
    }

    /// Number of active cursors (including primary).
    pub fn cursor_count(&self) -> usize {
        if self.active {
            self.cursors.len() + 1
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_edit::CursorPosition;

    #[test]
    fn add_and_count() {
        let mut mc = MultiCursorState::new();
        assert_eq!(mc.cursor_count(), 1);
        mc.add_cursor(CursorPosition {
            line: 1,
            grapheme_offset: 0,
            desired_col: None,
        });
        assert_eq!(mc.cursor_count(), 2);
        assert!(mc.active);
    }

    #[test]
    fn remove_nearest() {
        let mut mc = MultiCursorState::new();
        mc.add_cursor(CursorPosition {
            line: 5,
            grapheme_offset: 3,
            desired_col: None,
        });
        mc.add_cursor(CursorPosition {
            line: 10,
            grapheme_offset: 0,
            desired_col: None,
        });
        mc.remove_nearest(CursorPosition {
            line: 4,
            grapheme_offset: 2,
            desired_col: None,
        });
        assert_eq!(mc.cursors.len(), 1);
        assert_eq!(mc.cursors[0].position.line, 10);
    }

    #[test]
    fn clear_cursors() {
        let mut mc = MultiCursorState::new();
        mc.add_cursor(CursorPosition {
            line: 0,
            grapheme_offset: 0,
            desired_col: None,
        });
        mc.clear();
        assert!(!mc.active);
        assert_eq!(mc.cursor_count(), 1);
    }
}
