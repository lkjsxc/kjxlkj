//! View management: save/restore per-window view state
//! (cursor, scroll position, folds, local options).

use std::collections::HashMap;
use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_types::BufferId;

/// Saved view state for a buffer.
#[derive(Debug, Clone)]
pub struct ViewState {
    /// Buffer this view was for.
    pub buffer_id: BufferId,
    /// Cursor position.
    pub cursor: CursorPosition,
    /// Viewport top line.
    pub top_line: usize,
    /// Viewport left column.
    pub left_col: usize,
    /// Folded line ranges.
    pub folds: Vec<(usize, usize)>,
    /// Local option overrides (key → value).
    pub local_options: HashMap<String, String>,
}

/// Registry of saved views, keyed by buffer ID.
#[derive(Debug, Clone, Default)]
pub struct ViewRegistry {
    /// Saved views per buffer.
    views: HashMap<BufferId, ViewState>,
    /// Maximum number of saved views.
    pub max_views: usize,
}

impl ViewRegistry {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            max_views: 256,
        }
    }

    /// Save a view state for a buffer.
    pub fn save(
        &mut self,
        buffer_id: BufferId,
        cursor: CursorPosition,
        top_line: usize,
        left_col: usize,
    ) {
        if self.views.len() >= self.max_views
            && !self.views.contains_key(&buffer_id)
        {
            // Evict oldest (arbitrary) entry
            if let Some(&key) =
                self.views.keys().next()
            {
                self.views.remove(&key);
            }
        }
        self.views.insert(
            buffer_id,
            ViewState {
                buffer_id,
                cursor,
                top_line,
                left_col,
                folds: Vec::new(),
                local_options: HashMap::new(),
            },
        );
    }

    /// Restore previously saved view for a buffer.
    pub fn restore(
        &self,
        buffer_id: BufferId,
    ) -> Option<&ViewState> {
        self.views.get(&buffer_id)
    }

    /// Remove saved view for a buffer.
    pub fn remove(&mut self, buffer_id: BufferId) {
        self.views.remove(&buffer_id);
    }

    /// Number of saved views.
    pub fn count(&self) -> usize {
        self.views.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_restore() {
        let mut reg = ViewRegistry::new();
        let bid = BufferId(1);
        let cursor = CursorPosition {
            line: 10,
            grapheme_offset: 5,
            desired_col: None,
        };
        reg.save(bid, cursor, 5, 0);
        let view = reg.restore(bid).unwrap();
        assert_eq!(view.cursor.line, 10);
        assert_eq!(view.top_line, 5);
    }

    #[test]
    fn remove_view() {
        let mut reg = ViewRegistry::new();
        let bid = BufferId(1);
        let cursor = CursorPosition {
            line: 0,
            grapheme_offset: 0,
            desired_col: None,
        };
        reg.save(bid, cursor, 0, 0);
        assert_eq!(reg.count(), 1);
        reg.remove(bid);
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn max_views_eviction() {
        let mut reg = ViewRegistry::new();
        reg.max_views = 2;
        let c = CursorPosition {
            line: 0,
            grapheme_offset: 0,
            desired_col: None,
        };
        reg.save(BufferId(1), c, 0, 0);
        reg.save(BufferId(2), c, 0, 0);
        reg.save(BufferId(3), c, 0, 0);
        assert_eq!(reg.count(), 2);
    }
}
