//! UI model types and snapshot structures.
//!
//! This crate defines the types used for rendering the editor UI.

mod explorer;
mod finder;
mod jumplist;
mod popup;
mod quickfix;
mod snapshot;
mod view;
mod viewport;
mod which_key;
pub mod window;

pub use explorer::{Explorer, ExplorerNode, ExplorerRow, NodeKind, NodeState};
pub use finder::{
    Finder, FinderAction, FinderItem, FinderLocation, FinderQuery, FinderSource,
};
pub use jumplist::{ChangeEntry, Changelist, JumpEntry, Jumplist};
pub use popup::{
    Edge, Popup, PopupAnchor, PopupBorder, PopupConfig, PopupId, PopupKind, PopupManager,
};
pub use quickfix::{QuickfixEntry, QuickfixList, QuickfixManager, QuickfixType};
pub use snapshot::{BufferSnapshot, EditorSnapshot, StatusLine};
pub use view::{
    FocusManager, ViewBounds, ViewConfig, ViewId, ViewLayout, ViewType, ViewVisibility,
};
pub use viewport::Viewport;
pub use which_key::{
    CommandPalette, KeyHint, PaletteCommand, WhichKey, WhichKeyConfig, WhichKeyPosition,
};
pub use window::{
    BufferId, Direction, SplitDirection, TabPage, Window, WindowCursor, WindowId, WindowManager,
    WindowNode, WindowOptions, WindowViewport,
};

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{Cursor, Mode};

    #[test]
    fn test_viewport_new() {
        let vp = Viewport::new(0, 24, 0, 80);
        assert_eq!(vp.height, 24);
        assert_eq!(vp.width, 80);
    }

    #[test]
    fn test_viewport_contains() {
        let vp = Viewport::new(0, 10, 0, 80);
        assert!(vp.is_line_visible(0));
        assert!(vp.is_line_visible(9));
        assert!(!vp.is_line_visible(10));
    }

    #[test]
    fn test_status_line_new() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Insert, "file.rs".to_string(), true, &cursor, 100);
        assert!(sl.mode.contains("INSERT"));
    }

    #[test]
    fn test_status_line_mode_normal() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Normal, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl.mode.contains("NORMAL"));
    }

    #[test]
    fn test_status_line_with_message() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Normal, "test.txt".to_string(), false, &cursor, 1)
            .with_message("Hello".to_string(), false);
        assert!(sl.message.is_some());
    }

    #[test]
    fn test_status_line_default() {
        let sl = StatusLine::default();
        assert!(sl.mode.is_empty());
    }

    #[test]
    fn test_status_line_mode_visual() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Visual, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl.mode.contains("VISUAL"));
    }

    #[test]
    fn test_status_line_mode_visual_line() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::VisualLine, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl.mode.contains("VISUAL_LINE"));
    }

    #[test]
    fn test_status_line_mode_visual_block() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::VisualBlock, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl.mode.contains("VISUAL_BLOCK"));
    }

    #[test]
    fn test_status_line_mode_command() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Command, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl.mode.contains("COMMAND"));
    }

    #[test]
    fn test_status_line_mode_replace() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Replace, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl.mode.contains("REPLACE"));
    }

    #[test]
    fn test_viewport_top_line() {
        let vp = Viewport::new(10, 24, 0, 80);
        assert_eq!(vp.top_line, 10);
    }

    #[test]
    fn test_viewport_left_col() {
        let vp = Viewport::new(0, 24, 5, 80);
        assert_eq!(vp.left_col, 5);
    }

    #[test]
    fn test_viewport_default() {
        let vp = Viewport::default();
        assert_eq!(vp.top_line, 0);
        assert_eq!(vp.left_col, 0);
    }

    #[test]
    fn test_viewport_clone() {
        let vp = Viewport::new(10, 20, 5, 80);
        let cloned = vp;
        assert_eq!(vp.top_line, cloned.top_line);
        assert_eq!(vp.height, cloned.height);
    }

    #[test]
    fn test_status_line_clone() {
        let cursor = Cursor::new(0, 0);
        let sl = StatusLine::new(Mode::Normal, "test.txt".to_string(), false, &cursor, 1);
        let cloned = sl.clone();
        assert_eq!(sl.mode, cloned.mode);
    }

    #[test]
    fn test_status_line_debug() {
        let sl = StatusLine::default();
        let debug = format!("{:?}", sl);
        assert!(debug.contains("StatusLine"));
    }

    #[test]
    fn test_viewport_debug() {
        let vp = Viewport::new(0, 24, 0, 80);
        let debug = format!("{:?}", vp);
        assert!(debug.contains("Viewport"));
    }

    #[test]
    fn test_viewport_line_visible_edge() {
        let vp = Viewport::new(5, 10, 0, 80);
        assert!(!vp.is_line_visible(4));
        assert!(vp.is_line_visible(5));
        assert!(vp.is_line_visible(14));
        assert!(!vp.is_line_visible(15));
    }

    #[test]
    fn test_status_line_modified_flag() {
        let cursor = Cursor::new(0, 0);
        let sl_modified = StatusLine::new(Mode::Normal, "test.txt".to_string(), true, &cursor, 1);
        let sl_clean = StatusLine::new(Mode::Normal, "test.txt".to_string(), false, &cursor, 1);
        assert!(sl_modified.modified);
        assert!(!sl_clean.modified);
    }

    #[test]
    fn test_viewport_zero_height() {
        let vp = Viewport::new(0, 0, 0, 80);
        assert_eq!(vp.height, 0);
    }

    #[test]
    fn test_viewport_zero_width() {
        let vp = Viewport::new(0, 24, 0, 0);
        assert_eq!(vp.width, 0);
    }

    #[test]
    fn test_status_line_position_display() {
        let cursor = Cursor::new(10, 20);
        let sl = StatusLine::new(Mode::Normal, "test.txt".to_string(), false, &cursor, 100);
        assert_eq!(sl.line, 10);
        assert_eq!(sl.col, 20);
        assert_eq!(sl.total_lines, 100);
    }
}
