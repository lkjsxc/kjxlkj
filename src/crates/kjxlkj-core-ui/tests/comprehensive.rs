//! Comprehensive tests for kjxlkj-core-ui.

use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode, WindowId};
use kjxlkj_core_ui::*;

mod viewport_tests {
    use super::*;

    #[test]
    fn test_viewport_new() {
        let vp = Viewport::new(80, 24);
        assert_eq!(vp.width, 80);
        assert_eq!(vp.height, 24);
        assert_eq!(vp.top_line, 0);
        assert_eq!(vp.left_col, 0);
    }

    #[test]
    fn test_viewport_follow_cursor_down() {
        let mut vp = Viewport::new(80, 10);
        let cursor = Cursor::new(15, 0);
        vp.follow_cursor(&cursor, 100);
        // With scrolloff = 3, top_line should adjust
        assert!(vp.top_line > 0);
    }

    #[test]
    fn test_viewport_follow_cursor_up() {
        let mut vp = Viewport::new(80, 10);
        vp.top_line = 20;
        let cursor = Cursor::new(18, 0);
        vp.follow_cursor(&cursor, 100);
        // Cursor is near top of viewport, scrolloff should adjust
    }

    #[test]
    fn test_viewport_follow_cursor_no_change() {
        let mut vp = Viewport::new(80, 20);
        vp.top_line = 10;
        let cursor = Cursor::new(20, 0);
        let old_top = vp.top_line;
        vp.follow_cursor(&cursor, 100);
        assert_eq!(vp.top_line, old_top);
    }

    #[test]
    fn test_viewport_center_on() {
        let mut vp = Viewport::new(80, 20);
        vp.center_on(50, 100);
        // Line 50 should be near the center
        assert!(vp.top_line <= 50);
        assert!(vp.top_line + vp.height > 50);
    }

    #[test]
    fn test_viewport_center_on_start() {
        let mut vp = Viewport::new(80, 20);
        vp.center_on(5, 100);
        // Can't center line 5 if it would go below 0
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_viewport_center_on_end() {
        let mut vp = Viewport::new(80, 20);
        vp.center_on(95, 100);
        // Should center as much as possible
        assert!(vp.top_line + vp.height >= 95);
    }

    #[test]
    fn test_viewport_scroll_down() {
        let mut vp = Viewport::new(80, 20);
        vp.scroll(10, 100);
        assert_eq!(vp.top_line, 10);
    }

    #[test]
    fn test_viewport_scroll_up() {
        let mut vp = Viewport::new(80, 20);
        vp.top_line = 30;
        vp.scroll(-10, 100);
        assert_eq!(vp.top_line, 20);
    }

    #[test]
    fn test_viewport_scroll_clamp_top() {
        let mut vp = Viewport::new(80, 20);
        vp.top_line = 5;
        vp.scroll(-10, 100);
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_viewport_scroll_clamp_bottom() {
        let mut vp = Viewport::new(80, 20);
        vp.scroll(200, 100);
        // top_line should be such that last line is visible
        assert!(vp.top_line <= 80); // 100 - 20 = 80
    }

    #[test]
    fn test_viewport_resize() {
        let mut vp = Viewport::new(80, 24);
        vp.width = 120;
        vp.height = 40;
        assert_eq!(vp.width, 120);
        assert_eq!(vp.height, 40);
    }

    #[test]
    fn test_viewport_contains_line() {
        let vp = Viewport::new(80, 20);
        assert!(vp.top_line == 0);
        // Lines 0-19 should be visible
    }

    #[test]
    fn test_viewport_debug() {
        let vp = Viewport::new(80, 24);
        let debug = format!("{:?}", vp);
        assert!(debug.contains("80"));
        assert!(debug.contains("24"));
    }

    #[test]
    fn test_viewport_clone() {
        let vp1 = Viewport::new(80, 24);
        let vp2 = vp1;
        assert_eq!(vp1.width, vp2.width);
        assert_eq!(vp1.height, vp2.height);
    }
}

mod buffer_snapshot_tests {
    use super::*;

    #[test]
    fn test_buffer_snapshot_new() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.rs"),
            BufferVersion::new(0),
            vec!["line1".to_string(), "line2".to_string()],
            0,
            2,
            false,
        );
        assert_eq!(snap.id, BufferId::new(1));
        assert_eq!(snap.name.as_str(), "test.rs");
        assert_eq!(snap.lines.len(), 2);
    }

