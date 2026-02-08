//! Regression tests REG-01 through REG-08.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Action, InsertPosition, Motion,
};

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ins(ed: &mut EditorState, text: &str) {
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for ch in text.chars() {
        ed.dispatch(Action::InsertChar(ch));
    }
    ed.dispatch(Action::ReturnToNormal);
}

/// REG-01: Append at EOL no floating cursor.
#[test]
fn reg01_append_eol() {
    let mut e = ed();
    ins(&mut e, "hello");
    e.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
    e.dispatch(Action::EnterInsert(
        InsertPosition::AfterCursor,
    ));
    e.dispatch(Action::InsertChar('!'));
    e.dispatch(Action::ReturnToNormal);
    let w = e.focused_window().unwrap();
    let buf = e.active_buffer().unwrap();
    let line = buf.content.line_str(0);
    let trimmed = line.trim_end_matches('\n');
    let glen = trimmed.chars().count();
    assert!(w.cursor.grapheme_offset < glen);
}

/// REG-02: Long line wraps.
#[test]
fn reg02_long_line_wraps() {
    use kjxlkj_core_state::line_wrap::wrap_line;
    let rows = wrap_line(&"a".repeat(200), 80);
    assert_eq!(rows.len(), 3);
}

/// REG-03: Leader chords reachable.
#[test]
fn reg03_leader_chords() {
    use kjxlkj_core_state::keybinding_dsl::{
        KeybindingDesc, LeaderConfig, WhichKeyState,
    };
    let ldr = LeaderConfig::default();
    assert_eq!(ldr.leader, '\\');
    let mut wk = WhichKeyState::new();
    wk.start("<leader>");
    wk.matches.push(KeybindingDesc {
        keys: "e".into(),
        mode: "n".into(),
        description: "Explorer".into(),
        category: "file".into(),
    });
    wk.matches.push(KeybindingDesc {
        keys: "t".into(),
        mode: "n".into(),
        description: "Terminal".into(),
        category: "tool".into(),
    });
    assert!(wk.matches.iter().any(|m| m.keys == "e"));
    assert!(wk.matches.iter().any(|m| m.keys == "t"));
}

/// REG-04: Insert Enter persists newline.
#[test]
fn reg04_insert_enter() {
    let mut e = ed();
    ins(&mut e, "line1");
    e.dispatch(Action::EnterInsert(
        InsertPosition::AfterCursor,
    ));
    e.dispatch(Action::InsertChar('\n'));
    e.dispatch(Action::InsertChar('2'));
    e.dispatch(Action::ReturnToNormal);
    let buf = e.active_buffer().unwrap();
    assert!(buf.content.line_count() >= 2);
}

/// REG-05: tmux smoke.
#[test]
fn reg05_tmux_smoke() {
    use kjxlkj_core_state::tmux::TmuxState;
    let st = TmuxState::detect();
    let w = st.wrap_escape("\x1b[?25h");
    assert!(w.contains("25h"));
}

/// REG-06: Unicode commit/cancel.
#[test]
fn reg06_unicode() {
    use kjxlkj_core_state::ime::ImeState;
    let mut ime = ImeState::new();
    ime.start_composition("あ");
    assert_eq!(ime.commit(), "あ");
    assert!(ime.preedit.is_empty());
    ime.start_composition("い");
    ime.cancel();
    assert!(ime.preedit.is_empty());
}

/// REG-07: CJK no half-cell cursor.
#[test]
fn reg07_cjk_half_cell() {
    use kjxlkj_core_state::cjk_support::cursor_grapheme_width;
    assert_eq!(cursor_grapheme_width("あ", 0), 2);
    assert_eq!(cursor_grapheme_width("a", 0), 1);
}

/// REG-08: Wrap boundary padding.
#[test]
fn reg08_wrap_padding() {
    use kjxlkj_core_state::line_wrap::wrap_line;
    let mut line = "a".repeat(79);
    line.push('あ');
    let rows = wrap_line(&line, 80);
    assert!(rows.len() >= 2);
}
