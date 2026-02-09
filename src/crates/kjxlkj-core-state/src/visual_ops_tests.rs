//! Tests for visual mode operations.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode, VisualKind};

    fn editor_with_text(text: &str) -> EditorState {
        let mut ed = EditorState::new(80, 24);
        ed.open_file("test.txt", text);
        ed
    }

    #[test]
    fn test_enter_visual_char() {
        let mut ed = editor_with_text("hello world\n");
        ed.handle_key(Key::char('v'));
        assert!(matches!(ed.mode, Mode::Visual(VisualKind::Char)));
        assert!(ed.visual_anchor.is_some());
    }

    #[test]
    fn test_visual_esc_exits() {
        let mut ed = editor_with_text("hello\n");
        ed.handle_key(Key::char('v'));
        ed.handle_key(Key::esc());
        assert_eq!(ed.mode, Mode::Normal);
        assert!(ed.visual_anchor.is_none());
    }

    #[test]
    fn test_visual_motion_extends() {
        let mut ed = editor_with_text("hello world\n");
        ed.handle_key(Key::char('v'));
        let anchor = ed.visual_anchor.unwrap();
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        let cursor = ed.windows.focused().cursor;
        assert_eq!(anchor.grapheme, 0);
        assert_eq!(cursor.grapheme, 2);
    }

    #[test]
    fn test_visual_d_deletes_selection() {
        let mut ed = editor_with_text("hello world\n");
        ed.handle_key(Key::char('v'));
        // Select 5 chars: h(0) e(1) l(2) l(3) o(4)
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('d'));
        assert_eq!(ed.mode, Mode::Normal);
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with(" world"));
    }

    #[test]
    fn test_visual_y_yanks_selection() {
        let mut ed = editor_with_text("hello world\n");
        ed.handle_key(Key::char('v'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('y'));
        assert_eq!(ed.mode, Mode::Normal);
        let reg = ed.registers.get_unnamed();
        assert!(reg.is_some());
        assert_eq!(reg.unwrap().content, "hello");
    }

    #[test]
    fn test_visual_o_swaps_anchor() {
        let mut ed = editor_with_text("hello world\n");
        ed.handle_key(Key::char('v'));
        ed.handle_key(Key::char('l'));
        ed.handle_key(Key::char('l'));
        let cursor_before = ed.windows.focused().cursor;
        let anchor_before = ed.visual_anchor.unwrap();
        ed.handle_key(Key::char('o'));
        let cursor_after = ed.windows.focused().cursor;
        let anchor_after = ed.visual_anchor.unwrap();
        assert_eq!(cursor_before, anchor_after);
        assert_eq!(anchor_before, cursor_after);
    }

    #[test]
    fn test_visual_line_mode() {
        let mut ed = editor_with_text("aaa\nbbb\nccc\n");
        ed.handle_key(Key::char('V'));
        assert!(matches!(ed.mode, Mode::Visual(VisualKind::Line)));
        ed.handle_key(Key::char('j'));
        ed.handle_key(Key::char('d'));
        assert_eq!(ed.mode, Mode::Normal);
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with("ccc"));
    }

    #[test]
    fn test_visual_switch_kind() {
        let mut ed = editor_with_text("hello\n");
        ed.handle_key(Key::char('v'));
        assert!(matches!(ed.mode, Mode::Visual(VisualKind::Char)));
        ed.handle_key(Key::char('V'));
        assert!(matches!(ed.mode, Mode::Visual(VisualKind::Line)));
    }

    #[test]
    fn test_visual_v_again_exits() {
        let mut ed = editor_with_text("hello\n");
        ed.handle_key(Key::char('v'));
        assert!(matches!(ed.mode, Mode::Visual(VisualKind::Char)));
        ed.handle_key(Key::char('v'));
        assert_eq!(ed.mode, Mode::Normal);
    }
}
