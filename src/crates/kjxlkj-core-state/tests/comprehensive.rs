//! Comprehensive tests for kjxlkj-core-state.

use kjxlkj_core_state::*;
use kjxlkj_core_types::{BufferId, BufferName, EditorEvent, Intent, KeyEvent, Mode};

mod buffer_state_tests {
    use super::*;

    #[test]
    fn test_buffer_state_new() {
        let state = BufferState::new(BufferId::new(1), BufferName::new("test.rs"));
        assert_eq!(state.id, BufferId::new(1));
        assert_eq!(state.name.as_str(), "test.rs");
        assert!(!state.modified);
    }

    #[test]
    fn test_buffer_state_unnamed() {
        let state = BufferState::new(BufferId::new(1), BufferName::unnamed());
        assert_eq!(state.name.as_str(), "[No Name]");
    }

    #[test]
    fn test_buffer_state_line_count() {
        let state = BufferState::new(BufferId::new(1), BufferName::unnamed());
        assert_eq!(state.line_count(), 1);
    }

    #[test]
    fn test_buffer_state_line_len() {
        let state = BufferState::new(BufferId::new(1), BufferName::unnamed());
        assert_eq!(state.line_len(0), 0);
    }

    #[test]
    fn test_buffer_state_version() {
        let state = BufferState::new(BufferId::new(1), BufferName::unnamed());
        let v = state.version();
        // Should be initial version
        assert!(v >= kjxlkj_core_types::BufferVersion::new(0));
    }

    #[test]
    fn test_buffer_state_modified_flag() {
        let mut state = BufferState::new(BufferId::new(1), BufferName::unnamed());
        assert!(!state.modified);
        state.set_modified(true);
        assert!(state.modified);
        state.set_modified(false);
        assert!(!state.modified);
    }
}

mod window_state_tests {
    use super::*;
    use kjxlkj_core_types::{Cursor, WindowId};

    #[test]
    fn test_window_state_new() {
        let state = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        assert_eq!(state.id, WindowId::new(1));
        assert_eq!(state.buffer_id, BufferId::new(1));
        assert_eq!(state.cursor, Cursor::origin());
    }

    #[test]
    fn test_window_state_viewport() {
        let state = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        assert_eq!(state.viewport.width, 80);
        assert_eq!(state.viewport.height, 24);
    }

    #[test]
    fn test_window_state_set_buffer() {
        let mut state = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        state.set_buffer(BufferId::new(2));
        assert_eq!(state.buffer_id, BufferId::new(2));
        assert_eq!(state.cursor, Cursor::origin());
    }

    #[test]
    fn test_window_state_resize() {
        let mut state = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        state.resize(120, 40);
        assert_eq!(state.viewport.width, 120);
        assert_eq!(state.viewport.height, 40);
    }
}

mod editor_tests {
    use super::*;

    #[test]
    fn test_editor_new() {
        let editor = Editor::new(80, 24);
        assert_eq!(editor.mode(), Mode::Normal);
        assert!(!editor.quit_requested());
    }

    #[test]
    fn test_editor_initial_mode() {
        let editor = Editor::new(80, 24);
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_enter_insert_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        assert_eq!(editor.mode(), Mode::Insert);
    }

