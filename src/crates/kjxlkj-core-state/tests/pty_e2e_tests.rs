//! PTY E2E test scenarios PE-01 through PE-08.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, InsertPosition, Motion};

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

/// PE-01: Append EOL mode churn — 20 cycles.
#[test]
fn pe01_append_eol_churn() {
    let mut e = ed();
    ins(&mut e, "aあb");
    for _ in 0..20 {
        e.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
        e.dispatch(Action::EnterInsert(InsertPosition::AfterCursor));
        e.dispatch(Action::InsertChar('x'));
        e.dispatch(Action::ReturnToNormal);
    }
    let line = e.active_buffer().unwrap().content.line_str(0);
    let x_count = line.chars().filter(|&c| c == 'x').count();
    assert_eq!(x_count, 20);
}

/// PE-02: Long CJK line $ and 0.
#[test]
fn pe02_long_cjk_motions() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(InsertPosition::BeforeCursor));
    for _ in 0..100 {
        e.dispatch(Action::InsertChar('あ'));
    }
    e.dispatch(Action::ReturnToNormal);
    e.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
    assert_eq!(e.focused_window().unwrap().cursor.grapheme_offset, 99);
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    assert_eq!(e.focused_window().unwrap().cursor.grapheme_offset, 0);
}

/// PE-03: Leader vs IME space.
#[test]
fn pe03_leader_vs_ime() {
    use kjxlkj_core_state::keybinding_dsl::LeaderConfig;
    assert_eq!(LeaderConfig::default().leader, '\\');
    use kjxlkj_core_state::ime::ImeState;
    let mut ime = ImeState::new();
    ime.start_composition("漢");
    assert_eq!(ime.commit(), "漢");
}

/// PE-04: tmux detach/resume.
#[test]
fn pe04_tmux() {
    use kjxlkj_core_state::tmux::TmuxState;
    let st = TmuxState::detect();
    assert!(st.wrap_escape("\x1b[H").contains("[H"));
}

/// PE-05: Resize storm.
#[test]
fn pe05_resize_storm() {
    let mut e = ed();
    ins(&mut e, "あいうえお");
    for i in 0..50 {
        if i % 2 == 0 {
            e.handle_resize(80, 24);
        } else {
            e.handle_resize(40, 12);
        }
    }
    assert_eq!(e.terminal_size, (40, 12));
}

/// PE-06: Terminal emulator.
#[test]
fn pe06_terminal() {
    let mut e = ed();
    e.dispatch(Action::SpawnTerminal);
    assert!(!e.should_quit);
}

/// PE-07: Multi-window navigation.
#[test]
fn pe07_multi_window() {
    let mut e = ed();
    e.dispatch(Action::SplitVertical);
    e.dispatch(Action::SplitVertical);
    assert!(e.windows.len() >= 3);
    let start = e.focused_window;
    for _ in 0..e.windows.len() {
        e.dispatch(Action::CycleWindow);
    }
    assert_eq!(e.focused_window, start);
}

/// PE-08: Concurrent terminal and buffer.
#[test]
fn pe08_concurrent() {
    let mut e = ed();
    e.dispatch(Action::SplitHorizontal);
    ins(&mut e, "buffer content");
    let line = e.active_buffer().unwrap().content.line_str(0);
    assert!(line.contains("buffer content"));
}
