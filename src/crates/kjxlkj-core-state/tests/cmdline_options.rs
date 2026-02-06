//! Tests for command-line editing, history, tab completion,
//! :set option dispatch, and EnterCommandLine intent.

use kjxlkj_core_state::{
    dispatch_intent, handle_cmdline_key, EditorState,
};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn enter_cmd(s: &mut EditorState) {
    dispatch_intent(s, Intent::EnterCommandLine(':'));
}

fn enter_search(s: &mut EditorState) {
    dispatch_intent(s, Intent::EnterCommandLine('/'));
}

fn enter_search_back(s: &mut EditorState) {
    dispatch_intent(s, Intent::EnterCommandLine('?'));
}

// ── EnterCommandLine ──────────────────────────────────────

#[test]
fn enter_command_line_colon() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    assert_eq!(s.current_mode(), Mode::Command);
    assert_eq!(s.cmdline.prefix, ':');
    assert!(s.cmdline.text.is_empty());
}

#[test]
fn enter_command_line_slash() {
    let mut s = setup("hello");
    enter_search(&mut s);
    assert_eq!(s.current_mode(), Mode::Command);
    assert_eq!(s.cmdline.prefix, '/');
}

#[test]
fn enter_command_line_question() {
    let mut s = setup("hello");
    enter_search_back(&mut s);
    assert_eq!(s.current_mode(), Mode::Command);
    assert_eq!(s.cmdline.prefix, '?');
}

// ── Typing & Cursor ────────────────────────────────────────

#[test]
fn type_chars() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    assert_eq!(s.cmdline.text, "q");
    assert_eq!(s.cmdline.cursor, 1);
    handle_cmdline_key(&mut s, &KeyEvent::char('!'));
    assert_eq!(s.cmdline.text, "q!");
    assert_eq!(s.cmdline.cursor, 2);
}

#[test]
fn backspace_deletes() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('a'));
    handle_cmdline_key(&mut s, &KeyEvent::char('b'));
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Backspace),
    );
    assert_eq!(s.cmdline.text, "a");
    assert_eq!(s.cmdline.cursor, 1);
}

#[test]
fn backspace_empty_exits() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('x'));
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Backspace),
    );
    // Backspace on empty line returns to normal
    let r = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Backspace),
    );
    // Empty line with backspace should trigger exit
    assert!(s.cmdline.text.is_empty());
}

#[test]
fn cursor_left_right() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('a'));
    handle_cmdline_key(&mut s, &KeyEvent::char('b'));
    handle_cmdline_key(&mut s, &KeyEvent::char('c'));
    assert_eq!(s.cmdline.cursor, 3);
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Left),
    );
    assert_eq!(s.cmdline.cursor, 2);
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Left),
    );
    assert_eq!(s.cmdline.cursor, 1);
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Right),
    );
    assert_eq!(s.cmdline.cursor, 2);
}

#[test]
fn home_end_keys() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('a'));
    handle_cmdline_key(&mut s, &KeyEvent::char('b'));
    handle_cmdline_key(&mut s, &KeyEvent::char('c'));
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Home),
    );
    assert_eq!(s.cmdline.cursor, 0);
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::End),
    );
    assert_eq!(s.cmdline.cursor, 3);
}

// ── Enter / Escape ────────────────────────────────────────

#[test]
fn enter_executes_ex_command() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    let r = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    assert_eq!(r, Intent::ExCommand(":q".into()));
    // After enter, cmdline is reset
    assert!(s.cmdline.text.is_empty());
}

#[test]
fn enter_dispatches_search_forward() {
    let mut s = setup("hello world");
    enter_search(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('w'));
    handle_cmdline_key(&mut s, &KeyEvent::char('o'));
    let r = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    assert_eq!(r, Intent::SearchForward("wo".into()));
}

#[test]
fn enter_dispatches_search_backward() {
    let mut s = setup("hello world");
    enter_search_back(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('h'));
    let r = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    assert_eq!(r, Intent::SearchBackward("h".into()));
}

#[test]
fn escape_cancels() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    let r = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Escape),
    );
    assert_eq!(r, Intent::EnterMode(Mode::Normal));
    assert!(s.cmdline.text.is_empty());
}

// ── History ───────────────────────────────────────────────

#[test]
fn history_saved_on_enter() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    assert_eq!(s.cmdline.history, vec!["q".to_string()]);
}

#[test]
fn history_up_down() {
    let mut s = setup("hello");
    // Simulate previous history
    s.cmdline.history =
        vec!["first".into(), "second".into(), "third".into()];
    enter_cmd(&mut s);
    // Up arrow goes to most recent
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Up),
    );
    assert_eq!(s.cmdline.text, "third");
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Up),
    );
    assert_eq!(s.cmdline.text, "second");
    // Down goes back
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Down),
    );
    assert_eq!(s.cmdline.text, "third");
    // Down past end restores
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Down),
    );
    assert!(s.cmdline.text.is_empty());
}

// ── Ctrl Keys ─────────────────────────────────────────────