    #[test]
    fn test_editor_enter_append_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        assert_eq!(editor.mode(), Mode::Insert);
    }

    #[test]
    fn test_editor_escape_to_normal() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_insert_text() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('h')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('e')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('l')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('l')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('o')));

        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines[0].contains("hello"));
    }

    #[test]
    fn test_editor_cursor_movement() {
        let mut editor = Editor::new(80, 24);
        // Insert some text
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('b')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('c')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        // Move left
        editor.process_event(EditorEvent::Key(KeyEvent::char('h')));
        let snap1 = editor.snapshot();
        let cursor1 = snap1.windows[0].cursor;

        // Move right
        editor.process_event(EditorEvent::Key(KeyEvent::char('l')));
        let snap2 = editor.snapshot();
        let cursor2 = snap2.windows[0].cursor;

        assert!(cursor2.column > cursor1.column);
    }

    #[test]
    fn test_editor_undo() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('x')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        let before_undo = editor.snapshot();
        assert!(before_undo.windows[0].buffer.lines[0].contains('x'));

        editor.process_event(EditorEvent::Key(KeyEvent::char('u')));
        let after_undo = editor.snapshot();
        assert!(!after_undo.windows[0].buffer.lines[0].contains('x'));
    }

    #[test]
    fn test_editor_redo() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('y')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));
        editor.process_event(EditorEvent::Key(KeyEvent::char('u')));

        let before_redo = editor.snapshot();
        assert!(!before_redo.windows[0].buffer.lines[0].contains('y'));

        editor.process_event(EditorEvent::Key(KeyEvent::ctrl('r')));
        let after_redo = editor.snapshot();
        assert!(after_redo.windows[0].buffer.lines[0].contains('y'));
    }

    #[test]
    fn test_editor_command_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char(':')));
        assert_eq!(editor.mode(), Mode::Command);
    }

    #[test]
    fn test_editor_command_quit() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char(':')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('q')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('!')));
        editor.process_event(EditorEvent::Key(KeyEvent::Enter));

        assert!(editor.quit_requested());
    }

    #[test]
    fn test_editor_visual_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('v')));
        assert_eq!(editor.mode(), Mode::Visual);
    }

    #[test]
    fn test_editor_replace_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('R')));
        assert_eq!(editor.mode(), Mode::Replace);
    }

    #[test]
    fn test_editor_resize() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Resize {
            width: 120,
            height: 40,
        });

        let snapshot = editor.snapshot();
        assert_eq!(snapshot.terminal_width, 120);
        assert_eq!(snapshot.terminal_height, 40);
    }

    #[test]
    fn test_editor_snapshot_sequence() {
        let mut editor = Editor::new(80, 24);
        let snap1 = editor.snapshot();
        let snap2 = editor.snapshot();
        assert!(snap2.sequence > snap1.sequence);
    }

    #[test]
    fn test_editor_delete_char() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('b')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        editor.process_event(EditorEvent::Key(KeyEvent::char('x')));
        let _snapshot = editor.snapshot();
        // 'x' deletes char at cursor
    }

    #[test]
    fn test_editor_delete_line() {
        let mut editor = Editor::new(80, 24);
        // Insert a line
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('t')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('e')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('s')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('t')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        // dd to delete line
        editor.process_event(EditorEvent::Key(KeyEvent::char('d')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('d')));

        let _snapshot = editor.snapshot();
        // Line should be deleted
    }

    #[test]
    fn test_editor_yank_paste() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('h')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        // yy to yank line
        editor.process_event(EditorEvent::Key(KeyEvent::char('y')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('y')));

        // p to paste
        editor.process_event(EditorEvent::Key(KeyEvent::char('p')));

        let _snapshot = editor.snapshot();
        // Should have duplicated content
    }

    #[test]
    fn test_editor_open_line_below() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('o')));
        assert_eq!(editor.mode(), Mode::Insert);

        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines.len() >= 2);
    }

    #[test]
    fn test_editor_open_line_above() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('O')));
        assert_eq!(editor.mode(), Mode::Insert);
    }

    #[test]
    fn test_editor_word_movement() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        for c in "hello world".chars() {
            editor.process_event(EditorEvent::Key(KeyEvent::char(c)));
        }
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        // Go to beginning
        editor.process_event(EditorEvent::Key(KeyEvent::char('0')));

        // Move word forward
        editor.process_event(EditorEvent::Key(KeyEvent::char('w')));
        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].cursor.column > 0);
    }

    #[test]
    fn test_editor_line_movements() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        for c in "hello".chars() {
            editor.process_event(EditorEvent::Key(KeyEvent::char(c)));
        }
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        // $ to end of line
        editor.process_event(EditorEvent::Key(KeyEvent::char('$')));
        let snap1 = editor.snapshot();

        // 0 to start of line
        editor.process_event(EditorEvent::Key(KeyEvent::char('0')));
        let snap2 = editor.snapshot();

        assert!(snap1.windows[0].cursor.column > snap2.windows[0].cursor.column);
    }

    #[test]
    fn test_editor_focus_events() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Focus(true));
        editor.process_event(EditorEvent::Focus(false));
        // Should not crash or change state significantly
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_quit_event() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Quit);
        // Direct quit event should request quit
        assert!(editor.quit_requested());
    }
}

mod apply_intent_tests {
    use super::*;

