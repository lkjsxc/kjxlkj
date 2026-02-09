//! Wave 10 tests.

use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier};

use crate::editor::EditorState;

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn enter() -> Key {
    Key::new(KeyCode::Enter, Modifier::NONE)
}

#[test]
fn test_yank_sets_bracket_marks() {
    let mut e = ed();
    e.open_file("t.txt", "aaa\nbbb\nccc\n");
    e.handle_key(Key::char('y'));
    e.handle_key(Key::char('y'));
    let bid = e.current_buffer_id().0 as usize;
    let s = e.marks.get('[', bid);
    let end = e.marks.get(']', bid);
    assert!(s.is_some(), "[ mark should be set after yank");
    assert!(end.is_some(), "] mark should be set after yank");
    assert_eq!(s.unwrap().line, 0);
    assert_eq!(end.unwrap().line, 0);
}

#[test]
fn test_macro_depth_limit() {
    let mut e = ed();
    e.open_file("t.txt", "hello\n");
    // Record macro that calls itself: qajq -> then @a should stop.
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('a'));
    // Record: @a (self-call)
    e.handle_key(Key::char('@'));
    e.handle_key(Key::char('a'));
    e.handle_key(Key::char('q'));
    // Play: @a — should not infinite loop due to depth limit.
    e.handle_key(Key::char('@'));
    e.handle_key(Key::char('a'));
    // If we get here, the depth limit worked.
    assert!(matches!(e.mode, Mode::Normal));
}

#[test]
fn test_count_range_offset() {
    let mut e = ed();
    e.open_file("t.txt", "line1\nline2\nline3\nline4\nline5\n");
    // Go to line 3 via :3
    e.handle_key(Key::char(':'));
    for c in "3".chars() {
        e.handle_key(Key::char(c));
    }
    e.handle_key(enter());
    assert_eq!(e.windows.focused().cursor.line, 2);
}

#[test]
fn test_bare_offset_range() {
    let mut e = ed();
    e.open_file("t.txt", "line1\nline2\nline3\nline4\nline5\n");
    // Set cursor to line 2 (0-indexed 1).
    e.handle_key(Key::char('j'));
    // :+2 should go to line 4 (0-indexed 3).
    e.handle_key(Key::char(':'));
    for c in "+2".chars() {
        e.handle_key(Key::char(c));
    }
    e.handle_key(enter());
    assert_eq!(e.windows.focused().cursor.line, 3);
}

#[test]
fn test_expression_eval_basic() {
    use crate::expr_eval::eval_expression;
    assert_eq!(eval_expression("2+3").unwrap(), "5");
    assert_eq!(eval_expression("10*5").unwrap(), "50");
    assert_eq!(eval_expression("17%5").unwrap(), "2");
}

#[test]
fn test_expression_eval_string() {
    use crate::expr_eval::eval_expression;
    assert_eq!(eval_expression("\"hello\"").unwrap(), "hello");
    assert_eq!(eval_expression("\"a\" . \"b\"").unwrap(), "ab");
}

#[test]
fn test_last_inserted_text_register() {
    let mut e = ed();
    e.open_file("t.txt", "hello\n");
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('X'));
    e.handle_key(Key::char('Y'));
    e.handle_key(Key::esc());
    let reg = e
        .registers
        .get(kjxlkj_core_edit::RegisterName::LastInserted);
    assert!(reg.is_some(), ". register should have content");
    assert_eq!(reg.unwrap().content, "XY");
}

#[test]
fn test_user_command_completion() {
    let mut e = ed();
    // Define a user command.
    e.execute_ex_command("command! MyTest echo test");
    // Now try completing "My" — should find MyTest.
    e.cmdline.open(':');
    e.cmdline.content = "My".to_string();
    e.cmdline_complete_next();
    assert_eq!(e.cmdline.content, "MyTest");
}

#[test]
fn test_config_section_headers() {
    use std::io::Write;
    let dir = std::env::temp_dir().join("kjxlkj_test_cfg");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("test_config.toml");
    {
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "[editor]").unwrap();
        writeln!(f, "tabstop = 4").unwrap();
        writeln!(f, "[ui]").unwrap();
        writeln!(f, "theme = \"dark\"").unwrap();
    }
    let mut e = ed();
    e.load_config_file(path.to_str().unwrap());
    assert_eq!(e.options.get_int("editor.tabstop"), 4);
    assert_eq!(e.options.get_str("ui.theme"), "dark");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_formatoptions_gq() {
    let mut e = ed();
    e.options.set(
        "formatoptions",
        crate::options::OptionValue::Str("tcq".to_string()),
    );
    e.options
        .set("textwidth", crate::options::OptionValue::Int(20));
    e.open_file("t.txt", "a bb cc dd ee ff gg hh ii jj kk\n");
    // gqq should format.
    e.handle_key(Key::char('g'));
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('q'));
    let buf = e.buffers.current();
    let text = buf.content.to_string();
    // Should be wrapped at ~20 chars.
    assert!(text.contains('\n'));
    let first_line = text.lines().next().unwrap();
    assert!(first_line.len() <= 21);
}