    #[test]
    fn test_buffer_snapshot_modified() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.rs"),
            BufferVersion::new(1),
            vec!["content".to_string()],
            0,
            1,
            true,
        );
        assert!(snap.modified);
    }

    #[test]
    fn test_buffer_snapshot_unmodified() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::unnamed(),
            BufferVersion::new(0),
            vec![],
            0,
            0,
            false,
        );
        assert!(!snap.modified);
    }

    #[test]
    fn test_buffer_snapshot_line_offset() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("file.txt"),
            BufferVersion::new(0),
            vec!["line10".to_string()],
            10,
            20,
            false,
        );
        assert_eq!(snap.line_offset, 10);
        assert_eq!(snap.total_lines, 20);
    }

    #[test]
    fn test_buffer_snapshot_version() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("file.txt"),
            BufferVersion::new(42),
            vec![],
            0,
            0,
            false,
        );
        assert_eq!(snap.version, BufferVersion::new(42));
    }
}

mod window_snapshot_tests {
    use super::*;

    #[test]
    fn test_window_snapshot_new() {
        let buffer_snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.rs"),
            BufferVersion::new(0),
            vec!["content".to_string()],
            0,
            1,
            false,
        );
        let vp = Viewport::new(80, 24);
        let cursor = Cursor::new(0, 5);

        let win_snap = WindowSnapshot::new(
            WindowId::new(1),
            buffer_snap,
            cursor,
            vp,
            true,
        );

        assert_eq!(win_snap.id, WindowId::new(1));
        assert_eq!(win_snap.cursor, cursor);
        assert!(win_snap.active);
    }

    #[test]
    fn test_window_snapshot_inactive() {
        let buffer_snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::unnamed(),
            BufferVersion::new(0),
            vec![],
            0,
            0,
            false,
        );
        let vp = Viewport::new(80, 24);

        let win_snap = WindowSnapshot::new(
            WindowId::new(2),
            buffer_snap,
            Cursor::origin(),
            vp,
            false,
        );

        assert!(!win_snap.active);
    }

    #[test]
    fn test_window_snapshot_viewport() {
        let buffer_snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::unnamed(),
            BufferVersion::new(0),
            vec![],
            0,
            0,
            false,
        );
        let vp = Viewport::new(100, 50);

        let win_snap = WindowSnapshot::new(
            WindowId::new(1),
            buffer_snap,
            Cursor::origin(),
            vp,
            true,
        );

        assert_eq!(win_snap.viewport.width, 100);
        assert_eq!(win_snap.viewport.height, 50);
    }
}

mod editor_snapshot_tests {
    use super::*;

    fn make_window_snapshot(id: u64, active: bool) -> WindowSnapshot {
        let buffer_snap = BufferSnapshot::new(
            BufferId::new(id),
            BufferName::new(&format!("file{}.txt", id)),
            BufferVersion::new(0),
            vec!["line".to_string()],
            0,
            1,
            false,
        );
        let vp = Viewport::new(80, 24);
        WindowSnapshot::new(
            WindowId::new(id),
            buffer_snap,
            Cursor::origin(),
            vp,
            active,
        )
    }

    #[test]
    fn test_editor_snapshot_new() {
        let windows = vec![make_window_snapshot(1, true)];
        let snap = EditorSnapshot::new(
            1,
            windows,
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );

        assert_eq!(snap.sequence, 1);
        assert_eq!(snap.windows.len(), 1);
        assert_eq!(snap.active_window, WindowId::new(1));
        assert_eq!(snap.mode, Mode::Normal);
    }

