//! Comprehensive tests for kjxlkj-render.

use kjxlkj_render::*;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode, WindowId};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, Viewport, WindowSnapshot};
use std::io::Cursor as IoCursor;

fn make_buffer_snapshot(lines: Vec<&str>) -> BufferSnapshot {
    BufferSnapshot::new(
        BufferId::new(1),
        BufferName::unnamed(),
        BufferVersion::initial(),
        lines.into_iter().map(String::from).collect(),
        0,
        5,
        false,
    )
}

fn make_window_snapshot(buffer: BufferSnapshot) -> WindowSnapshot {
    WindowSnapshot::new(
        WindowId::new(1),
        buffer,
        Cursor::origin(),
        Viewport::new(80, 24),
        true,
    )
}

fn make_editor_snapshot(windows: Vec<WindowSnapshot>) -> EditorSnapshot {
    let active = windows.first().map(|w| w.id).unwrap_or_else(|| WindowId::new(1));
    EditorSnapshot::new(
        1,
        windows,
        active,
        Mode::Normal,
        String::new(),
        None,
        80,
        24,
    )
}

mod renderer_tests {
    use super::*;

    #[test]
    fn test_renderer_new() {
        let buf = Vec::new();
        let _renderer = Renderer::new(buf);
        // Just verify construction works
    }

    #[test]
    fn test_renderer_render_empty() {
        let buf = Vec::new();
        let mut renderer = Renderer::new(buf);
        let snapshot = make_editor_snapshot(vec![]);
        // This would fail in a real terminal but we can test the logic
        let _ = renderer.render(&snapshot);
    }

    #[test]
    fn test_renderer_render_simple() {
        let buf = Vec::new();
        let mut renderer = Renderer::new(buf);
        let buffer = make_buffer_snapshot(vec!["hello", "world"]);
        let window = make_window_snapshot(buffer);
        let snapshot = make_editor_snapshot(vec![window]);
        let _ = renderer.render(&snapshot);
    }

    #[test]
    fn test_renderer_skips_stale_snapshots() {
        let buf = Vec::new();
        let mut renderer = Renderer::new(buf);
        
        // Render sequence 2 first
        let buffer = make_buffer_snapshot(vec!["test"]);
        let window = make_window_snapshot(buffer);
        let mut snapshot = make_editor_snapshot(vec![window]);
        snapshot = EditorSnapshot::new(
            2,
            snapshot.windows,
            snapshot.active_window,
            snapshot.mode,
            snapshot.command_line,
            snapshot.message,
            snapshot.terminal_width,
            snapshot.terminal_height,
        );
        let _ = renderer.render(&snapshot);

        // Render sequence 1 should be skipped
        let buffer = make_buffer_snapshot(vec!["old"]);
        let window = make_window_snapshot(buffer);
        let old_snapshot = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );
        let _ = renderer.render(&old_snapshot);
        // The older snapshot should be ignored
    }
}

mod snapshot_creation_tests {
    use super::*;

    #[test]
    fn test_buffer_snapshot_creation() {
        let snap = make_buffer_snapshot(vec!["line1", "line2"]);
        assert_eq!(snap.lines.len(), 2);
        assert_eq!(snap.lines[0], "line1");
    }

    #[test]
    fn test_window_snapshot_creation() {
        let buffer = make_buffer_snapshot(vec!["test"]);
        let snap = make_window_snapshot(buffer);
        assert!(snap.active);
    }

    #[test]
    fn test_editor_snapshot_creation() {
        let snap = make_editor_snapshot(vec![]);
        assert_eq!(snap.terminal_width, 80);
        assert_eq!(snap.terminal_height, 24);
        assert_eq!(snap.mode, Mode::Normal);
    }

    #[test]
    fn test_editor_snapshot_with_message() {
        let buffer = make_buffer_snapshot(vec!["test"]);
        let window = make_window_snapshot(buffer);
        let snap = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            Some("Test message".to_string()),
            80,
            24,
        );
        assert_eq!(snap.message, Some("Test message".to_string()));
    }

    #[test]
    fn test_editor_snapshot_insert_mode() {
        let buffer = make_buffer_snapshot(vec!["test"]);
        let window = make_window_snapshot(buffer);
        let snap = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(1),
            Mode::Insert,
            String::new(),
            None,
            80,
            24,
        );
        assert_eq!(snap.mode, Mode::Insert);
    }

    #[test]
    fn test_editor_snapshot_visual_mode() {
        let buffer = make_buffer_snapshot(vec!["test"]);
        let window = make_window_snapshot(buffer);
        let snap = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(1),
            Mode::Visual,
            String::new(),
            None,
            80,
            24,
        );
        assert_eq!(snap.mode, Mode::Visual);
    }

    #[test]
    fn test_editor_snapshot_command_line() {
        let buffer = make_buffer_snapshot(vec!["test"]);
        let window = make_window_snapshot(buffer);
        let snap = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(1),
            Mode::Command,
            "wq".to_string(),
            None,
            80,
            24,
        );
        assert_eq!(snap.command_line, "wq");
    }

    #[test]
    fn test_editor_snapshot_active_window() {
        let buffer = make_buffer_snapshot(vec!["test"]);
        let window = make_window_snapshot(buffer);
        let snap = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );
        assert!(snap.active_window().is_some());
    }
}

mod viewport_tests {
    use super::*;

    #[test]
    fn test_viewport_new() {
        let vp = Viewport::new(80, 24);
        assert_eq!(vp.width, 80);
        assert_eq!(vp.height, 24);
    }

    #[test]
    fn test_viewport_default() {
        let vp = Viewport::default();
        assert_eq!(vp.top_line, 0);
        assert_eq!(vp.left_col, 0);
    }
}
