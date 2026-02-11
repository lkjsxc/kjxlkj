//! Tests for command-line input handling.
use crate::editor::EditorState;
use kjxlkj_core_types::{BufferId, CommandKind, Key, KeyModifiers, Mode};

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn type_str(s: &mut EditorState, text: &str, kind: CommandKind) {
    for c in text.chars() { s.handle_command_input(&Key::Char(c), &m(), kind); }
}

#[test]
fn ex_quit_from_cmdline() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "q", CommandKind::Ex);
    s.handle_command_input(&Key::Enter, &m(), CommandKind::Ex);
    assert!(s.quit_requested);
}

#[test]
fn search_forward_moves_cursor() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "hello world").unwrap();
    s.mode = Mode::Command(CommandKind::SearchForward);
    s.activate_cmdline(CommandKind::SearchForward);
    type_str(&mut s, "world", CommandKind::SearchForward);
    s.handle_command_input(&Key::Enter, &m(), CommandKind::SearchForward);
    assert_eq!(s.windows.get(&s.focus.focused).unwrap().cursor.col, 6);
}

#[test]
fn escape_cancels_cmdline() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "w", CommandKind::Ex);
    s.handle_command_input(&Key::Escape, &m(), CommandKind::Ex);
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn backspace_on_empty_exits() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    s.handle_command_input(&Key::Backspace, &m(), CommandKind::Ex);
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn left_right_cursor() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "abc", CommandKind::Ex);
    assert_eq!(s.cmdline.cursor_pos, 3);
    s.handle_command_input(&Key::Left, &m(), CommandKind::Ex);
    assert_eq!(s.cmdline.cursor_pos, 2);
    s.handle_command_input(&Key::Right, &m(), CommandKind::Ex);
    assert_eq!(s.cmdline.cursor_pos, 3);
}

#[test]
fn ctrl_b_e_beginning_end() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "hello", CommandKind::Ex);
    let ctrl = KeyModifiers { ctrl: true, ..Default::default() };
    s.handle_command_input(&Key::Char('b'), &ctrl, CommandKind::Ex);
    assert_eq!(s.cmdline.cursor_pos, 0);
    s.handle_command_input(&Key::Char('e'), &ctrl, CommandKind::Ex);
    assert_eq!(s.cmdline.cursor_pos, 5);
}

#[test]
fn ctrl_w_deletes_word() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "hello world", CommandKind::Ex);
    let ctrl = KeyModifiers { ctrl: true, ..Default::default() };
    s.handle_command_input(&Key::Char('w'), &ctrl, CommandKind::Ex);
    assert_eq!(s.cmdline.content, "hello ");
}

#[test]
fn ctrl_u_deletes_to_start() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "abcdef", CommandKind::Ex);
    s.handle_command_input(&Key::Left, &m(), CommandKind::Ex);
    s.handle_command_input(&Key::Left, &m(), CommandKind::Ex);
    let ctrl = KeyModifiers { ctrl: true, ..Default::default() };
    s.handle_command_input(&Key::Char('u'), &ctrl, CommandKind::Ex);
    assert_eq!(s.cmdline.content, "ef");
    assert_eq!(s.cmdline.cursor_pos, 0);
}

#[test]
fn delete_key_removes_under_cursor() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "abc", CommandKind::Ex);
    let ctrl = KeyModifiers { ctrl: true, ..Default::default() };
    s.handle_command_input(&Key::Char('b'), &ctrl, CommandKind::Ex);
    s.handle_command_input(&Key::Delete, &m(), CommandKind::Ex);
    assert_eq!(s.cmdline.content, "bc");
}

#[test]
fn mid_string_insert() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "ac", CommandKind::Ex);
    s.handle_command_input(&Key::Left, &m(), CommandKind::Ex);
    s.handle_command_input(&Key::Char('b'), &m(), CommandKind::Ex);
    assert_eq!(s.cmdline.content, "abc");
    assert_eq!(s.cmdline.cursor_pos, 2);
}

#[test]
fn ex_updates_colon_register() {
    let mut s = ed();
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    type_str(&mut s, "write", CommandKind::Ex);
    s.handle_command_input(&Key::Enter, &m(), CommandKind::Ex);
    assert_eq!(s.registers.get(':').unwrap().text, "write");
}

#[test]
fn search_updates_slash_register() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "foo bar baz").unwrap();
    s.mode = Mode::Command(CommandKind::SearchForward);
    s.activate_cmdline(CommandKind::SearchForward);
    type_str(&mut s, "bar", CommandKind::SearchForward);
    s.handle_command_input(&Key::Enter, &m(), CommandKind::SearchForward);
    assert_eq!(s.registers.get('/').unwrap().text, "bar");
}