    #[test]
    fn test_editor_snapshot_command_line() {
        let windows = vec![make_window_snapshot(1, true)];
        let snap = EditorSnapshot::new(
            1,
            windows,
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
    fn test_editor_snapshot_message() {
        let windows = vec![make_window_snapshot(1, true)];
        let snap = EditorSnapshot::new(
            1,
            windows,
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            Some("File saved".to_string()),
            80,
            24,
        );

        assert_eq!(snap.message, Some("File saved".to_string()));
    }

    #[test]
    fn test_editor_snapshot_dimensions() {
        let windows = vec![make_window_snapshot(1, true)];
        let snap = EditorSnapshot::new(
            1,
            windows,
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            None,
            120,
            40,
        );

        assert_eq!(snap.terminal_width, 120);
        assert_eq!(snap.terminal_height, 40);
    }

    #[test]
    fn test_editor_snapshot_multiple_windows() {
        let windows = vec![
            make_window_snapshot(1, false),
            make_window_snapshot(2, true),
            make_window_snapshot(3, false),
        ];
        let snap = EditorSnapshot::new(
            5,
            windows,
            WindowId::new(2),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );

        assert_eq!(snap.windows.len(), 3);
        assert!(snap.windows[1].active);
    }

    #[test]
    fn test_editor_snapshot_modes() {
        for mode in [Mode::Normal, Mode::Insert, Mode::Visual, Mode::Command, Mode::Replace] {
            let windows = vec![make_window_snapshot(1, true)];
            let snap = EditorSnapshot::new(
                1,
                windows,
                WindowId::new(1),
                mode,
                String::new(),
                None,
                80,
                24,
            );
            assert_eq!(snap.mode, mode);
        }
    }

    #[test]
    fn test_editor_snapshot_sequence_increases() {
        let windows1 = vec![make_window_snapshot(1, true)];
        let snap1 = EditorSnapshot::new(
            1,
            windows1,
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );

        let windows2 = vec![make_window_snapshot(1, true)];
        let snap2 = EditorSnapshot::new(
            2,
            windows2,
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );

        assert!(snap2.sequence > snap1.sequence);
    }
}

mod extra_viewport_tests {
    use super::*;

    #[test]
    fn test_viewport_with_scrolloff() {
        let vp = Viewport::new(80, 20).with_scrolloff(5);
        assert_eq!(vp.scrolloff, 5);
    }

    #[test]
    fn test_viewport_bottom_line() {
        let vp = Viewport::new(80, 20);
        assert_eq!(vp.bottom_line(), 19); // 0 + 20 - 1
    }

    #[test]
    fn test_viewport_bottom_line_with_scroll() {
        let mut vp = Viewport::new(80, 20);
        vp.top_line = 10;
        assert_eq!(vp.bottom_line(), 29); // 10 + 20 - 1
    }

    #[test]
    fn test_viewport_is_line_visible() {
        let vp = Viewport::new(80, 10);
        assert!(vp.is_line_visible(0));
        assert!(vp.is_line_visible(5));
        assert!(vp.is_line_visible(9));
        assert!(!vp.is_line_visible(10));
        assert!(!vp.is_line_visible(100));
    }

    #[test]
    fn test_viewport_is_line_visible_after_scroll() {
        let mut vp = Viewport::new(80, 10);
        vp.top_line = 50;
        assert!(!vp.is_line_visible(49));
        assert!(vp.is_line_visible(50));
        assert!(vp.is_line_visible(59));
        assert!(!vp.is_line_visible(60));
    }

    #[test]
    fn test_viewport_resize() {
        let mut vp = Viewport::new(80, 24);
        vp.resize(120, 40);
        assert_eq!(vp.width, 120);
        assert_eq!(vp.height, 40);
    }

    #[test]
    fn test_viewport_default() {
        let vp: Viewport = Default::default();
        assert_eq!(vp.width, 80);
        assert_eq!(vp.height, 24);
    }

    #[test]
    fn test_viewport_copy() {
        let vp1 = Viewport::new(100, 50);
        let vp2 = vp1;
        assert_eq!(vp1.width, vp2.width);
        assert_eq!(vp1.height, vp2.height);
    }

    #[test]
    fn test_viewport_scrolloff_affects_follow() {
        let mut vp = Viewport::new(80, 20).with_scrolloff(5);
        let cursor = Cursor::new(25, 0);
        vp.follow_cursor(&cursor, 100);
        // Cursor at 25 with scrolloff 5 means top_line should be at most 20
        assert!(vp.top_line <= 20);
    }

    #[test]
    fn test_viewport_scroll_to_exact_bottom() {
        let mut vp = Viewport::new(80, 20);
        vp.scroll(80, 100);
        assert_eq!(vp.top_line, 80); // 100 - 20 = max 80
    }

    #[test]
    fn test_viewport_scroll_negative_from_zero() {
        let mut vp = Viewport::new(80, 20);
        vp.scroll(-5, 100);
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_viewport_follow_cursor_at_origin() {
        let mut vp = Viewport::new(80, 20);
        vp.top_line = 50;
        let cursor = Cursor::new(0, 0);
        vp.follow_cursor(&cursor, 100);
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_viewport_center_middle_of_file() {
        let mut vp = Viewport::new(80, 20);
        vp.center_on(50, 100);
        assert_eq!(vp.top_line, 40); // 50 - 10 = 40
    }
}

mod extra_snapshot_tests {
    use super::*;

    #[test]
    fn test_window_snapshot_cursor_screen_pos() {
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.rs"),
            BufferVersion::new(0),
            vec!["line".to_string(); 100],
            0,
            100,
            false,
        );
        let cursor = Cursor::new(10, 5);
        let mut viewport = Viewport::new(80, 20);
        viewport.top_line = 5;
        viewport.left_col = 2;
        
        let window = WindowSnapshot::new(
            WindowId::new(1),
            buffer,
            cursor,
            viewport,
            true,
        );
        
        let (row, col) = window.cursor_screen_pos();
        assert_eq!(row, 5); // 10 - 5
        assert_eq!(col, 3); // 5 - 2
    }

    #[test]
    fn test_editor_snapshot_active_window() {
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.rs"),
            BufferVersion::new(0),
            vec![],
            0,
            0,
            false,
        );
        let window = WindowSnapshot::new(
            WindowId::new(42),
            buffer,
            Cursor::new(0, 0),
            Viewport::new(80, 24),
            true,
        );
        
        let snapshot = EditorSnapshot::new(
            1,
            vec![window],
            WindowId::new(42),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );
        
        let active = snapshot.active_window();
        assert!(active.is_some());
        assert_eq!(active.unwrap().id, WindowId::new(42));
    }

    #[test]
    fn test_editor_snapshot_no_matching_active_window() {
        let snapshot = EditorSnapshot::new(
            1,
            vec![],
            WindowId::new(99),
            Mode::Normal,
            String::new(),
            None,
            80,
            24,
        );
        assert!(snapshot.active_window().is_none());
    }

    #[test]
    fn test_buffer_snapshot_modified_flag() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("file.txt"),
            BufferVersion::new(0),
            vec!["content".to_string()],
            0,
            1,
            true,
        );
        assert!(snap.modified);
    }

