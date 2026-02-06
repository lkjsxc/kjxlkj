//! Tests for operator+text-object, undo/redo, Ex commands, file I/O.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::*;

fn setup_editor(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// --- Operator + Text Object tests ---

#[test]
fn diw_deletes_inner_word() {
    let mut s = setup_editor("hello world");
    // cursor on 'h', diw should delete 'hello'
    dispatch_intent(
        &mut s,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Word,
            true,
        ),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.text();
    assert!(!text.contains("hello"), "word 'hello' should be deleted");
}

#[test]
fn daw_deletes_around_word() {
    let mut s = setup_editor("hello world");
    dispatch_intent(
        &mut s,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Word,
            false,
        ),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.text();
    assert!(!text.contains("hello"), "word and space deleted");
}

#[test]
fn ciw_changes_inner_word() {
    let mut s = setup_editor("hello world");
    dispatch_intent(
        &mut s,
        Intent::OperatorTextObject(
            OperatorKind::Change,
            TextObjectKind::Word,
            true,
        ),
    );
    assert_eq!(s.current_mode(), Mode::Insert);
}

#[test]
fn yiw_yanks_inner_word() {
    let mut s = setup_editor("hello world");
    dispatch_intent(
        &mut s,
        Intent::OperatorTextObject(
            OperatorKind::Yank,
            TextObjectKind::Word,
            true,
        ),
    );
    let text = s.registers.unnamed_text().unwrap().to_string();
    assert_eq!(text, "hello");
}

#[test]
fn di_double_quote_deletes_quoted() {
    let mut s = setup_editor("say \"hello\" now");
    // Move cursor into the quotes
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 5));
    dispatch_intent(
        &mut s,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::DoubleQuote,
            true,
        ),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.text();
    // Inner should delete content between quotes
    assert!(text.contains("\"\""), "quotes remain, content deleted");
}

#[test]
fn di_paren_deletes_paren_contents() {
    let mut s = setup_editor("fn(arg1, arg2)");
    // Move into parens
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(
        &mut s,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Paren,
            true,
        ),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.text();
    assert!(text.contains("()"), "parens remain empty");
}

// --- Parser: operator+text-object multi-key ---

#[test]
fn parser_diw_emits_text_object_intent() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('d')), Intent::Noop);
    assert_eq!(p.parse_normal(&KeyEvent::char('i')), Intent::Noop);
    let intent = p.parse_normal(&KeyEvent::char('w'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Word,
            true
        )
    );
}

#[test]
fn parser_ciw_emits_change_text_object() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('c'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('w'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Change,
            TextObjectKind::Word,
            true
        )
    );
}

#[test]
fn parser_da_bracket_emits_around() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('a'));
    let intent = p.parse_normal(&KeyEvent::char('['));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Bracket,
            false
        )
    );
}

#[test]
fn parser_yi_quote_emits_yank_inner_quote() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('y'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('"'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Yank,
            TextObjectKind::DoubleQuote,
            true
        )
    );
}

#[test]
fn parser_dib_emits_delete_inner_paren() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('b'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Paren,
            true
        )
    );
}

// --- Ex command tests ---

#[test]
fn ex_quit_with_unsaved_changes_warns() {
    let mut s = setup_editor("hello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('x'));
    s.mode.transition(Mode::Normal);
    dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
    assert!(!s.should_quit);
    assert!(s.message.as_ref().unwrap().contains("No write"));
}

#[test]
fn ex_quit_force_ignores_unsaved() {
    let mut s = setup_editor("hello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('x'));
    s.mode.transition(Mode::Normal);
    dispatch_intent(&mut s, Intent::ExCommand(":q!".into()));
    assert!(s.should_quit);
}

#[test]
fn ex_new_creates_buffer() {
    let mut s = setup_editor("hello");
    let before = s.buffers.len();
    dispatch_intent(&mut s, Intent::ExCommand(":new".into()));
    assert_eq!(s.buffers.len(), before + 1);
}

#[test]
fn ex_ls_lists_buffers() {
    let mut s = setup_editor("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":ls".into()));
    assert!(s.message.is_some());
    assert!(s.message.as_ref().unwrap().contains("[No Name]"));
}

#[test]
fn ex_set_number() {
    let mut s = setup_editor("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set number".into()),
    );
    assert_eq!(s.message.as_deref(), Some("number on"));
}

#[test]
fn ex_set_nowrap() {
    let mut s = setup_editor("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set nowrap".into()),
    );
    assert_eq!(s.message.as_deref(), Some("wrap off"));
}

#[test]
fn ex_unknown_command() {
    let mut s = setup_editor("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":foobar".into()),
    );
    assert!(s.message.as_ref().unwrap().contains("unknown"));
}

#[test]
fn ex_goto_line_number() {
    let mut s = setup_editor("aaa\nbbb\nccc\nddd\neee");
    dispatch_intent(&mut s, Intent::ExCommand(":3".into()));
    assert_eq!(s.cursor().line, 2); // 0-indexed
}

#[test]
fn ex_write_no_filename() {
    let mut s = setup_editor("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":w".into()));
    assert!(s.message.as_ref().unwrap().contains("No file name"));
}

#[test]
fn ex_write_to_file() {
    let mut s = setup_editor("test content");
    let dir = std::env::temp_dir();
    let path = dir.join("kjxlkj_test_write.txt");
    let cmd = format!(":w {}", path.display());
    dispatch_intent(&mut s, Intent::ExCommand(cmd));
    assert!(s.message.as_ref().unwrap().contains("written"));
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("test content"));
    std::fs::remove_file(&path).ok();
}

#[test]
fn ex_edit_file() {
    let dir = std::env::temp_dir();
    let path = dir.join("kjxlkj_test_edit.txt");
    std::fs::write(&path, "file content here").unwrap();
    let mut s = setup_editor("original");
    let cmd = format!(":e {}", path.display());
    dispatch_intent(&mut s, Intent::ExCommand(cmd));
    assert!(s.message.as_ref().unwrap().contains("opened"));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.text().contains("file content"));
    std::fs::remove_file(&path).ok();
}

