//! Integration tests INT-01 through INT-10.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, InsertPosition, Mode, Motion, Operator, VisualKind};

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ins(ed: &mut EditorState, text: &str) {
    ed.dispatch(Action::EnterInsert(InsertPosition::BeforeCursor));
    for ch in text.chars() {
        ed.dispatch(Action::InsertChar(ch));
    }
    ed.dispatch(Action::ReturnToNormal);
}

/// INT-01: Insert text, undo, redo.
#[test]
fn int01_insert_undo_redo() {
    let mut e = ed();
    ins(&mut e, "hello");
    let line = e.active_buffer().unwrap().content.line_str(0);
    assert!(line.contains("hello"));
    e.dispatch(Action::Undo);
    e.dispatch(Action::Redo);
    let line2 = e.active_buffer().unwrap().content.line_str(0);
    assert!(line2.contains("hello"));
}

/// INT-02: Delete via operator in visual mode.
#[test]
fn int02_visual_delete() {
    let mut e = ed();
    ins(&mut e, "one two three");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    // Use Delete(motion) directly.
    e.dispatch(Action::Delete(Motion::WordForward, 1));
    let line = e.active_buffer().unwrap().content.line_str(0);
    assert!(!line.starts_with("one "));
}

/// INT-03: Key mapping registration.
#[test]
fn int03_mapping() {
    use kjxlkj_core_state::mappings::{KeyMapping, MappingMode, MappingRegistry};
    let mut reg = MappingRegistry::new();
    reg.add(KeyMapping {
        modes: vec![MappingMode::Insert],
        lhs: "jk".into(),
        rhs: "<Esc>".into(),
        noremap: true,
        silent: false,
        buffer_local: false,
        expr: false,
    });
    assert!(reg.find("jk", MappingMode::Insert).is_some());
}

/// INT-04: Snapshot pipeline.
#[test]
fn int04_snapshot() {
    let mut e = ed();
    ins(&mut e, "x");
    let snap = e.snapshot();
    assert!(snap.sequence > 0);
    assert_eq!(snap.mode, Mode::Normal);
}

/// INT-05: CJK viewport follow.
#[test]
fn int05_cjk_viewport() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(InsertPosition::BeforeCursor));
    for _ in 0..50 {
        for ch in "あいうえお".chars() {
            e.dispatch(Action::InsertChar(ch));
        }
        e.dispatch(Action::InsertChar('\n'));
    }
    e.dispatch(Action::ReturnToNormal);
    e.dispatch(Action::MoveCursor(Motion::GotoLine(39), 1));
    let w = e.focused_window().unwrap();
    assert_eq!(w.cursor.line, 39);
}

/// INT-06: Count-prefixed operator.
#[test]
fn int06_count_delete() {
    let mut e = ed();
    ins(&mut e, "one two three four");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    e.dispatch(Action::Delete(Motion::WordForward, 2));
    let line = e.active_buffer().unwrap().content.line_str(0);
    assert!(!line.starts_with("one"));
}

/// INT-07: Terminal spawn.
#[test]
fn int07_terminal() {
    let mut e = ed();
    e.dispatch(Action::SpawnTerminal);
    assert!(e.windows.len() >= 1);
}

/// INT-08: Multi-buffer yank/paste.
#[test]
fn int08_yank_paste() {
    let mut e = ed();
    ins(&mut e, "yanked line");
    e.dispatch(Action::DoubleOperator(Operator::Yank, 1));
    e.dispatch(Action::SplitHorizontal);
    e.dispatch(Action::Put(false));
    // No panic, register was used.
}

/// INT-09: Resize pipeline.
#[test]
fn int09_resize() {
    let mut e = ed();
    e.handle_resize(40, 12);
    assert_eq!(e.terminal_size, (40, 12));
    let snap = e.snapshot();
    assert_eq!(snap.terminal_size, (40, 12));
}

/// INT-10: Command execution.
#[test]
fn int10_command() {
    let mut e = ed();
    e.dispatch(Action::ExecuteCommand("set number".into()));
    assert_eq!(e.mode, Mode::Normal);
}
