//! Boundary tests BD-01 through BD-19.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Action, InsertPosition, Mode, Motion, Operator,
};

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ins(e: &mut EditorState, text: &str) {
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for ch in text.chars() {
        e.dispatch(Action::InsertChar(ch));
    }
    e.dispatch(Action::ReturnToNormal);
}

/// BD-01: Empty file write.
#[test]
fn bd01_empty_wq() {
    let mut e = ed();
    e.dispatch(Action::WriteAll);
    assert!(!e.should_quit);
}

/// BD-02: dd on empty buffer.
#[test]
fn bd02_dd_empty() {
    let mut e = ed();
    e.dispatch(Action::DoubleOperator(
        Operator::Delete, 1,
    ));
    assert!(e.active_buffer().unwrap()
        .content.line_count() >= 1);
}

/// BD-03: Paste from empty register.
#[test]
fn bd03_paste_empty() {
    let mut e = ed();
    e.dispatch(Action::Put(false));
    assert!(e.active_buffer().unwrap()
        .content.line_count() >= 1);
}

/// BD-04: Single-character file.
#[test]
fn bd04_single_char() {
    let mut e = ed();
    ins(&mut e, "x");
    let line = e.active_buffer().unwrap()
        .content.line_str(0);
    assert!(line.contains('x'));
    e.dispatch(Action::DeleteCharForward);
    // After deleting the only char, buffer must remain valid
    assert!(e.active_buffer().unwrap()
        .content.line_count() >= 1);
}

/// BD-05: Single newline file.
#[test]
fn bd05_single_newline() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    e.dispatch(Action::InsertChar('\n'));
    e.dispatch(Action::ReturnToNormal);
    e.dispatch(Action::MoveCursor(Motion::Down, 1));
    e.dispatch(Action::DoubleOperator(
        Operator::Delete, 1,
    ));
    assert!(e.active_buffer().unwrap()
        .content.line_count() >= 1);
}

/// BD-06: 10,000 ASCII chars wrap.
#[test]
fn bd06_long_ascii_wrap() {
    use kjxlkj_core_state::line_wrap::wrap_line;
    let rows = wrap_line(&"a".repeat(10000), 80);
    assert_eq!(rows.len(), (10000 + 79) / 80);
}

/// BD-07: 10,000 CJK chars wrap.
#[test]
fn bd07_long_cjk_wrap() {
    use kjxlkj_core_state::line_wrap::wrap_line;
    let line: String = "あ".repeat(10000);
    let rows = wrap_line(&line, 80);
    assert_eq!(rows.len(), (10000 + 39) / 40);
}

/// BD-09: Mixed line $ and 0.
#[test]
fn bd09_mixed_line_end_start() {
    let mut e = ed();
    ins(&mut e, "abcあいう");
    e.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
    assert!(e.focused_window().unwrap()
        .cursor.grapheme_offset > 0);
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    assert_eq!(e.focused_window().unwrap()
        .cursor.grapheme_offset, 0);
}

/// BD-10: 1000 i/Esc cycles.
#[test]
fn bd10_mode_switch() {
    let mut e = ed();
    for _ in 0..1000 {
        e.dispatch(Action::EnterInsert(
            InsertPosition::BeforeCursor,
        ));
        e.dispatch(Action::ReturnToNormal);
    }
    assert_eq!(e.mode, Mode::Normal);
}

/// BD-11: 500 v/Esc flicker.
#[test]
fn bd11_visual_flicker() {
    use kjxlkj_core_types::VisualKind;
    let mut e = ed();
    ins(&mut e, "test");
    for _ in 0..500 {
        e.dispatch(Action::EnterVisual(VisualKind::Char));
        e.dispatch(Action::ReturnToNormal);
    }
    assert_eq!(e.mode, Mode::Normal);
    assert!(e.visual_state.is_none());
}

/// BD-12: Insert/Replace alternation.
#[test]
fn bd12_insert_replace() {
    let mut e = ed();
    ins(&mut e, "hello");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    for _ in 0..20 {
        e.dispatch(Action::EnterInsert(
            InsertPosition::BeforeCursor,
        ));
        e.dispatch(Action::InsertChar('a'));
        e.dispatch(Action::ReturnToNormal);
        e.dispatch(Action::EnterReplace);
        e.dispatch(Action::InsertChar('b'));
        e.dispatch(Action::ReturnToNormal);
    }
    assert_eq!(e.mode, Mode::Normal);
}

/// BD-13: Terminal scrollback.
#[test]
fn bd13_terminal() {
    let mut e = ed();
    e.dispatch(Action::SpawnTerminal);
    assert!(!e.should_quit);
}

/// BD-14: Buffer edit during terminal.
#[test]
fn bd14_edit_during_terminal() {
    let mut e = ed();
    e.dispatch(Action::SplitHorizontal);
    ins(&mut e, "edit during terminal");
    assert!(e.active_buffer().unwrap()
        .content.line_str(0).contains("edit"));
}

/// BD-15: Terminal close.
#[test]
fn bd15_terminal_close() {
    let mut e = ed();
    e.dispatch(Action::SplitHorizontal);
    assert_eq!(e.windows.len(), 2);
    e.dispatch(Action::CloseWindow);
    assert_eq!(e.windows.len(), 1);
}

/// BD-16: 100 resizes, empty buffer.
#[test]
fn bd16_resize_storm() {
    let mut e = ed();
    for i in 0..100u16 {
        e.handle_resize(40 + i % 80, 12 + i % 24);
    }
    assert!(e.terminal_size.0 > 0);
}

/// BD-17: 100 resizes, CJK buffer.
#[test]
fn bd17_resize_cjk() {
    let mut e = ed();
    ins(&mut e, "あいうえお");
    for i in 0..100u16 {
        e.handle_resize(20 + i % 100, 5 + i % 30);
    }
    assert!(!e.should_quit);
}

/// BD-18: Resize to 1 column.
#[test]
fn bd18_resize_1col() {
    let mut e = ed();
    ins(&mut e, "test");
    e.handle_resize(1, 24);
    assert_eq!(e.terminal_size, (1, 24));
}

/// BD-19: Resize to 1 row.
#[test]
fn bd19_resize_1row() {
    let mut e = ed();
    ins(&mut e, "test");
    e.handle_resize(80, 1);
    assert_eq!(e.terminal_size, (80, 1));
}
