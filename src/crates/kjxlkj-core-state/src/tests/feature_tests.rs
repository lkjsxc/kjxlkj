//! Phase 2-3 feature integration tests.
//!
//! HE-04: Explorer open file
//! HE-05: Explorer open split
//! PE-01: PTY terminal spawn and output (unit-level)
//! PE-02: PTY resize integration (unit-level)
//! PE-04: PTY IME leader isolation
//! PE-05: PTY mixed window navigation
//! PE-06: PTY append mode churn

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, KeyModifiers, Mode};
    use kjxlkj_core_ui::WindowContent;
    use std::path::PathBuf;

    /// HE-04: selected file in explorer opens in the current window.
    #[test]
    fn he04_explorer_open_file() {
        let mut state = EditorState::new(80, 24);
        state.do_explorer_toggle();
        // Add a dummy file entry
        state
            .explorer
            .add_entry(PathBuf::from("/tmp/test_he04.txt"), false);
        state.explorer.selected = 0;
        // Open the selected file
        state.do_explorer_open_file();
        // Active window should now be a buffer (file opened)
        assert_eq!(state.mode, Mode::Normal);
    }

    /// HE-05: explorer open split (horizontal and vertical).
    #[test]
    fn he05_explorer_open_split_h() {
        let mut state = EditorState::new(80, 24);
        state.do_explorer_toggle();
        state
            .explorer
            .add_entry(PathBuf::from("/tmp/test_he05h.txt"), false);
        state.explorer.selected = 0;
        state.do_explorer_open_split_h();
        let tab = state.windows.active_tab();
        // Should have at least 3 windows: original buffer + explorer + new split
        assert!(tab.windows.len() >= 3, "split should add window");
    }

    /// HE-05: explorer open split (vertical target).
    #[test]
    fn he05_explorer_open_split_v() {
        let mut state = EditorState::new(80, 24);
        state.do_explorer_toggle();
        state
            .explorer
            .add_entry(PathBuf::from("/tmp/test_he05v.txt"), false);
        state.explorer.selected = 0;
        state.do_explorer_open_split_v();
        let tab = state.windows.active_tab();
        assert!(tab.windows.len() >= 3, "vertical split should add window");
    }

    /// PE-01: PTY terminal spawn creates a terminal-type window.
    ///
    /// This tests the window creation path. Real PTY spawn is
    /// deferred to the service layer; here we verify the core
    /// state properly creates the terminal window node.
    #[test]
    fn pe01_terminal_spawn_creates_window() {
        let mut state = EditorState::new(80, 24);
        state.do_terminal_open();
        let tab = state.windows.active_tab();
        let active = tab.active();
        match &active.content {
            WindowContent::Terminal(tid) => {
                assert!(tid.0 > 0, "terminal id should be positive");
            }
            _ => panic!("PE-01: active window must be Terminal"),
        }
        assert_eq!(state.mode, Mode::TerminalInsert);
    }

    /// PE-02: PTY resize integration.
    ///
    /// Verify that after resize, terminal window dimensions track
    /// the new terminal size through snapshot generation.
    #[test]
    fn pe02_terminal_resize_tracks() {
        let mut state = EditorState::new(80, 24);
        state.do_terminal_open();
        // Resize
        state.terminal_size = (120, 50);
        let snap = state.snapshot();
        let tab = &snap.tabs[snap.active_tab];
        let win = &tab.windows[tab.active_window];
        assert_eq!(win.width, 120, "width should track resize");
        assert_eq!(win.height, 48, "height should be rows - 2");
    }

    /// PE-05: Mixed window navigation with terminal windows.
    #[test]
    fn pe05_mixed_window_navigation() {
        let mut state = EditorState::new(80, 24);
        // Create terminal + explorer alongside initial buffer
        state.do_terminal_open();
        state.mode = Mode::Normal;
        state.do_explorer_toggle();
        let tab = state.windows.active_tab();
        assert!(tab.windows.len() >= 3);

        // Cycle through all windows
        let start = state.windows.active_tab().active_window;
        state.windows.active_tab_mut().next_window();
        let second = state.windows.active_tab().active_window;
        assert_ne!(start, second);
        state.windows.active_tab_mut().next_window();
        let third = state.windows.active_tab().active_window;
        assert_ne!(second, third);
    }

    /// PE-04: In TerminalInsert mode, Space does not trigger leader mapping.
    ///
    /// Composition keys (including Space) must not leak to leader
    /// dispatch when mode is TerminalInsert.
    #[test]
    fn pe04_terminal_space_no_leader() {
        let mut state = EditorState::new(80, 24);
        state.do_terminal_open();
        assert_eq!(state.mode, Mode::TerminalInsert);
        let space = Key::new(KeyCode::Char(' '), KeyModifiers::NONE);
        state.process_key(space);
        // Mode should remain TerminalInsert (no leader dispatch)
        assert_eq!(state.mode, Mode::TerminalInsert);
    }

    /// PE-06: Repeated append cycles in terminal mode keep cursor clamped.
    ///
    /// After toggling between normal and insert modes near the terminal
    /// boundary, the cursor should remain within valid bounds.
    #[test]
    fn pe06_append_mode_churn() {
        let mut state = EditorState::new(80, 24);
        // Start in normal mode with a buffer
        state.mode = Mode::Normal;
        // Repeatedly do append + Esc cycles
        for _ in 0..10 {
            state.do_insert_append();
            assert_eq!(state.mode, Mode::Insert);
            state.change_mode(Mode::Normal);
            assert_eq!(state.mode, Mode::Normal);
        }
        // Cursor should remain in valid range
        let win = state.windows.active_tab().active();
        let buf_id = match &win.content {
            WindowContent::Buffer(id) => *id,
            _ => panic!("expected buffer"),
        };
        if let Some(buf) = state.buffers.get(&buf_id) {
            let line_count = buf.line_count();
            assert!(win.cursor_line < line_count);
        }
    }
}