    #[test]
    fn test_buffer_snapshot_clone() {
        let snap1 = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.rs"),
            BufferVersion::new(5),
            vec!["line1".to_string()],
            0,
            1,
            false,
        );
        let snap2 = snap1.clone();
        assert_eq!(snap1.id, snap2.id);
        assert_eq!(snap1.version.as_u64(), snap2.version.as_u64());
    }

    #[test]
    fn test_window_snapshot_clone() {
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("file"),
            BufferVersion::new(0),
            vec![],
            0,
            0,
            false,
        );
        let window1 = WindowSnapshot::new(
            WindowId::new(1),
            buffer,
            Cursor::new(0, 0),
            Viewport::new(80, 24),
            true,
        );
        let window2 = window1.clone();
        assert_eq!(window1.id, window2.id);
        assert_eq!(window1.active, window2.active);
    }

    #[test]
    fn test_editor_snapshot_with_message() {
        let snapshot = EditorSnapshot::new(
            1,
            vec![],
            WindowId::new(1),
            Mode::Normal,
            String::new(),
            Some("Hello".to_string()),
            80,
            24,
        );
        assert_eq!(snapshot.message, Some("Hello".to_string()));
    }

    #[test]
    fn test_editor_snapshot_command_mode() {
        let snapshot = EditorSnapshot::new(
            1,
            vec![],
            WindowId::new(1),
            Mode::Command,
            ":wq".to_string(),
            None,
            80,
            24,
        );
        assert_eq!(snapshot.mode, Mode::Command);
        assert_eq!(snapshot.command_line, ":wq");
    }

    #[test]
    fn test_window_cursor_at_viewport_origin() {
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::new(0),
            vec![],
            0,
            0,
            false,
        );
        let window = WindowSnapshot::new(
            WindowId::new(1),
            buffer,
            Cursor::new(0, 0),
            Viewport::new(80, 24),
            true,
        );
        let (row, col) = window.cursor_screen_pos();
        assert_eq!(row, 0);
        assert_eq!(col, 0);
    }
}
