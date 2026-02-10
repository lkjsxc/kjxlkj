//! Wiring and integration tests for Phase 2 features.
//!
//! WR-03: :terminal command route
//! WR-04: <leader>t route
//! WR-05: :Explorer and <leader>e route
//! WR-06: Ctrl-w mixed-window navigation
//! HE-04: Explorer open file
//! HE-06: Terminal open

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, KeyModifiers, Mode};
    use kjxlkj_core_ui::WindowContent;

    /// WR-03: `:terminal` command creates a terminal window.
    #[test]
    fn wr03_terminal_command_creates_window() {
        let mut state = EditorState::new(80, 24);
        state.execute_ex("terminal");
        let tab = state.windows.active_tab();
        let has_terminal = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Terminal(_)));
        assert!(has_terminal, ":terminal must create a terminal window");
        assert_eq!(state.mode, Mode::TerminalInsert);
    }

    /// WR-04: `<leader>t` creates terminal via same path.
    #[test]
    fn wr04_leader_t_creates_terminal() {
        let mut state = EditorState::new(80, 24);
        // Simulate <leader>t = Space then t then another key to commit
        let space = Key::new(KeyCode::Char(' '), KeyModifiers::NONE);
        let t = Key::new(KeyCode::Char('t'), KeyModifiers::NONE);
        let esc = Key::new(KeyCode::Esc, KeyModifiers::NONE);
        state.process_key(space);
        state.process_key(t);
        // <leader>t is 2-key prefix; next key resolves
        state.process_key(esc);
        let tab = state.windows.active_tab();
        let has_terminal = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Terminal(_)));
        assert!(has_terminal, "<leader>t must create a terminal window");
    }

    /// WR-05: `:Explorer` and `<leader>e` produce visible explorer window.
    #[test]
    fn wr05_explorer_command_creates_window() {
        let mut state = EditorState::new(80, 24);
        state.execute_ex("Explorer");
        let tab = state.windows.active_tab();
        let has_explorer = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Explorer));
        assert!(has_explorer, ":Explorer must create an explorer window");
    }

    /// WR-05b: `<leader>e` toggle creates explorer.
    #[test]
    fn wr05b_leader_e_creates_explorer() {
        let mut state = EditorState::new(80, 24);
        let space = Key::new(KeyCode::Char(' '), KeyModifiers::NONE);
        let e = Key::new(KeyCode::Char('e'), KeyModifiers::NONE);
        state.process_key(space);
        state.process_key(e);
        let tab = state.windows.active_tab();
        let has_explorer = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Explorer));
        assert!(has_explorer, "<leader>e must create an explorer window");
    }

    /// WR-05c: `<leader>e` toggles explorer closed.
    #[test]
    fn wr05c_leader_e_toggle_close() {
        let mut state = EditorState::new(80, 24);
        // Open explorer
        state.do_explorer_toggle();
        let count = state
            .windows
            .active_tab()
            .windows
            .iter()
            .filter(|w| matches!(w.content, WindowContent::Explorer))
            .count();
        assert_eq!(count, 1, "Explorer should be open");

        // Toggle close
        state.do_explorer_toggle();
        let count = state
            .windows
            .active_tab()
            .windows
            .iter()
            .filter(|w| matches!(w.content, WindowContent::Explorer))
            .count();
        assert_eq!(count, 0, "Explorer should be closed");
    }

    /// WR-06: Ctrl-w navigation across mixed window types.
    #[test]
    fn wr06_ctrl_w_mixed_windows() {
        let mut state = EditorState::new(80, 24);
        // Create explorer and terminal alongside buffer
        state.do_explorer_toggle();
        state.do_terminal_open();

        let tab = state.windows.active_tab();
        assert!(tab.windows.len() >= 3, "Should have 3+ windows");

        // Navigate through windows with Ctrl-w w
        let ctrl_w = Key::new(KeyCode::Char('w'), KeyModifiers::CTRL);
        let w = Key::new(KeyCode::Char('w'), KeyModifiers::NONE);

        // Save initial active window
        let initial = state.windows.active_tab().active_window;
        state.mode = Mode::Normal; // Reset mode for Ctrl-w

        state.process_key(ctrl_w.clone());
        state.process_key(w.clone());
        let after_first = state.windows.active_tab().active_window;
        assert_ne!(initial, after_first, "Ctrl-w w should cycle window");
    }

    /// HE-06: Terminal open and displays.
    #[test]
    fn he06_terminal_open() {
        let mut state = EditorState::new(80, 24);
        state.do_terminal_open();
        let tab = state.windows.active_tab();
        let active = tab.active();
        assert!(
            matches!(active.content, WindowContent::Terminal(_)),
            "Active window should be terminal"
        );
    }

    /// Test <leader>th creates horizontal terminal split.
    #[test]
    fn leader_th_horizontal_split() {
        let mut state = EditorState::new(80, 24);
        let space = Key::new(KeyCode::Char(' '), KeyModifiers::NONE);
        let t_key = Key::new(KeyCode::Char('t'), KeyModifiers::NONE);
        let h = Key::new(KeyCode::Char('h'), KeyModifiers::NONE);
        state.process_key(space);
        state.process_key(t_key);
        state.process_key(h);
        let tab = state.windows.active_tab();
        let has_terminal = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Terminal(_)));
        assert!(has_terminal, "<leader>th must create terminal");
    }

    /// Test <leader>tv creates vertical terminal split.
    #[test]
    fn leader_tv_vertical_split() {
        let mut state = EditorState::new(80, 24);
        let space = Key::new(KeyCode::Char(' '), KeyModifiers::NONE);
        let t_key = Key::new(KeyCode::Char('t'), KeyModifiers::NONE);
        let v = Key::new(KeyCode::Char('v'), KeyModifiers::NONE);
        state.process_key(space);
        state.process_key(t_key);
        state.process_key(v);
        let tab = state.windows.active_tab();
        let has_terminal = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Terminal(_)));
        assert!(has_terminal, "<leader>tv must create terminal");
    }
}
