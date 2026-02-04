//! Render diffing for efficient updates.

use kjxlkj_core_ui::EditorSnapshot;

/// Represents changes between two snapshots.
#[derive(Debug, Clone, Default)]
pub struct RenderDiff {
    /// Lines that changed.
    pub changed_lines: Vec<usize>,
    /// Whether status line changed.
    pub status_changed: bool,
    /// Whether command line changed.
    pub command_changed: bool,
    /// Whether cursor position changed.
    pub cursor_changed: bool,
    /// Whether mode changed.
    pub mode_changed: bool,
    /// Whether selection changed.
    pub selection_changed: bool,
}

impl RenderDiff {
    /// Compute diff between two snapshots.
    pub fn compute(old: &EditorSnapshot, new: &EditorSnapshot) -> Self {
        let mut changed_lines = Vec::new();

        // Check each visible line
        let old_lines = &old.buffer.lines;
        let new_lines = &new.buffer.lines;

        let max_lines = old_lines.len().max(new_lines.len());
        for i in 0..max_lines {
            let old_line = old_lines.get(i);
            let new_line = new_lines.get(i);
            if old_line != new_line {
                changed_lines.push(i);
            }
        }

        // Check viewport scroll
        if old.buffer.viewport.top_line != new.buffer.viewport.top_line {
            // All lines changed
            changed_lines = (0..max_lines).collect();
        }

        Self {
            changed_lines,
            status_changed: old.status != new.status,
            command_changed: old.command_line != new.command_line,
            cursor_changed: old.cursor != new.cursor,
            mode_changed: old.mode != new.mode,
            selection_changed: old.selection != new.selection,
        }
    }

    /// Check if anything needs to be redrawn.
    pub fn needs_redraw(&self) -> bool {
        !self.changed_lines.is_empty()
            || self.status_changed
            || self.command_changed
            || self.cursor_changed
            || self.mode_changed
            || self.selection_changed
    }

    /// Check if a full redraw is needed.
    pub fn needs_full_redraw(&self) -> bool {
        // If more than half the lines changed, do a full redraw
        self.changed_lines.len() > 10 || self.mode_changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, BufferVersion, BufferName, Cursor, Mode};
    use kjxlkj_core_ui::{BufferSnapshot, StatusLine, Viewport};

