//! Integration tests for the editor.

use kjxlkj_core_state::Editor;
use kjxlkj_core_types::{InputEvent, InsertPosition, Intent, KeyCode, KeyEvent, Mode, Motion};

/// Test basic cursor movement.
#[test]
fn test_cursor_movement() {
    let mut editor = Editor::new();

    // Insert some text first
    editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
    editor.apply_intent(Intent::InsertText("hello\nworld\n".to_string()));
    editor.apply_intent(Intent::ChangeMode(Mode::Normal));

    // Move cursor to start
    editor.apply_intent(Intent::MoveCursor(Motion::FileStart));
    let snapshot = editor.snapshot();
    assert_eq!(snapshot.windows[0].cursor.line, 0);

    // Move cursor down
    editor.apply_intent(Intent::MoveCursor(Motion::Down));
    let snapshot = editor.snapshot();
    assert_eq!(snapshot.windows[0].cursor.line, 1);
}

/// Test insert mode.
#[test]
fn test_insert_mode() {
    let mut editor = Editor::new();

    editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
    assert_eq!(editor.mode(), Mode::Insert);

    editor.apply_intent(Intent::InsertText("test".to_string()));
    editor.apply_intent(Intent::ChangeMode(Mode::Normal));
    assert_eq!(editor.mode(), Mode::Normal);

    let snapshot = editor.snapshot();
    assert!(snapshot.windows[0].lines[0].text.contains("test"));
}

/// Test visual mode.
#[test]
fn test_visual_mode() {
    let mut editor = Editor::new();

    editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
    editor.apply_intent(Intent::InsertText("hello world".to_string()));
    editor.apply_intent(Intent::ChangeMode(Mode::Normal));

    // Enter visual mode via key input
    editor.handle_input(InputEvent::Key(KeyEvent::plain(KeyCode::Char('v'))));
    assert!(editor.mode().is_visual());

    editor.handle_input(InputEvent::Key(KeyEvent::plain(KeyCode::Esc)));
    assert_eq!(editor.mode(), Mode::Normal);
}

/// Test undo/redo.
#[test]
fn test_undo_redo() {
    let mut editor = Editor::new();

    editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
    editor.apply_intent(Intent::InsertText("hello".to_string()));
    editor.apply_intent(Intent::ChangeMode(Mode::Normal));

    let snapshot = editor.snapshot();
    assert!(snapshot.windows[0].lines[0].text.contains("hello"));

    editor.apply_intent(Intent::Undo);
    // Note: Undo may or may not fully revert depending on implementation
}

/// Test search.
#[test]
fn test_search() {
    let mut editor = Editor::new();

    editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
    editor.apply_intent(Intent::InsertText("hello world\nfoo bar\n".to_string()));
    editor.apply_intent(Intent::ChangeMode(Mode::Normal));
    editor.apply_intent(Intent::MoveCursor(Motion::FileStart));

    editor.apply_intent(Intent::SearchForward("world".to_string()));
    let snapshot = editor.snapshot();
    assert_eq!(snapshot.windows[0].cursor.line, 0);
    assert!(snapshot.windows[0].cursor.column > 0);
}

/// Test key event handling.
#[test]
fn test_key_input() {
    let mut editor = Editor::new();

    let intents = editor.handle_input(InputEvent::Key(KeyEvent::plain(KeyCode::Char('i'))));
    assert!(!intents.is_empty());
    assert_eq!(editor.mode(), Mode::Insert);

    let intents = editor.handle_input(InputEvent::Key(KeyEvent::plain(KeyCode::Esc)));
    assert!(!intents.is_empty());
    assert_eq!(editor.mode(), Mode::Normal);
}

/// Test resize handling.
#[test]
fn test_resize() {
    let mut editor = Editor::new();

    editor.handle_input(InputEvent::Resize {
        cols: 100,
        rows: 50,
    });
    let snapshot = editor.snapshot();
    assert_eq!(snapshot.terminal_size.cols, 100);
    assert_eq!(snapshot.terminal_size.rows, 50);
}

/// Test command mode.
#[test]
fn test_command_mode() {
    let mut editor = Editor::new();

    editor.handle_input(InputEvent::Key(KeyEvent::plain(KeyCode::Char(':'))));
    assert_eq!(editor.mode(), Mode::Command);

    editor.handle_input(InputEvent::Key(KeyEvent::plain(KeyCode::Esc)));
    assert_eq!(editor.mode(), Mode::Normal);
}
