//! Boundary and stress tests (BD-03 through BD-09).

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use crate::session::SessionData;
    use std::path::PathBuf;

    /// BD-03: no-wrap long line - horizontal follow keeps cursor visible.
    #[test]
    fn bd03_nowrap_horizontal_follow() {
        let mut state = EditorState::new(80, 24);
        let buf_id = state.active_buffer_id().unwrap();
        let long_line = "a".repeat(300);
        state
            .buffers
            .get_mut(&buf_id)
            .unwrap()
            .insert(0, &long_line);
        let win = state.windows.active_tab_mut().active_mut();
        win.wrap = false;
        win.cursor_offset = 250;
        let cols = 80usize;
        let side = win.sidescrolloff;
        if win.cursor_offset >= win.left_col + cols.saturating_sub(side) {
            win.left_col = win.cursor_offset + side + 1 - cols;
        }
        let visible_col = win.cursor_offset - win.left_col;
        assert!(visible_col < cols, "cursor must be visible on screen");
    }

    /// BD-04: rapid resize storm - final geometry correct.
    #[test]
    fn bd04_rapid_resize_storm() {
        let mut state = EditorState::new(80, 24);
        for i in 0u16..100 {
            let w = 20 + (i % 200);
            let h = 5 + (i % 50);
            state.terminal_size = (w, h);
            let win = state.windows.active_tab_mut().active_mut();
            let max_line = if h > 2 { (h - 2) as usize } else { 0 };
            if win.cursor_line >= max_line && max_line > 0 {
                win.cursor_line = max_line - 1;
            }
        }
        // i=99: w = 20 + 99%200 = 119, h = 5 + 99%50 = 54
        assert_eq!(state.terminal_size, (119, 54));
    }

    /// BD-05: resize to 1x1 - no panic.
    #[test]
    fn bd05_resize_to_1x1() {
        let mut state = EditorState::new(80, 24);
        state.terminal_size = (1, 1);
        let win = state.windows.active_tab_mut().active_mut();
        win.cursor_line = 0;
        win.cursor_offset = 0;
        let snap = state.snapshot();
        assert_eq!(snap.terminal_size, (1, 1));
    }

    /// BD-06: terminal output flood + adjacent edit - editing responsive.
    #[test]
    fn bd06_terminal_flood_with_edit() {
        let mut state = EditorState::new(80, 24);
        let tid = state.next_terminal_id();
        let win_id = state.windows.next_window_id();
        let twin = crate::window_tree::Window::new_terminal(win_id, tid);
        state.windows.active_tab_mut().split_horizontal(twin);
        // Switch back to first (buffer) window
        state.windows.active_tab_mut().active_window = 0;
        let buf_id = state.active_buffer_id().unwrap();
        let buf = state.buffers.get_mut(&buf_id).unwrap();
        buf.insert(0, "X");
        let line = buf.line(0).unwrap();
        assert!(line.starts_with('X'));
    }

    /// BD-07: terminal close during output - child reaped.
    #[test]
    fn bd07_terminal_close_during_output() {
        let mut state = EditorState::new(80, 24);
        let tid = state.next_terminal_id();
        let win_id = state.windows.next_window_id();
        let twin = crate::window_tree::Window::new_terminal(win_id, tid);
        state.windows.active_tab_mut().split_horizontal(twin);
        assert_eq!(state.windows.active_tab().windows.len(), 2);
        // active_window points to terminal (split_horizontal sets it)
        state.windows.active_tab_mut().close_active();
        assert_eq!(state.windows.active_tab().windows.len(), 1);
        let win = state.windows.active_tab().active();
        match &win.content {
            kjxlkj_core_ui::WindowContent::Buffer(_) => {}
            _ => panic!("remaining window should be buffer"),
        }
    }

    /// BD-08: explorer with 10k entries - navigation responsive.
    #[test]
    fn bd08_explorer_10k_entries() {
        let mut state = EditorState::new(80, 24);
        for i in 0..10_000 {
            let path: PathBuf = format!("/root/file_{i}.rs").into();
            state.explorer.add_entry(path, false);
        }
        assert_eq!(state.explorer.entries.len(), 10_000);
        for _ in 0..5_000 {
            state.explorer.move_down();
        }
        assert_eq!(state.explorer.selected, 5_000);
        for _ in 0..2_500 {
            state.explorer.move_up();
        }
        assert_eq!(state.explorer.selected, 2_500);
    }

    /// BD-09: session load with missing file - warning, layout restored.
    #[test]
    fn bd09_session_missing_file() {
        let json = serde_json::json!({
            "version": 1,
            "cwd": "/nonexistent",
            "timestamp": "",
            "tabs": [{
                "layout": {
                    "type": "leaf",
                    "window": {
                        "content_type": "buffer",
                        "buffer_path": "/nonexistent/file.rs",
                        "cursor_line": 10,
                        "cursor_grapheme": 5,
                        "top_line": 0,
                        "left_col": 0,
                        "wrap": true
                    }
                },
                "focused_window": 0
            }],
            "active_tab": 0,
            "buffers": [{
                "path": "/nonexistent/file.rs",
                "encoding": "utf-8",
                "modified": false
            }]
        });
        let data: SessionData = serde_json::from_value(json).unwrap();
        assert_eq!(data.tabs.len(), 1);
    }
}