    fn make_snapshot(lines: Vec<&str>) -> EditorSnapshot {
        let viewport = Viewport::new(0, 24, 0, 80);
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::new(1),
            lines.len(),
            lines.iter().map(|s| s.to_string()).collect(),
            viewport,
            false,
        );
        let cursor = Cursor::new(0, 0);
        let status = StatusLine::new(Mode::Normal, "test".to_string(), false, &cursor, lines.len());
        EditorSnapshot::new(buffer, cursor, Mode::Normal, None, status, None, None, 80, 24)
    }

    #[test]
    fn test_diff_no_changes() {
        let old = make_snapshot(vec!["hello", "world"]);
        let new = make_snapshot(vec!["hello", "world"]);
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.changed_lines.is_empty());
        assert!(!diff.needs_redraw());
    }

    #[test]
    fn test_diff_line_changed() {
        let old = make_snapshot(vec!["hello", "world"]);
        let new = make_snapshot(vec!["hello", "rust"]);
        let diff = RenderDiff::compute(&old, &new);
        assert_eq!(diff.changed_lines, vec![1]);
        assert!(diff.needs_redraw());
    }

    #[test]
    fn test_diff_all_lines_changed() {
        let old = make_snapshot(vec!["a", "b", "c"]);
        let new = make_snapshot(vec!["x", "y", "z"]);
        let diff = RenderDiff::compute(&old, &new);
        assert_eq!(diff.changed_lines.len(), 3);
    }

    #[test]
    fn test_diff_first_line_changed() {
        let old = make_snapshot(vec!["hello", "world"]);
        let new = make_snapshot(vec!["HELLO", "world"]);
        let diff = RenderDiff::compute(&old, &new);
        assert_eq!(diff.changed_lines, vec![0]);
    }

    #[test]
    fn test_diff_new_line_added() {
        let old = make_snapshot(vec!["hello"]);
        let new = make_snapshot(vec!["hello", "world"]);
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.changed_lines.contains(&1));
    }

    #[test]
    fn test_diff_line_removed() {
        let old = make_snapshot(vec!["hello", "world"]);
        let new = make_snapshot(vec!["hello"]);
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.changed_lines.contains(&1));
    }

    #[test]
    fn test_needs_full_redraw_many_lines() {
        let old = make_snapshot((0..20).map(|i| format!("line {}", i)).collect::<Vec<_>>().iter().map(|s| s.as_str()).collect());
        let new = make_snapshot((0..20).map(|i| format!("changed {}", i)).collect::<Vec<_>>().iter().map(|s| s.as_str()).collect());
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.needs_full_redraw());
    }

    #[test]
    fn test_needs_redraw_cursor_change() {
        let old = make_snapshot(vec!["hello"]);
        let mut new = make_snapshot(vec!["hello"]);
        new.cursor.position.col = 5;
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.needs_redraw());
        assert!(diff.cursor_changed);
    }

    #[test]
    fn test_mode_change_triggers_full_redraw() {
        let old = make_snapshot(vec!["hello"]);
        let mut new = make_snapshot(vec!["hello"]);
        new.mode = Mode::Insert;
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.mode_changed);
        assert!(diff.needs_full_redraw());
    }

    #[test]
    fn test_render_diff_default() {
        let diff = RenderDiff::default();
        assert!(diff.changed_lines.is_empty());
        assert!(!diff.needs_redraw());
    }

    #[test]
    fn test_status_changed() {
        let old = make_snapshot(vec!["hello"]);
        let mut new = make_snapshot(vec!["hello"]);
        new.status.modified = true;
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.status_changed);
        assert!(diff.needs_redraw());
    }

    #[test]
    fn test_command_changed() {
        let old = make_snapshot(vec!["hello"]);
        let mut new = make_snapshot(vec!["hello"]);
        new.command_line = Some(":wq".to_string());
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.command_changed);
        assert!(diff.needs_redraw());
    }

    #[test]
    fn test_multiple_lines_changed() {
        let old = make_snapshot(vec!["a", "b", "c", "d", "e"]);
        let new = make_snapshot(vec!["a", "X", "c", "Y", "e"]);
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.changed_lines.contains(&1));
        assert!(diff.changed_lines.contains(&3));
        assert!(!diff.changed_lines.contains(&0));
        assert!(!diff.changed_lines.contains(&2));
    }

    #[test]
    fn test_render_diff_clone() {
        let diff = RenderDiff {
            changed_lines: vec![1, 2],
            status_changed: true,
            command_changed: false,
            cursor_changed: true,
            mode_changed: false,
            selection_changed: false,
        };
        let cloned = diff.clone();
        assert_eq!(cloned.changed_lines, diff.changed_lines);
        assert_eq!(cloned.status_changed, diff.status_changed);
    }

    #[test]
    fn test_render_diff_debug() {
        let diff = RenderDiff::default();
        let debug = format!("{:?}", diff);
        assert!(debug.contains("RenderDiff"));
    }

    #[test]
    fn test_selection_changed() {
        let old = make_snapshot(vec!["hello"]);
        let mut new = make_snapshot(vec!["hello"]);
        new.selection = Some(kjxlkj_core_types::Selection::char_wise(
            kjxlkj_core_types::Position::new(0, 0),
            kjxlkj_core_types::Position::new(0, 5),
        ));
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.selection_changed);
        assert!(diff.needs_redraw());
    }

    #[test]
    fn test_empty_snapshot_diff() {
        let old = make_snapshot(vec![]);
        let new = make_snapshot(vec![]);
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.changed_lines.is_empty());
    }

    #[test]
    fn test_only_cursor_change() {
        let old = make_snapshot(vec!["hello", "world"]);
        let mut new = make_snapshot(vec!["hello", "world"]);
        new.cursor = Cursor::new(1, 2);
        let diff = RenderDiff::compute(&old, &new);
        assert!(diff.cursor_changed);
        assert!(!diff.status_changed);
        assert!(diff.changed_lines.is_empty());
    }
}