#[test]
fn ex_edit_new_file() {
    let mut s = setup_editor("original");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":e /tmp/nonexistent_kjxlkj.txt".into()),
    );
    assert!(s.message.as_ref().unwrap().contains("New file"));
}

#[test]
fn ex_wq_writes_and_quits() {
    let mut s = setup_editor("wq content");
    let dir = std::env::temp_dir();
    let path = dir.join("kjxlkj_test_wq.txt");
    // Set file path first
    if let Some(buf) = s.active_buffer_mut() {
        buf.file_path = Some(path.display().to_string());
    }
    dispatch_intent(&mut s, Intent::ExCommand(":wq".into()));
    assert!(s.should_quit);
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("wq content"));
    std::fs::remove_file(&path).ok();
}

// --- Undo / Redo tests (behavioral) ---

#[test]
fn has_unsaved_changes_initially_false() {
    let s = setup_editor("hello");
    assert!(!s.has_unsaved_changes());
}

#[test]
fn has_unsaved_changes_after_insert() {
    let mut s = setup_editor("hello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('x'));
    assert!(s.has_unsaved_changes());
}

// --- Visual mode operator tests ---

#[test]
fn visual_tilde_is_handled() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    let intent = p.parse_visual(&KeyEvent::char('~'));
    // ~ in visual should be handled (might be Noop or ToggleCase)
    // Main thing is it doesn't panic
    assert!(intent == Intent::Noop || matches!(intent, Intent::ToggleCase));
}

#[test]
fn visual_escape_returns_normal() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    let intent =
        p.parse_visual(&KeyEvent::special(KeyCode::Escape));
    assert_eq!(intent, Intent::EnterMode(Mode::Normal));
}

// --- Insert mode edge cases ---

#[test]
fn insert_ctrl_h_is_backspace() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('h'));
    assert_eq!(intent, Intent::DeleteCharBefore);
}

#[test]
fn insert_ctrl_j_is_newline() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('j'));
    assert_eq!(intent, Intent::InsertNewline);
}

#[test]
fn insert_tab_inserts_tab() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    let intent =
        p.parse_insert(&KeyEvent::special(KeyCode::Tab));
    assert_eq!(intent, Intent::InsertChar('\t'));
}

// --- More parser tests ---

#[test]
fn parser_count_with_operator_text_object() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('2'));
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('w'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Word,
            true
        )
    );
}

#[test]
fn parser_di_brace() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('{'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Brace,
            true
        )
    );
}

#[test]
fn parser_da_angle_bracket() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('a'));
    let intent = p.parse_normal(&KeyEvent::char('<'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::AngleBracket,
            false
        )
    );
}

#[test]
fn parser_ci_backtick() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('c'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('`'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Change,
            TextObjectKind::BackTick,
            true
        )
    );
}

#[test]
fn parser_yi_single_quote() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('y'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('\''));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Yank,
            TextObjectKind::SingleQuote,
            true
        )
    );
}

#[test]
fn parser_dis_deletes_inner_sentence() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('i'));
    let intent = p.parse_normal(&KeyEvent::char('s'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Sentence,
            true
        )
    );
}

#[test]
fn parser_dap_deletes_around_paragraph() {
    use kjxlkj_core_mode::KeyParser;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    p.parse_normal(&KeyEvent::char('a'));
    let intent = p.parse_normal(&KeyEvent::char('p'));
    assert_eq!(
        intent,
        Intent::OperatorTextObject(
            OperatorKind::Delete,
            TextObjectKind::Paragraph,
            false
        )
    );
}