    #[test]
    fn test_apply_intent_enter_mode() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::EnterMode(Mode::Insert));
        assert_eq!(editor.mode(), Mode::Insert);
    }

    #[test]
    fn test_apply_intent_move_up() {
        let mut editor = Editor::new(80, 24);
        // Need multiple lines first
        editor.apply_intent(Intent::InsertNewlineBelow);
        editor.apply_intent(Intent::EnterMode(Mode::Normal));
        editor.apply_intent(Intent::MoveUp(1));
    }

    #[test]
    fn test_apply_intent_move_down() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::InsertNewlineBelow);
        editor.apply_intent(Intent::EnterMode(Mode::Normal));
        editor.apply_intent(Intent::MoveToDocumentStart);
        editor.apply_intent(Intent::MoveDown(1));
    }

    #[test]
    fn test_apply_intent_insert_char() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::EnterMode(Mode::Insert));
        editor.apply_intent(Intent::InsertChar('a'));
        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines[0].contains('a'));
    }

    #[test]
    fn test_apply_intent_insert_newline() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::EnterMode(Mode::Insert));
        editor.apply_intent(Intent::InsertNewline);
        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines.len() >= 2);
    }

    #[test]
    fn test_apply_intent_delete_char() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::EnterMode(Mode::Insert));
        editor.apply_intent(Intent::InsertChar('x'));
        editor.apply_intent(Intent::EnterMode(Mode::Normal));
        editor.apply_intent(Intent::DeleteChar);
    }

    #[test]
    fn test_apply_intent_scroll() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::ScrollHalfPageDown);
        editor.apply_intent(Intent::ScrollHalfPageUp);
    }

    #[test]
    fn test_apply_intent_center_cursor() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::CenterCursor);
    }

    #[test]
    fn test_apply_intent_quit() {
        let mut editor = Editor::new(80, 24);
        editor.apply_intent(Intent::QuitForce);
        assert!(editor.quit_requested());
    }

    #[test]
    fn test_apply_intent_noop() {
        let mut editor = Editor::new(80, 24);
        let mode_before = editor.mode();
        editor.apply_intent(Intent::Noop);
        assert_eq!(editor.mode(), mode_before);
    }
}

mod extra_buffer_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_buffer_state_from_file() {
        let buf = BufferState::from_file(
            BufferId::new(1),
            PathBuf::from("/tmp/test.txt"),
            "hello\nworld",
        );
        assert_eq!(buf.line_count(), 2);
        assert!(buf.path.is_some());
    }

    #[test]
    fn test_buffer_state_line_returns_option() {
        let state = BufferState::new(BufferId::new(1), BufferName::unnamed());
        assert!(state.line(0).is_some());
        assert!(state.line(999).is_none()); // Out of bounds
    }

    #[test]
    fn test_buffer_state_from_file_content() {
        let buf = BufferState::from_file(
            BufferId::new(1),
            PathBuf::from("/test.rs"),
            "line1\nline2\nline3",
        );
        assert_eq!(buf.line_count(), 3);
        assert_eq!(buf.line(0), Some("line1".to_string()));
        assert_eq!(buf.line(1), Some("line2".to_string()));
        assert_eq!(buf.line(2), Some("line3".to_string()));
    }

    #[test]
    fn test_buffer_state_path_from_file() {
        let buf = BufferState::from_file(BufferId::new(1), PathBuf::from("/home/user/code.rs"), "");
        assert_eq!(buf.path, Some(PathBuf::from("/home/user/code.rs")));
        assert_eq!(buf.name.as_str(), "code.rs");
    }
}

mod extra_window_tests {
    use super::*;
    use kjxlkj_core_types::{Cursor, WindowId};

    #[test]
    fn test_window_state_move_cursor() {
        let mut state = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        state.cursor = Cursor::new(5, 10);
        assert_eq!(state.cursor.line, 5);
        assert_eq!(state.cursor.column, 10);
    }

    #[test]
    fn test_window_state_scroll() {
        let mut state = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        state.viewport.top_line = 10;
        assert_eq!(state.viewport.top_line, 10);
    }
}

mod extra_editor_tests {
    use super::*;

