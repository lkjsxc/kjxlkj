//! Phase 1 editor core integration tests.
//!
//! HE-01: Create, edit, save, quit
//! HE-08: Command-line option change
//! WR-07: Long-line display safety (via wrap algorithm)

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::Mode;

    /// HE-01: Create buffer, insert text, verify content matches.
    ///
    /// Full save-to-disk is deferred to service layer; this test
    /// verifies the in-memory create-edit-verify path.
    #[test]
    fn he01_create_edit_verify() {
        let mut state = EditorState::new(80, 24);
        // Enter insert mode
        state.enter_mode_with_cursor(Mode::Insert);
        // Type "Hello"
        for c in "Hello".chars() {
            state.do_insert_char(c);
        }
        // Return to normal mode
        state.change_mode(Mode::Normal);
        // Verify buffer content
        let buf_id = state.active_buffer_id().expect("should have buffer");
        let buf = state.buffers.get(&buf_id).expect("buffer exists");
        let content = buf.to_string_content();
        assert!(content.starts_with("Hello"), "buffer should contain Hello");
    }

    /// HE-01 extended: multiple lines of editing.
    #[test]
    fn he01_multiline_edit() {
        let mut state = EditorState::new(80, 24);
        state.enter_mode_with_cursor(Mode::Insert);
        for c in "line1".chars() {
            state.do_insert_char(c);
        }
        // Open line below
        state.change_mode(Mode::Normal);
        state.do_open_line_below();
        for c in "line2".chars() {
            state.do_insert_char(c);
        }
        state.change_mode(Mode::Normal);
        let buf_id = state.active_buffer_id().unwrap();
        let buf = state.buffers.get(&buf_id).unwrap();
        assert!(buf.line_count() >= 2, "should have 2+ lines");
    }

    /// HE-08: `:set wrap` and `:set nowrap` update window options.
    #[test]
    fn he08_set_command_wrap() {
        let mut state = EditorState::new(80, 24);
        // Default should be wrap=true
        assert!(state.windows.active_tab().active().wrap);
        // Execute :set nowrap
        state.execute_ex("set nowrap");
        assert!(
            !state.windows.active_tab().active().wrap,
            "nowrap should disable wrap"
        );
        // Execute :set wrap
        state.execute_ex("set wrap");
        assert!(
            state.windows.active_tab().active().wrap,
            "set wrap should enable wrap"
        );
    }

    /// HE-08: `:set number` and `:set nonumber`.
    #[test]
    fn he08_set_command_number() {
        let mut state = EditorState::new(80, 24);
        assert!(state.windows.active_tab().active().line_numbers);
        state.execute_ex("set nonumber");
        assert!(
            !state.windows.active_tab().active().line_numbers,
            "nonumber should disable"
        );
        state.execute_ex("set number");
        assert!(
            state.windows.active_tab().active().line_numbers,
            "set number should enable"
        );
    }
}
