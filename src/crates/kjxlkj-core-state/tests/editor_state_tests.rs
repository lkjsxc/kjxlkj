//! Integration tests for EditorState subsystems.

use kjxlkj_core_state::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Mode, Position, WindowId};

#[test]
fn create_editor_state() {
    let state = EditorState::new();
    assert_eq!(state.buffers.len(), 1);
    assert_eq!(state.windows.len(), 1);
    assert_eq!(state.mode.current(), Mode::Normal);
}

#[test]
fn add_buffer() {
    let mut state = EditorState::new();
    let id = state.alloc_buffer_id();
    let buf = TextBuffer::from_text(id, "test.txt".into(), "content\n");
    state.buffers.insert(id, buf);
    assert_eq!(state.buffers.len(), 2);
}

#[test]
fn switch_buffer() {
    let mut state = EditorState::new();
    let id = state.alloc_buffer_id();
    let buf = TextBuffer::new(id, "buf2".into());
    state.buffers.insert(id, buf);
    state.windows[0].buffer_id = id;
    assert_eq!(state.active_buffer().name(), "buf2");
}

#[test]
fn set_option_number() {
    let mut opts = EditorOptions::default();
    let action = options::parse_set_arg("tabstop=4");
    assert_eq!(action, SetAction::SetInt("tabstop".into(), 4));
    options::apply_set_action(&mut opts, action).unwrap();
    assert_eq!(opts.tabstop, 4);
}

#[test]
fn set_option_wrap() {
    let mut opts = EditorOptions::default();
    options::apply_set_action(&mut opts, SetAction::SetBool("wrap".into(), false)).unwrap();
    assert!(!opts.wrap);
}

#[test]
fn mark_set_get() {
    let mut reg = MarkRegistry::new();
    reg.set('a', BufferId(1), Position::new(5, 3));
    let m = reg.get('a', BufferId(1)).unwrap();
    assert_eq!(m.position, Position::new(5, 3));
}

#[test]
fn jump_list_push_pop() {
    let mut jl = JumpList::new();
    jl.push(JumpEntry { buffer_id: BufferId(1), position: Position::new(0, 0) });
    jl.push(JumpEntry { buffer_id: BufferId(1), position: Position::new(10, 0) });
    let e = jl.jump_back().unwrap();
    assert_eq!(e.position.line, 10);
    let e2 = jl.jump_back().unwrap();
    assert_eq!(e2.position.line, 0);
}

#[test]
fn change_list_push() {
    let mut cl = ChangeList::new();
    cl.push(ChangeListEntry { position: Position::new(3, 0) });
    assert_eq!(cl.len(), 1);
    let e = cl.older().unwrap();
    assert_eq!(e.position.line, 3);
}

#[test]
fn register_set_get() {
    let mut regs = std::collections::HashMap::new();
    registers::yank_to_register(&mut regs, registers::DEFAULT_REGISTER, "hello", false);
    let r = registers::get_register(&regs, registers::YANK_REGISTER).unwrap();
    assert_eq!(r.content, "hello");
}

#[test]
fn search_state_forward() {
    let buf = TextBuffer::from_text(BufferId(1), "t".into(), "hello world\nfoo bar\n");
    let pos = search::search_forward(&buf, "foo", Position::ZERO);
    assert_eq!(pos, Some(Position::new(1, 0)));
}

#[test]
fn search_state_backward() {
    let buf = TextBuffer::from_text(BufferId(1), "t".into(), "abc\ndef\nabc\n");
    let pos = search::search_backward(&buf, "abc", Position::new(2, 2));
    assert_eq!(pos, Some(Position::new(2, 0)));
}

#[test]
fn command_history_push() {
    let mut h = CommandHistory::new(50);
    h.push("w".into());
    h.push("q".into());
    let prev = h.prev().unwrap();
    assert_eq!(prev, "q");
}

#[test]
fn macro_record_stop() {
    let mut ms = MacroState::new();
    ms.start_recording('a');
    assert!(ms.is_recording());
    ms.record_key(kjxlkj_core_types::KeyEvent::char('x'));
    let (reg, keys) = ms.stop_recording().unwrap();
    assert_eq!(reg, 'a');
    assert_eq!(keys.len(), 1);
}

#[test]
fn viewport_scroll_delta() {
    let mut vp = ViewportState::new(24, 80);
    viewport::scroll(&mut vp, 5, 100);
    assert_eq!(vp.top_line, 5);
    viewport::scroll(&mut vp, -3, 100);
    assert_eq!(vp.top_line, 2);
}

#[test]
fn viewport_center() {
    let mut vp = ViewportState::new(20, 80);
    viewport::center_on_line(&mut vp, 50);
    assert_eq!(vp.top_line, 40);
}

#[test]
fn parse_command_quit() {
    assert_eq!(parse_command(":q").unwrap(), commands::ExCommand::Quit);
}

#[test]
fn parse_command_write() {
    assert_eq!(parse_command(":w").unwrap(), commands::ExCommand::Write(None));
}

#[test]
fn parse_command_edit() {
    match parse_command(":e main.rs").unwrap() {
        commands::ExCommand::Edit(p, false) => assert_eq!(p, "main.rs"),
        other => panic!("expected Edit, got {other:?}"),
    }
}

#[test]
fn window_split() {
    let mut state = EditorState::new();
    let wid = state.alloc_window_id();
    let bid = state.active_buffer_id();
    let win = kjxlkj_core_state::WindowState::new(wid, bid, 80, 12);
    state.windows.push(win);
    assert_eq!(state.windows.len(), 2);
}

#[test]
fn alternate_buffer_switch() {
    let mut state = EditorState::new();
    let id = state.alloc_buffer_id();
    let buf = TextBuffer::new(id, "alt".into());
    state.buffers.insert(id, buf);
    state.windows[0].buffer_id = id;
    assert_eq!(state.active_buffer().name(), "alt");
}
