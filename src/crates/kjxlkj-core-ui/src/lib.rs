//! UI model types and snapshot structures.
//!
//! This crate defines the types used for rendering the editor UI.

mod snapshot;
mod viewport;

pub use snapshot::{BufferSnapshot, EditorSnapshot, StatusLine};
pub use viewport::Viewport;

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
}
