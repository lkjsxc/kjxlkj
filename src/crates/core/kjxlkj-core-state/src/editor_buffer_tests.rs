//! Integration tests for buffer management via handle_key / apply_action.
//! Tests buffer switching, alternate file, navigation, and deletion
//! through the full editor pipeline.

use crate::editor::EditorState;
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{Action, BufferId, ContentKind, Key, KeyModifiers, Mode};

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }

/// Helper: add a scratch buffer and return its ID.
fn add_buf(s: &mut EditorState) -> BufferId {
    let id = BufferId(s.next_id());
    s.buffers.insert(id, Buffer::new_scratch(id));
    id
}

fn cur_buf(s: &EditorState) -> BufferId {
    let wid = s.focus.focused;
    match s.windows.get(&wid).unwrap().content {
        ContentKind::Buffer(id) => id, _ => panic!("no buffer"),
    }
}

#[test]
fn bn_via_command_cycles_buffers() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    // :bn to go to id1
    s.apply_action(Action::NextBuffer);
    assert_eq!(cur_buf(&s), id1);
    // :bn wraps to buffer 0
    s.apply_action(Action::NextBuffer);
    assert_eq!(cur_buf(&s), BufferId(0));
}

#[test]
fn bp_via_command_cycles_reverse() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    // :bp wraps to last buffer
    s.apply_action(Action::PreviousBuffer);
    assert_eq!(cur_buf(&s), id1);
}

#[test]
fn bd_removes_current_and_falls_back() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    s.apply_action(Action::NextBuffer); // go to id1
    assert_eq!(cur_buf(&s), id1);
    s.apply_action(Action::DeleteBuffer);
    assert_eq!(cur_buf(&s), BufferId(0));
    assert!(!s.buffers.contains_key(&id1));
}

#[test]
fn switch_buffer_by_id() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    let id2 = add_buf(&mut s);
    s.apply_action(Action::SwitchBuffer(id2));
    assert_eq!(cur_buf(&s), id2);
    s.apply_action(Action::SwitchBuffer(id1));
    assert_eq!(cur_buf(&s), id1);
}

#[test]
fn ctrl_caret_toggles_alternate() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    s.apply_action(Action::NextBuffer); // to id1, alternate=0
    assert_eq!(s.alternate_buffer, Some(BufferId(0)));
    s.apply_action(Action::SwitchAlternate); // back to 0, alternate=id1
    assert_eq!(cur_buf(&s), BufferId(0));
    assert_eq!(s.alternate_buffer, Some(id1));
    s.apply_action(Action::SwitchAlternate); // back to id1
    assert_eq!(cur_buf(&s), id1);
}

#[test]
fn bfirst_blast_navigate() {
    let mut s = ed();
    let _id1 = add_buf(&mut s);
    let id2 = add_buf(&mut s);
    s.apply_action(Action::LastBuffer);
    assert_eq!(cur_buf(&s), id2);
    s.apply_action(Action::FirstBuffer);
    assert_eq!(cur_buf(&s), BufferId(0));
}

#[test]
fn open_file_dedup() {
    let mut s = ed();
    // Opening same path twice should not create duplicate buffers.
    s.open_file("/tmp/test_kjxlkj_nonexist");
    let count1 = s.buffers.len();
    let id1 = cur_buf(&s);
    s.apply_action(Action::NextBuffer); // go somewhere else
    s.open_file("/tmp/test_kjxlkj_nonexist");
    assert_eq!(s.buffers.len(), count1);
    assert_eq!(cur_buf(&s), id1);
}

#[test]
fn delete_last_buffer_is_noop() {
    let mut s = ed();
    assert_eq!(s.buffers.len(), 1);
    s.apply_action(Action::DeleteBuffer);
    assert_eq!(s.buffers.len(), 1);
}

#[test]
fn ctrl_6_via_handle_key() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    s.apply_action(Action::NextBuffer); // switch to id1
    assert_eq!(cur_buf(&s), id1);
    // Ctrl-6 should switch alternate
    let mods = KeyModifiers { ctrl: true, ..Default::default() };
    s.handle_key(&Key::Char('6'), &mods);
    assert_eq!(cur_buf(&s), BufferId(0));
}

#[test]
fn ex_bfirst_blast_parsing() {
    use crate::command_parse::parse_ex_command;
    assert_eq!(parse_ex_command("bf"), Action::FirstBuffer);
    assert_eq!(parse_ex_command("bfirst"), Action::FirstBuffer);
    assert_eq!(parse_ex_command("bl"), Action::LastBuffer);
    assert_eq!(parse_ex_command("blast"), Action::LastBuffer);
}

#[test]
fn ex_ls_parsing() {
    use crate::command_parse::parse_ex_command;
    assert_eq!(parse_ex_command("ls"), Action::ListBuffers);
    assert_eq!(parse_ex_command("buffers"), Action::ListBuffers);
}

#[test]
fn delete_updates_alternate() {
    let mut s = ed();
    let id1 = add_buf(&mut s);
    let id2 = add_buf(&mut s);
    s.apply_action(Action::SwitchBuffer(id1));
    s.apply_action(Action::SwitchBuffer(id2));
    // alternate=id1, current=id2
    assert_eq!(s.alternate_buffer, Some(id1));
    s.apply_action(Action::DeleteBuffer); // delete id2, falls back to alternate=id1
    assert_eq!(cur_buf(&s), id1);
}
