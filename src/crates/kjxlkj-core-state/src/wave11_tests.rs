//! Wave 11 tests: read-only regs, changelist, range validation,
//! session cursors, block render, macro error halt, very-magic, hlsearch render.
#![cfg(test)]

use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier};

fn make_editor() -> crate::editor::EditorState {
    let mut ed = crate::editor::EditorState::new(80, 24);
    ed.open_file("/tmp/test.txt", "line one\nline two\nline three\n");
    ed
}

/// REQ-ROREG-01: Read-only register % returns current file path.
#[test]
fn readonly_register_percent() {
    let mut ed = make_editor();
    // Select register %, then put after — should paste "/tmp/test.txt".
    ed.pending_register = Some('%');
    ed.handle_action(kjxlkj_core_types::Action::PutAfter);
    let buf = ed.buffers.current();
    let line: String = buf.content.line(0).chars().collect();
    assert!(line.contains("/tmp/test.txt"), "Expected filename in put, got: {line}");
}

/// REQ-ROREG-01: Read-only register : returns last ex command.
#[test]
fn readonly_register_colon() {
    let mut ed = make_editor();
    ed.execute_ex_command("set number");
    ed.pending_register = Some(':');
    ed.handle_action(kjxlkj_core_types::Action::PutAfter);
    let buf = ed.buffers.current();
    let line: String = buf.content.line(0).chars().collect();
    assert!(line.contains("set number"), "Expected 'set number' in put, got: {line}");
}

/// REQ-CHANGELIST-01: g; navigates changelist backward.
#[test]
fn changelist_navigation() {
    let mut ed = make_editor();
    // Enter insert at line 0, type a char, exit.
    ed.handle_key(Key::char('i'));
    ed.handle_key(Key::char('X'));
    ed.handle_key(Key::esc());
    // Move to line 2 and insert.
    ed.handle_key(Key::char('j'));
    ed.handle_key(Key::char('j'));
    ed.handle_key(Key::char('i'));
    ed.handle_key(Key::char('Y'));
    ed.handle_key(Key::esc());
    assert_eq!(ed.changelist.len(), 2);
    // g; should go to older entry (line 0 area).
    ed.handle_key(Key::char('g'));
    ed.handle_key(Key::char(';'));
    // Should navigate to changelist[0].
    assert_eq!(ed.changelist_idx, 1);
}

/// REQ-RANGERR-01: Backwards range gives error.
#[test]
fn backwards_range_error() {
    let mut ed = make_editor();
    ed.execute_ex_command("3,1d");
    let has_err = ed.notifications.iter().any(|n| n.message.contains("E493"));
    assert!(has_err, "Expected E493 backwards range error");
}

/// REQ-RANGERR-01: Mark not set error.
#[test]
fn mark_not_set_error() {
    let mut ed = make_editor();
    ed.execute_ex_command("'z,'zd");
    let has_err = ed.notifications.iter().any(|n| n.message.contains("E20"));
    assert!(has_err, "Expected E20 mark not set error");
}

/// REQ-SESSCUR-01: Session saves cursor position.
#[test]
fn session_saves_cursor() {
    let mut ed = make_editor();
    ed.windows.focused_mut().cursor.line = 2;
    ed.windows.focused_mut().cursor.grapheme = 3;
    let path = "/tmp/test_session_w11.vim";
    ed.handle_mksession(Some(path));
    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.contains("call cursor(3, 4)"), "Expected cursor pos in session: {content}");
    let _ = std::fs::remove_file(path);
}

/// REQ-BLOCKREN-01: Visual block selection info in snapshot.
#[test]
fn visual_block_in_snapshot() {
    let mut ed = make_editor();
    // Enter visual block mode (Ctrl-V).
    ed.mode = Mode::Visual(kjxlkj_core_types::VisualKind::Block);
    ed.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 0));
    ed.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(2, 3);
    let snap = ed.snapshot();
    let tab = &snap.tabs[0];
    let ws = tab.windows.values().next().unwrap();
    assert!(ws.visual_selection.is_some(), "Expected visual selection in snapshot");
    let vs = ws.visual_selection.as_ref().unwrap();
    assert!(matches!(vs.kind, kjxlkj_core_types::VisualKind::Block));
}

/// REQ-MACERR-01: Macro halts on error.
#[test]
fn macro_halts_on_error() {
    let mut ed = make_editor();
    // Record a macro that does an unknown command.
    ed.start_recording('a');
    ed.record_key(&Key::char(':'));
    ed.record_key(&Key::char('n'));
    ed.record_key(&Key::char('o'));
    ed.record_key(&Key::char('p'));
    ed.record_key(&Key::char('e'));
    ed.record_key(&Key::new(KeyCode::Enter, Modifier::NONE));
    ed.stop_recording();
    // Play it 100 times — should halt on first error.
    ed.notifications.clear();
    ed.play_macro('a', 100);
    // Should have at most a few error notifications, not 100.
    let err_count = ed.notifications.iter().filter(|n|
        matches!(n.level, kjxlkj_core_ui::NotificationLevel::Error)).count();
    assert!(err_count <= 2, "Expected macro to halt on error, got {err_count} errors");
}

/// REQ-VMAGIC-01: Search with \v prefix uses regex.
#[test]
fn very_magic_search() {
    let mut ed = make_editor();
    ed.open_file("/tmp/vmagic.txt", "abc123\ndef456\nghi789\n");
    // Set search pattern with \v prefix.
    ed.search.pattern = Some("\\v[0-9]+".to_string());
    ed.search.active = true;
    ed.search.forward = true;
    ed.windows.focused_mut().cursor.line = 0;
    ed.windows.focused_mut().cursor.grapheme = 0;
    ed.search_next();
    // Should have moved to the first digit match.
    let cursor = ed.windows.focused().cursor;
    assert_eq!(cursor.grapheme, 3, "Expected cursor at col 3 (start of 123)");
}

/// REQ-HLRENDER-01: Compute hlsearch produces highlight ranges.
#[test]
fn hlsearch_produces_ranges() {
    let mut ed = make_editor();
    ed.options.set("hlsearch", crate::options::OptionValue::Bool(true));
    ed.search.pattern = Some("line".to_string());
    ed.search.active = true;
    let snap = ed.snapshot();
    assert!(!snap.search.highlight_ranges.is_empty(), "Expected highlight ranges for 'line'");
    // All ranges should reference column 0 (where "line" starts).
    for &(_, start, _) in &snap.search.highlight_ranges {
        assert_eq!(start, 0, "Expected highlight start at col 0");
    }
}