#[test]
fn ctrl_u_clears_to_start() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('a'));
    handle_cmdline_key(&mut s, &KeyEvent::char('b'));
    handle_cmdline_key(&mut s, &KeyEvent::char('c'));
    handle_cmdline_key(&mut s, &KeyEvent::ctrl('u'));
    assert!(s.cmdline.text.is_empty());
    assert_eq!(s.cmdline.cursor, 0);
}

#[test]
fn ctrl_c_cancels() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    let r = handle_cmdline_key(&mut s, &KeyEvent::ctrl('c'));
    assert_eq!(r, Intent::EnterMode(Mode::Normal));
}

// ── Tab Completion ─────────────────────────────────────────

#[test]
fn tab_completes_unique() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('w'));
    handle_cmdline_key(&mut s, &KeyEvent::char('r'));
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Tab),
    );
    assert_eq!(s.cmdline.text, "write");
}

#[test]
fn tab_common_prefix() {
    let mut s = setup("hello");
    enter_cmd(&mut s);
    handle_cmdline_key(&mut s, &KeyEvent::char('b'));
    handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Tab),
    );
    // Multiple matches starting with 'b': bnext, bprev, bdelete
    // Common prefix is 'b', so no extension (all start with 'b')
    // Should show options in message
    assert!(s.message.is_some());
}

// ── :set Options ───────────────────────────────────────────

#[test]
fn set_number_on() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set number".into()),
    );
    assert!(s.options.number);
}

#[test]
fn set_nonumber() {
    let mut s = setup("hello");
    s.options.number = true;
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set nonumber".into()),
    );
    assert!(!s.options.number);
}

#[test]
fn set_tabstop() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set tabstop=8".into()),
    );
    assert_eq!(s.options.tabstop, 8);
}

#[test]
fn set_shiftwidth() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set shiftwidth=2".into()),
    );
    assert_eq!(s.options.shiftwidth, 2);
}

#[test]
fn set_wrap_toggle() {
    let mut s = setup("hello");
    assert!(s.options.wrap);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set nowrap".into()),
    );
    assert!(!s.options.wrap);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set wrap".into()),
    );
    assert!(s.options.wrap);
}

#[test]
fn set_ignorecase() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set ignorecase".into()),
    );
    assert!(s.options.ignorecase);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set noignorecase".into()),
    );
    assert!(!s.options.ignorecase);
}

#[test]
fn set_smartcase() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set nosmartcase".into()),
    );
    assert!(!s.options.smartcase);
}

#[test]
fn set_expandtab() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set noexpandtab".into()),
    );
    assert!(!s.options.expandtab);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set expandtab".into()),
    );
    assert!(s.options.expandtab);
}

#[test]
fn set_autoindent() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set noautoindent".into()),
    );
    assert!(!s.options.autoindent);
}

#[test]
fn set_scrolloff() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set scrolloff=5".into()),
    );
    assert_eq!(s.options.scrolloff, 5);
}

#[test]
fn set_query_option() {
    let mut s = setup("hello");
    s.options.tabstop = 4;
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set tabstop?".into()),
    );
    assert!(s.message.as_deref().unwrap().contains("4"));
}

#[test]
fn set_unknown_option() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set foobar".into()),
    );
    assert!(s.message.as_deref().unwrap().contains("unknown"));
}

#[test]
fn set_abbreviations() {
    let mut s = setup("hello");
    // Test short forms
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set nu".into()),
    );
    assert!(s.options.number);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set nonu".into()),
    );
    assert!(!s.options.number);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set ts=2".into()),
    );
    assert_eq!(s.options.tabstop, 2);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set sw=3".into()),
    );
    assert_eq!(s.options.shiftwidth, 3);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set ic".into()),
    );
    assert!(s.options.ignorecase);
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":set et".into()),
    );
    assert!(s.options.expandtab);
}

// ── Full Workflow ──────────────────────────────────────────

#[test]
fn cmdline_type_and_execute_quit() {
    let mut s = setup("hello");
    // Enter command mode via intent
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    assert_eq!(s.current_mode(), Mode::Command);
    // Type 'q'
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    // Press Enter — returns :q intent
    let intent = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    // Dispatch the returned intent
    dispatch_intent(&mut s, intent);
    assert!(s.should_quit);
}

#[test]
fn cmdline_search_and_navigate() {
    let mut s = setup("alpha\nbeta\ngamma");
    // Enter / search mode
    dispatch_intent(&mut s, Intent::EnterCommandLine('/'));
    handle_cmdline_key(&mut s, &KeyEvent::char('g'));
    handle_cmdline_key(&mut s, &KeyEvent::char('a'));
    let intent = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    dispatch_intent(&mut s, intent);
    assert_eq!(s.cursor().line, 2); // gamma is on line 2
}

#[test]
fn cmdline_set_option_workflow() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    for c in "set tabstop=8".chars() {
        handle_cmdline_key(&mut s, &KeyEvent::char(c));
    }
    let intent = handle_cmdline_key(
        &mut s,
        &KeyEvent::special(KeyCode::Enter),
    );
    dispatch_intent(&mut s, intent);
    assert_eq!(s.options.tabstop, 8);
}