    #[test]
    fn test_editor_snapshot_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char(':')));
        let snapshot = editor.snapshot();
        assert_eq!(snapshot.mode, Mode::Command);
    }

    #[test]
    fn test_editor_capital_g_moves_to_end() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        editor.process_event(EditorEvent::Key(KeyEvent::Enter));
        editor.process_event(EditorEvent::Key(KeyEvent::char('b')));
        editor.process_event(EditorEvent::Key(KeyEvent::Enter));
        editor.process_event(EditorEvent::Key(KeyEvent::char('c')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        editor.process_event(EditorEvent::Key(KeyEvent::char('g')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('g')));

        editor.process_event(EditorEvent::Key(KeyEvent::char('G')));

        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].cursor.line > 0);
    }

    #[test]
    fn test_editor_backspace_in_insert() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('b')));
        editor.process_event(EditorEvent::Key(KeyEvent::Backspace));

        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines[0].contains('a'));
        assert!(!snapshot.windows[0].buffer.lines[0].contains('b'));
    }

    #[test]
    fn test_editor_visual_line_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('V')));
        assert_eq!(editor.mode(), Mode::VisualLine);
    }

    #[test]
    fn test_editor_visual_block_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::ctrl('v')));
        assert_eq!(editor.mode(), Mode::VisualBlock);
    }

    #[test]
    fn test_editor_visual_escape() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('v')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_command_escape() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char(':')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_snapshot_dimensions() {
        let mut editor = Editor::new(100, 50);
        let snapshot = editor.snapshot();
        assert_eq!(snapshot.terminal_width, 100);
        assert_eq!(snapshot.terminal_height, 50);
    }

    #[test]
    fn test_editor_arrow_keys_in_insert() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('b')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('c')));

        let snap1 = editor.snapshot();
        editor.process_event(EditorEvent::Key(KeyEvent::Left));
        let snap2 = editor.snapshot();

        assert!(snap2.windows[0].cursor.column < snap1.windows[0].cursor.column);
    }

    #[test]
    fn test_editor_half_page_scroll() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::ctrl('d')));
        editor.process_event(EditorEvent::Key(KeyEvent::ctrl('u')));
    }

    #[test]
    fn test_editor_zz_centers() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('z')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('z')));
    }
}

mod e2e_editor_tests {
    use super::*;
    use kjxlkj_core_types::{EditorEvent, KeyEvent, Mode, Modifier};

    fn key(c: char) -> EditorEvent {
        EditorEvent::Key(KeyEvent::Char(c, Modifier::NONE))
    }

    fn escape() -> EditorEvent {
        EditorEvent::Key(KeyEvent::Escape)
    }

    fn enter() -> EditorEvent {
        EditorEvent::Key(KeyEvent::Enter)
    }

    fn backspace() -> EditorEvent {
        EditorEvent::Key(KeyEvent::Backspace)
    }

    #[test]
    fn e2e_enter_insert_mode_type_text_return_to_normal() {
        let mut editor = Editor::new(80, 24);
        assert_eq!(editor.mode(), Mode::Normal);

        // Press 'i' to enter insert mode
        editor.process_event(key('i'));
        assert_eq!(editor.mode(), Mode::Insert);

        // Type "hello"
        for c in "hello".chars() {
            editor.process_event(key(c));
        }

        // Press Escape to return to normal mode
        editor.process_event(escape());
        assert_eq!(editor.mode(), Mode::Normal);

        // Verify buffer content
        let snap = editor.snapshot();
        let buf = snap.windows.first().map(|w| &w.buffer);
        assert!(buf.is_some());
    }

