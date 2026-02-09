//! Tests for mark, register, and search actions.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode};

    fn editor_with_text(text: &str) -> EditorState {
        let mut ed = EditorState::new(80, 24);
        ed.open_file("test.txt", text);
        ed
    }

    #[test]
    fn test_set_and_jump_mark() {
        let mut ed = editor_with_text("line1\nline2\nline3\n");
        // Move to line 2
        ed.handle_key(Key::char('j'));
        // Set mark 'a'
        ed.handle_key(Key::char('m'));
        ed.handle_key(Key::char('a'));
        // Move elsewhere
        ed.handle_key(Key::char('j'));
        assert_eq!(ed.windows.focused().cursor.line, 2);
        // Jump back to mark 'a' with backtick
        ed.handle_key(Key::char('`'));
        ed.handle_key(Key::char('a'));
        assert_eq!(ed.windows.focused().cursor.line, 1);
    }

    #[test]
    fn test_jump_mark_line() {
        let mut ed = editor_with_text("  hello\n  world\n");
        ed.handle_key(Key::char('j'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        // Set mark
        ed.handle_key(Key::char('m'));
        ed.handle_key(Key::char('b'));
        // Go back
        ed.handle_key(Key::char('k'));
        // Jump with quote (goes to first non-blank)
        ed.handle_key(Key::char('\''));
        ed.handle_key(Key::char('b'));
        assert_eq!(ed.windows.focused().cursor.line, 1);
        // ' jumps to first non-blank, which is col 2
        assert_eq!(ed.windows.focused().cursor.grapheme, 2);
    }

    #[test]
    fn test_register_selection() {
        let mut ed = editor_with_text("hello\nworld\n");
        // "ayy — yank line into register a
        ed.handle_key(Key::char('"'));
        ed.handle_key(Key::char('a'));
        assert!(ed.pending_register.is_some());
        assert_eq!(ed.pending_register, Some('a'));
    }

    #[test]
    fn test_gg_moves_to_top() {
        let mut ed = editor_with_text("aaa\nbbb\nccc\n");
        ed.handle_key(Key::char('G')); // go to bottom
        assert_eq!(
            ed.windows.focused().cursor.line,
            ed.buffers.current().line_count() - 1
        );
        ed.handle_key(Key::char('g'));
        ed.handle_key(Key::char('g'));
        assert_eq!(ed.windows.focused().cursor.line, 0);
    }

    #[test]
    fn test_search_navigation() {
        let mut ed = editor_with_text("foo bar foo baz\n");
        ed.search.pattern = Some("foo".to_string());
        // n should find first occurrence
        ed.handle_key(Key::char('n'));
        // cursor should be at a 'foo'
        let cursor = ed.windows.focused().cursor;
        assert_eq!(cursor.line, 0);
    }

    #[test]
    fn test_zz_scroll_center() {
        let mut ed = editor_with_text("a\nb\nc\nd\ne\nf\ng\n");
        ed.handle_key(Key::char('z'));
        ed.handle_key(Key::char('z'));
        assert_eq!(ed.mode, Mode::Normal);
    }

    #[test]
    fn test_indent_lines_range() {
        let mut ed = editor_with_text("hello\nworld\n");
        ed.handle_key(Key::char('>'));
        ed.handle_key(Key::char('>'));
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with("    hello"));
    }

    #[test]
    fn test_dedent_lines_range() {
        let mut ed = editor_with_text("    hello\n    world\n");
        ed.handle_key(Key::char('<'));
        ed.handle_key(Key::char('<'));
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with("hello"));
    }
}
