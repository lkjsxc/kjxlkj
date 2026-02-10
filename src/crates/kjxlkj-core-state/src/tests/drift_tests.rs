//! Tests for drift-matrix closures (R-WIN-01, R-I18N-01, R-WRAP-01).

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, KeyModifiers, Mode};

    /// R-I18N-01: IME commit inserts composed text in Insert mode.
    #[test]
    fn ime_commit_inserts_text() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        // Start composition
        state.ime.start_composition();
        state.ime.feed_preedit('か');
        state.ime.feed_preedit('な');
        // Commit via Enter key
        let enter = Key::new(KeyCode::Enter, KeyModifiers::NONE);
        state.process_key(enter);
        state.change_mode(Mode::Normal);
        let bid = state.active_buffer_id().unwrap();
        let content = state.buffers.get(&bid).unwrap().to_string_content();
        assert!(
            content.contains("かな"),
            "IME commit should insert text, got: {content}"
        );
    }

    /// R-I18N-01: IME cancel does not insert text.
    #[test]
    fn ime_cancel_inserts_nothing() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.ime.start_composition();
        state.ime.feed_preedit('あ');
        // Cancel via Esc
        let esc = Key::new(KeyCode::Esc, KeyModifiers::NONE);
        state.process_key(esc);
        let bid = state.active_buffer_id().unwrap();
        let content = state.buffers.get(&bid).unwrap().to_string_content();
        assert!(
            !content.contains('あ'),
            "IME cancel should not insert text, got: {content}"
        );
    }

    /// R-I18N-01: IME consumes Space during composition (no leader).
    #[test]
    fn ime_space_consumed_during_composition() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.ime.start_composition();
        state.ime.feed_preedit('か');
        // Space during composition should be consumed, not trigger leader
        let space = Key::new(KeyCode::Char(' '), KeyModifiers::NONE);
        state.process_key(space);
        assert_eq!(state.mode, Mode::Insert, "space should not change mode");
        assert!(state.ime.is_composing(), "IME should still be composing");
    }

    /// R-WIN-01: Ctrl-w h/l directional focus in vertical split.
    #[test]
    fn directional_focus_vertical_split() {
        let mut state = EditorState::new(80, 24);
        state.do_window_split_v();
        let tab = state.windows.active_tab();
        assert_eq!(tab.windows.len(), 2);
        let right_idx = state.windows.active_tab().active_window;
        let ctrl_w = Key::new(KeyCode::Char('w'), KeyModifiers::CTRL);
        let h = Key::new(KeyCode::Char('h'), KeyModifiers::NONE);
        state.process_key(ctrl_w.clone());
        state.process_key(h);
        let left_idx = state.windows.active_tab().active_window;
        assert_ne!(right_idx, left_idx, "Ctrl-w h should move left");
        let l = Key::new(KeyCode::Char('l'), KeyModifiers::NONE);
        state.process_key(ctrl_w.clone());
        state.process_key(l);
        let back = state.windows.active_tab().active_window;
        assert_eq!(back, right_idx, "Ctrl-w l should move right");
    }

    /// R-WIN-01: Ctrl-w j/k directional focus in horizontal split.
    #[test]
    fn directional_focus_horizontal_split() {
        let mut state = EditorState::new(80, 24);
        state.do_window_split_h();
        let tab = state.windows.active_tab();
        assert_eq!(tab.windows.len(), 2);
        let bottom_idx = state.windows.active_tab().active_window;
        let ctrl_w = Key::new(KeyCode::Char('w'), KeyModifiers::CTRL);
        let k = Key::new(KeyCode::Char('k'), KeyModifiers::NONE);
        state.process_key(ctrl_w.clone());
        state.process_key(k);
        let top_idx = state.windows.active_tab().active_window;
        assert_ne!(bottom_idx, top_idx, "Ctrl-w k should move up");
        let j = Key::new(KeyCode::Char('j'), KeyModifiers::NONE);
        state.process_key(ctrl_w.clone());
        state.process_key(j);
        let back = state.windows.active_tab().active_window;
        assert_eq!(back, bottom_idx, "Ctrl-w j should move down");
    }
}