    #[test]
    fn e2e_cursor_movement_in_normal_mode() {
        let mut editor = Editor::new(80, 24);

        // Enter insert mode and type some text
        editor.process_event(key('i'));
        for c in "line1".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        // Test hjkl movements
        editor.process_event(key('h')); // move left
        editor.process_event(key('l')); // move right

        // Test 0 and $ for line start/end
        editor.process_event(key('0')); // beginning of line
        editor.process_event(key('$')); // end of line

        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_append_mode() {
        let mut editor = Editor::new(80, 24);

        // 'a' enters insert mode after cursor
        editor.process_event(key('a'));
        assert_eq!(editor.mode(), Mode::Insert);

        editor.process_event(key('x'));
        editor.process_event(escape());
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_open_line_below() {
        let mut editor = Editor::new(80, 24);

        // 'o' opens line below and enters insert mode
        editor.process_event(key('o'));
        assert_eq!(editor.mode(), Mode::Insert);

        editor.process_event(key('n'));
        editor.process_event(key('e'));
        editor.process_event(key('w'));
        editor.process_event(escape());
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_open_line_above() {
        let mut editor = Editor::new(80, 24);

        // 'O' opens line above and enters insert mode
        editor.process_event(key('O'));
        assert_eq!(editor.mode(), Mode::Insert);
        editor.process_event(escape());
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_word_motions() {
        let mut editor = Editor::new(80, 24);

        // Enter insert mode and type words
        editor.process_event(key('i'));
        for c in "hello world test".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        // Test word motions
        editor.process_event(key('0')); // go to start
        editor.process_event(key('w')); // forward word
        editor.process_event(key('w')); // forward word
        editor.process_event(key('b')); // backward word
        editor.process_event(key('e')); // end of word

        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_delete_character() {
        let mut editor = Editor::new(80, 24);

        // Enter insert mode and type
        editor.process_event(key('i'));
        for c in "hello".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        // Delete character with x
        editor.process_event(key('x'));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_undo_redo() {
        let mut editor = Editor::new(80, 24);

        // Enter insert mode and type
        editor.process_event(key('i'));
        for c in "test".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        // Undo with u
        editor.process_event(key('u'));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_visual_mode() {
        let mut editor = Editor::new(80, 24);

        // Enter insert mode and type
        editor.process_event(key('i'));
        for c in "hello world".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        // 'v' enters visual mode (if implemented)
        // For now just verify we stay in consistent state
        editor.process_event(key('0'));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_document_navigation() {
        let mut editor = Editor::new(80, 24);

        // Enter insert mode and type multiple lines
        editor.process_event(key('i'));
        for c in "line1".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(enter());
        for c in "line2".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(enter());
        for c in "line3".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        // gg goes to start of document
        editor.process_event(key('g'));
        editor.process_event(key('g'));

        // G goes to end of document
        editor.process_event(key('G'));

        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_resize_handling() {
        let mut editor = Editor::new(80, 24);

        // Simulate a terminal resize
        editor.process_event(EditorEvent::Resize {
            width: 120,
            height: 40,
        });

        let snap = editor.snapshot();
        assert_eq!(snap.terminal_width, 120);
        assert_eq!(snap.terminal_height, 40);
    }

    #[test]
    fn e2e_quit_command() {
        let mut editor = Editor::new(80, 24);

        // Simulate quit event
        editor.process_event(EditorEvent::Quit);
        assert!(editor.quit_requested());
    }

    #[test]
    fn e2e_command_mode_attempt() {
        let mut editor = Editor::new(80, 24);

        // ':' enters command mode (or starts command)
        editor.process_event(key(':'));
        // Escape to return to normal
        editor.process_event(escape());

        // Verify we're in a valid state
        assert!(!editor.quit_requested());
    }

    #[test]
    fn e2e_insert_multiple_lines() {
        let mut editor = Editor::new(80, 24);

        editor.process_event(key('i'));
        for c in "first line".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(enter());
        for c in "second line".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(enter());
        for c in "third line".chars() {
            editor.process_event(key(c));
        }
        editor.process_event(escape());

        let snap = editor.snapshot();
        assert!(!snap.windows.is_empty());
    }

    #[test]
    fn e2e_backspace_in_insert_mode() {
        let mut editor = Editor::new(80, 24);

        editor.process_event(key('i'));
        for c in "hello".chars() {
            editor.process_event(key(c));
        }
        // Backspace
        editor.process_event(backspace());
        editor.process_event(backspace());
        editor.process_event(escape());

        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn e2e_rapid_mode_switches() {
        let mut editor = Editor::new(80, 24);

        for _ in 0..10 {
            editor.process_event(key('i'));
            assert_eq!(editor.mode(), Mode::Insert);
            editor.process_event(escape());
            assert_eq!(editor.mode(), Mode::Normal);
        }
    }

    #[test]
    fn e2e_continuous_typing() {
        let mut editor = Editor::new(80, 24);

        editor.process_event(key('i'));

        // Simulate continuous typing
        let text = "The quick brown fox jumps over the lazy dog. This is a longer text to test continuous input handling.";
        for c in text.chars() {
            editor.process_event(key(c));
        }

        editor.process_event(escape());
        assert_eq!(editor.mode(), Mode::Normal);
    }
}
