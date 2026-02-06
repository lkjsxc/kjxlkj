//! Command parsing edge cases and error handling tests.

use kjxlkj_core_types::Size;
use kjxlkj_core_state::EditorState;

fn setup() -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("line 1\nline 2\nline 3\nline 4\nline 5\n");
    s.create_window(bid);
    s
}

fn exec(state: &mut EditorState, cmd: &str) {
    kjxlkj_core_state::dispatch_intent(
        state,
        kjxlkj_core_types::Intent::ExCommand(cmd.into()),
    );
}

// ── Range parsing ─────────────────────────────────────────────────────

#[test]
fn percent_range_covers_all_lines() {
    let mut s = setup();
    exec(&mut s, ":%d");
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() <= 1);
}

#[test]
fn numeric_range_deletes_correct_lines() {
    let mut s = setup();
    exec(&mut s, ":2,3d");
    let buf = s.active_buffer().unwrap();
    let text = buf.text.line_to_string(0);
    assert_eq!(text, "line 1");
}

#[test]
fn dollar_means_last_line() {
    let mut s = setup();
    exec(&mut s, ":$");
    let win = s.active_window_state().unwrap();
    // Should move cursor to last line (5 content lines + trailing)
    assert!(win.cursor_line >= 4);
}

#[test]
fn dot_means_current_line() {
    let mut s = setup();
    s.active_window_mut().unwrap().cursor_line = 2;
    exec(&mut s, ":.d");
    let buf = s.active_buffer().unwrap();
    // Line 3 (0-indexed line 2) should be deleted
    let remaining = buf.text.line_to_string(2);
    assert_eq!(remaining, "line 4");
}

#[test]
fn range_with_offset() {
    let mut s = setup();
    s.active_window_mut().unwrap().cursor_line = 1;
    exec(&mut s, ":.+1d");
    let buf = s.active_buffer().unwrap();
    // Should delete line 3 (cursor at 1, +1 = line 2 zero-indexed = "line 3")
    let l = buf.text.line_to_string(2);
    assert_eq!(l, "line 4");
}

// ── Unknown commands produce messages ─────────────────────────────────

#[test]
fn unknown_command_sets_message() {
    let mut s = setup();
    exec(&mut s, ":nonexistent");
    assert!(s.message.is_some());
    assert!(s.message.as_ref().unwrap().contains("nonexistent"));
}

// ── Set option edge cases ─────────────────────────────────────────────

#[test]
fn set_no_args_shows_usage() {
    let mut s = setup();
    exec(&mut s, ":set");
    assert!(s.message.is_some());
}

#[test]
fn set_number_toggles() {
    let mut s = setup();
    assert!(!s.options.number);
    exec(&mut s, ":set number");
    assert!(s.options.number);
    exec(&mut s, ":set nonumber");
    assert!(!s.options.number);
}

#[test]
fn set_tabstop_numeric() {
    let mut s = setup();
    exec(&mut s, ":set tabstop=2");
    assert_eq!(s.options.tabstop, 2);
}

// ── Buffer commands ───────────────────────────────────────────────────

#[test]
fn ls_shows_buffers() {
    let mut s = setup();
    exec(&mut s, ":ls");
    assert!(s.message.is_some());
}

#[test]
fn buffer_no_args_shows_usage() {
    let mut s = setup();
    exec(&mut s, ":b");
    assert!(s.message.as_ref().unwrap().contains("Usage"));
}

#[test]
fn enew_creates_empty_buffer() {
    let mut s = setup();
    let old_count = s.buffers.len();
    exec(&mut s, ":enew");
    assert!(s.buffers.len() > old_count);
}

// ── Write commands ────────────────────────────────────────────────────

#[test]
fn write_no_path_reports_no_file() {
    let mut s = setup();
    exec(&mut s, ":w");
    // Should report no file path set
    assert!(s.message.is_some());
}

// ── Quit with unsaved changes ─────────────────────────────────────────

#[test]
fn quit_refuses_if_modified() {
    let mut s = setup();
    s.active_buffer_mut().unwrap().modified = true;
    exec(&mut s, ":q");
    assert!(!s.should_quit);
    assert!(s.message.is_some());
}

#[test]
fn quit_force_ignores_modified() {
    let mut s = setup();
    s.active_buffer_mut().unwrap().modified = true;
    exec(&mut s, ":q!");
    assert!(s.should_quit);
}

// ── Script execution ──────────────────────────────────────────────────

#[test]
fn execute_quoted_string() {
    let mut s = setup();
    exec(&mut s, ":execute \"set tabstop=3\"");
    assert_eq!(s.options.tabstop, 3);
}

#[test]
fn execute_no_args_reports_error() {
    let mut s = setup();
    exec(&mut s, ":execute");
    assert!(s.message.as_ref().unwrap().contains("E471"));
}

// ── Nohlsearch ────────────────────────────────────────────────────────

#[test]
fn nohlsearch_clears_pattern() {
    let mut s = setup();
    s.search_pattern = Some("test".into());
    exec(&mut s, ":noh");
    assert!(s.search_pattern.is_none());
}

// ── Sort ──────────────────────────────────────────────────────────────

#[test]
fn sort_reorders_lines() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("c\na\nb");
    s.create_window(bid);
    exec(&mut s, ":%sort");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "a");
    assert_eq!(buf.text.line_to_string(1), "b");
    assert_eq!(buf.text.line_to_string(2), "c");
}

// ── Map / unmap ───────────────────────────────────────────────────────

#[test]
fn nmap_creates_mapping() {
    let mut s = setup();
    exec(&mut s, ":nmap jk :quit");
    let m = s.mappings.get(kjxlkj_core_state::MappingMode::Normal, "jk");
    assert!(m.is_some());
    assert_eq!(m.unwrap().rhs, ":quit");
}

// ── Substitute ────────────────────────────────────────────────────────

#[test]
fn substitute_replaces_in_line() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("hello world");
    s.create_window(bid);
    exec(&mut s, ":s/hello/goodbye/");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "goodbye world");
}
