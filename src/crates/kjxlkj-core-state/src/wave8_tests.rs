//! Wave 8 tests: visual block, special marks, session,
//! config, semicolon range, file completion, WORD objects.

use kjxlkj_core_types::{Key, Mode, VisualKind};

use crate::editor::EditorState;

fn ed() -> EditorState {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "hello world\nfoo bar\nbaz qux\n");
    e
}

fn buf_text(e: &EditorState) -> String {
    let id = e.current_buffer_id();
    e.buffers.get(id).unwrap().content.to_string()
}

// --- Visual block mode ---

#[test]
fn ctrl_v_enters_visual_block() {
    let mut e = ed();
    e.handle_key(Key::ctrl('v'));
    assert!(matches!(e.mode, Mode::Visual(VisualKind::Block)));
}

#[test]
fn visual_block_delete() {
    let mut e = ed();
    e.handle_key(Key::ctrl('v'));
    e.handle_key(Key::char('l'));
    e.handle_key(Key::char('l'));
    e.handle_key(Key::char('j'));
    e.handle_key(Key::char('d'));
    assert!(matches!(e.mode, Mode::Normal));
    let text = buf_text(&e);
    // Should have removed first 3 cols from first 2 lines
    assert!(text.starts_with("lo"), "got: {}", text);
}

// --- Special marks ---

#[test]
fn dot_mark_set_on_insert_exit() {
    let mut e = ed();
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('X'));
    e.handle_key(Key::esc());
    let bid = e.current_buffer_id().0 as usize;
    let dot = e.marks.get('.', bid);
    assert!(dot.is_some(), "dot mark should be set");
}

#[test]
fn visual_marks_set_on_exit() {
    let mut e = ed();
    e.handle_key(Key::char('v'));
    e.handle_key(Key::char('l'));
    e.handle_key(Key::esc());
    let bid = e.current_buffer_id().0 as usize;
    let lt = e.marks.get('<', bid);
    let gt = e.marks.get('>', bid);
    assert!(lt.is_some(), "< mark should be set");
    assert!(gt.is_some(), "> mark should be set");
}

// --- iW WORD text object ---

#[test]
fn diw_big_deletes_word() {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "foo-bar baz\n");
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('W'));
    let text = buf_text(&e);
    // Should delete "foo-bar" (non-whitespace WORD)
    assert!(text.starts_with(" baz"), "got: {}", text);
}

// --- Semicolon range separator ---

#[test]
fn semicolon_range_sets_context() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["aaa", "bbb", "ccc", "ddd"];
    let ctx = RangeContext {
        current_line: 0,
        total_lines: 4,
        lines: &lines,
        mark_line: None,
        last_search: None,
        vars: None, call_fn: None,
    };
    // 2;/ddd/ means: addr1=line2(idx1), then set current=1 and search /ddd/
    let (range, _) = parse_range_ctx("2;/ddd/d", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 1); // line 2 = index 1
    assert_eq!(r.end, 3); // ddd = index 3
}

// --- :source command ---

#[test]
fn source_executes_file_commands() {
    let mut e = ed();
    // Create a temp file with commands.
    let path = "/tmp/kjxlkj_test_source.vim";
    std::fs::write(path, "\" comment\nset number\n").unwrap();
    e.handle_source(path);
    assert!(e.options.get_bool("number"));
    std::fs::remove_file(path).ok();
}

// --- :registers display ---

#[test]
fn registers_display_includes_content() {
    let mut e = ed();
    e.handle_key(Key::char('y'));
    e.handle_key(Key::char('y'));
    e.handle_list_registers_filtered("");
    // Should set notification
    assert!(!e.notifications.is_empty());
}

// --- Config loading ---

#[test]
fn config_file_applies_settings() {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "test\n");
    let path = "/tmp/kjxlkj_test_config.toml";
    std::fs::write(path, "number = true\ntabstop = 8\n").unwrap();
    e.load_config_file(path);
    assert!(e.options.get_bool("number"));
    assert_eq!(e.options.get_int("tabstop"), 8);
    std::fs::remove_file(path).ok();
}
