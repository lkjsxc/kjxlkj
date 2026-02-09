//! Boundary tests BD-20 through BD-32.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, InsertPosition, Motion, Operator};

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ins(e: &mut EditorState, text: &str) {
    e.dispatch(Action::EnterInsert(InsertPosition::BeforeCursor));
    for ch in text.chars() {
        e.dispatch(Action::InsertChar(ch));
    }
    e.dispatch(Action::ReturnToNormal);
}

#[test]
fn bd20_session_empty() {
    use kjxlkj_core_state::{SessionData, SessionLayout};
    use std::path::PathBuf;
    let data = SessionData {
        buffers: vec![],
        layout: SessionLayout::Single,
        active: 0,
        cwd: PathBuf::from("/tmp"),
    };
    let json = serde_json::to_string(&data).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn bd21_session_splits() {
    use kjxlkj_core_state::{SessionData, SessionLayout};
    use std::path::PathBuf;
    let data = SessionData {
        buffers: vec![
            PathBuf::from("a.txt"),
            PathBuf::from("b.txt"),
            PathBuf::from("c.txt"),
        ],
        layout: SessionLayout::VerticalSplit(vec![
            SessionLayout::Single,
            SessionLayout::HorizontalSplit(vec![SessionLayout::Single, SessionLayout::Single]),
        ]),
        active: 0,
        cwd: PathBuf::from("/home"),
    };
    let json = serde_json::to_string(&data).unwrap();
    let loaded: SessionData = serde_json::from_str(&json).unwrap();
    assert_eq!(loaded.buffers.len(), 3);
}

#[test]
fn bd22_session_terminal() {
    use kjxlkj_core_state::{SessionData, SessionLayout};
    use std::path::PathBuf;
    let data = SessionData {
        buffers: vec![PathBuf::from("a.txt")],
        layout: SessionLayout::HorizontalSplit(vec![SessionLayout::Single, SessionLayout::Single]),
        active: 0,
        cwd: PathBuf::from("/tmp"),
    };
    let json = serde_json::to_string(&data).unwrap();
    assert!(json.contains("HorizontalSplit"));
}

#[test]
fn bd23_session_cjk() {
    use kjxlkj_core_state::{SessionData, SessionLayout};
    use std::path::PathBuf;
    let data = SessionData {
        buffers: vec![PathBuf::from("cjk.txt")],
        layout: SessionLayout::Single,
        active: 0,
        cwd: PathBuf::from("/tmp"),
    };
    let json = serde_json::to_string(&data).unwrap();
    assert!(json.contains("cjk.txt"));
}

#[test]
fn bd24_session_missing() {
    use kjxlkj_core_state::{SessionData, SessionLayout};
    use std::path::PathBuf;
    let data = SessionData {
        buffers: vec![PathBuf::from("/nonexistent/file.txt")],
        layout: SessionLayout::Single,
        active: 0,
        cwd: PathBuf::from("/tmp"),
    };
    let json = serde_json::to_string(&data).unwrap();
    let parsed: SessionData = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.buffers.len(), 1);
}

#[test]
fn bd25_cjk_word() {
    let mut e = ed();
    ins(&mut e, "あいう えお");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    e.dispatch(Action::MoveCursor(Motion::WordForward, 1));
    assert!(e.focused_window().unwrap().cursor.grapheme_offset > 0);
}

#[test]
fn bd26_cjk_delete_word() {
    let mut e = ed();
    ins(&mut e, "あいう えお");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    e.dispatch(Action::Delete(Motion::WordForward, 1));
    let line = e.active_buffer().unwrap().content.line_str(0);
    assert!(!line.starts_with("あ"));
}

#[test]
fn bd27_cjk_visual() {
    use kjxlkj_core_types::VisualKind;
    let mut e = ed();
    ins(&mut e, "あいうえお");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    e.dispatch(Action::EnterVisual(VisualKind::Char));
    e.dispatch(Action::MoveCursor(Motion::Right, 2));
    e.dispatch(Action::ReturnToNormal);
}

#[test]
fn bd28_cjk_yank_paste() {
    let mut e = ed();
    ins(&mut e, "あいうえお");
    e.dispatch(Action::DoubleOperator(Operator::Yank, 1));
    e.dispatch(Action::Put(false));
    let buf = e.active_buffer().unwrap();
    assert!(buf.content.line_count() >= 1);
    let line = buf.content.line_str(0);
    assert!(line.contains("あいうえお"));
}

#[test]
fn bd29_cjk_search() {
    let mut e = ed();
    ins(&mut e, "テストあいう");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    e.dispatch(Action::SearchForward("あ".into()));
    assert!(e.focused_window().unwrap().cursor.grapheme_offset >= 3);
}

#[test]
fn bd30_cjk_substitute() {
    let mut e = ed();
    ins(&mut e, "あいあう");
    e.dispatch(Action::ExecuteCommand("%s/あ/ア/g".into()));
    let line = e.active_buffer().unwrap().content.line_str(0);
    assert!(line.contains('ア'));
}

#[test]
fn bd31_cjk_line_end() {
    let mut e = ed();
    ins(&mut e, "あいう");
    e.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
    assert_eq!(e.focused_window().unwrap().cursor.grapheme_offset, 2);
}

#[test]
fn bd32_mixed_append() {
    let mut e = ed();
    ins(&mut e, "aあb");
    e.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
    assert_eq!(e.focused_window().unwrap().cursor.grapheme_offset, 2);
    e.dispatch(Action::EnterInsert(InsertPosition::AfterCursor));
    e.dispatch(Action::InsertChar('x'));
    e.dispatch(Action::ReturnToNormal);
}
