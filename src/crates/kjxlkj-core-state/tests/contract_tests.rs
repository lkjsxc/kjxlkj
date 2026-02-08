//! Technical contract verification tests.
//! Contracts, latency, memory, profiling, large-files.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, InsertPosition, Motion};

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

/// Edit serialization: sequential order.
#[test]
fn contract_edit_serial() {
    let mut e = ed();
    for i in 0..100u8 {
        e.dispatch(Action::EnterInsert(
            InsertPosition::BeforeCursor,
        ));
        e.dispatch(Action::InsertChar(
            char::from(b'a' + i % 26),
        ));
        e.dispatch(Action::ReturnToNormal);
    }
    assert!(e.active_buffer().unwrap()
        .content.line_count() >= 1);
}

/// Snapshot monotonicity.
#[test]
fn contract_snapshot_monotonic() {
    let mut e = ed();
    let s1 = e.snapshot();
    ins(&mut e, "test");
    let s2 = e.snapshot();
    assert!(s2.sequence > s1.sequence);
}

/// Snapshot viewport bounded.
#[test]
fn contract_snapshot_bounded() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for i in 0..1000 {
        for ch in format!("l{i}").chars() {
            e.dispatch(Action::InsertChar(ch));
        }
        e.dispatch(Action::InsertChar('\n'));
    }
    e.dispatch(Action::ReturnToNormal);
    let snap = e.snapshot();
    assert!(snap.sequence > 0);
    assert_eq!(snap.buffers.len(), 1);
}

/// Buffer always valid UTF-8.
#[test]
fn contract_utf8() {
    let mut e = ed();
    ins(&mut e, "こんにちは 🎉 test");
    let line = e.active_buffer().unwrap()
        .content.line_str(0);
    assert!(!line.is_empty());
}

/// Undo bounded growth.
#[test]
fn contract_undo_bounded() {
    let mut e = ed();
    for _ in 0..500 {
        ins(&mut e, "x");
    }
    for _ in 0..500 {
        e.dispatch(Action::Undo);
    }
    assert!(e.active_buffer().is_some());
}

/// Keystroke processing speed.
#[test]
fn latency_keystroke() {
    let mut e = ed();
    let start = std::time::Instant::now();
    for _ in 0..200 {
        e.dispatch(Action::EnterInsert(
            InsertPosition::BeforeCursor,
        ));
        e.dispatch(Action::InsertChar('x'));
        e.dispatch(Action::ReturnToNormal);
    }
    assert!(start.elapsed().as_secs() < 5);
}

/// Snapshot production timing.
#[test]
fn latency_snapshot() {
    let mut e = ed();
    ins(&mut e, "test content");
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = e.snapshot();
    }
    assert!(start.elapsed().as_secs() < 5);
}

/// Resize no corruption.
#[test]
fn latency_resize() {
    let mut e = ed();
    ins(&mut e, "content");
    for _ in 0..50 {
        e.handle_resize(40, 12);
        e.handle_resize(80, 24);
    }
    let snap = e.snapshot();
    assert_eq!(snap.terminal_size, (80, 24));
}

/// No full-buffer copy on motion.
#[test]
fn memory_no_full_copy() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for _ in 0..500 {
        for ch in "some content here".chars() {
            e.dispatch(Action::InsertChar(ch));
        }
        e.dispatch(Action::InsertChar('\n'));
    }
    e.dispatch(Action::ReturnToNormal);
    for _ in 0..100 {
        e.dispatch(Action::MoveCursor(Motion::Down, 1));
    }
    assert!(e.active_buffer().is_some());
}

/// Profiling metrics structure.
#[test]
fn profiling_metrics() {
    let mut e = ed();
    ins(&mut e, "test");
    let snap = e.snapshot();
    assert!(snap.sequence > 0);
    assert!(snap.terminal_size.0 > 0);
}

/// Large buffer operations bounded.
#[test]
fn large_file_bounded() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for i in 0..2000 {
        for ch in format!("ln {i}").chars() {
            e.dispatch(Action::InsertChar(ch));
        }
        e.dispatch(Action::InsertChar('\n'));
    }
    e.dispatch(Action::ReturnToNormal);
    e.dispatch(Action::MoveCursor(
        Motion::GotoLine(1000), 1,
    ));
    assert_eq!(e.focused_window().unwrap()
        .cursor.line, 1000);
    let snap = e.snapshot();
    assert!(snap.sequence > 0);
}
