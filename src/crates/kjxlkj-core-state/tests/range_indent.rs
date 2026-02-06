//! Tests for command ranges, auto-indent, statusline, and new features.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{InsertPosition, Intent, KeyEvent, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn feed_key(state: &mut EditorState, key: KeyEvent) {
    let intent = state.parser.parse_normal(&key);
    dispatch_intent(state, intent);
}

fn feed_char(state: &mut EditorState, c: char) {
    feed_key(state, KeyEvent::char(c));
}

fn dispatch_ex(state: &mut EditorState, cmd: &str) {
    dispatch_intent(state, Intent::ExCommand(cmd.into()));
}

// ── Range parsing ────────────────────────────────

#[test]
fn range_percent_substitute() {
    let mut s = setup("aaa\naaa\naaa");
    dispatch_ex(&mut s, ":%s/aaa/bbb/g");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "bbb");
    assert_eq!(buf.text.line_to_string(1).trim(), "bbb");
    assert_eq!(buf.text.line_to_string(2).trim(), "bbb");
}

#[test]
fn range_line_numbers_substitute() {
    let mut s = setup("aaa\naaa\naaa\naaa");
    dispatch_ex(&mut s, ":2,3s/aaa/bbb");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "aaa");
    assert_eq!(buf.text.line_to_string(1).trim(), "bbb");
    assert_eq!(buf.text.line_to_string(2).trim(), "bbb");
    assert_eq!(buf.text.line_to_string(3).trim(), "aaa");
}

#[test]
fn range_single_line_substitute() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":2s/bbb/xxx");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "aaa");
    assert_eq!(buf.text.line_to_string(1).trim(), "xxx");
    assert_eq!(buf.text.line_to_string(2).trim(), "ccc");
}

#[test]
fn range_delete_lines() {
    let mut s = setup("a\nb\nc\nd\ne");
    dispatch_ex(&mut s, ":2,3d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 3);
    assert_eq!(buf.text.line_to_string(0).trim(), "a");
    assert_eq!(buf.text.line_to_string(1).trim(), "d");
    assert_eq!(buf.text.line_to_string(2).trim(), "e");
}

#[test]
fn range_delete_single_line() {
    let mut s = setup("a\nb\nc");
    dispatch_ex(&mut s, ":2d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
    assert_eq!(buf.text.line_to_string(0).trim(), "a");
    assert_eq!(buf.text.line_to_string(1).trim(), "c");
}

#[test]
fn range_yank_lines() {
    let mut s = setup("alpha\nbeta\ngamma");
    dispatch_ex(&mut s, ":1,2y");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("2 lines yanked"));
}

#[test]
fn range_dollar_address() {
    let mut s = setup("a\nb\nc");
    dispatch_ex(&mut s, ":$d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

#[test]
fn range_dot_address() {
    let mut s = setup("a\nb\nc");
    // Cursor is on line 0 by default
    dispatch_ex(&mut s, ":.d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
    assert_eq!(buf.text.line_to_string(0).trim(), "b");
}

#[test]
fn range_offset_address() {
    let mut s = setup("a\nb\nc\nd\ne");
    dispatch_ex(&mut s, ":.+1,.+2d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 3);
    assert_eq!(buf.text.line_to_string(0).trim(), "a");
    assert_eq!(buf.text.line_to_string(1).trim(), "d");
}

#[test]
fn goto_line_via_range() {
    let mut s = setup("a\nb\nc\nd\ne");
    dispatch_ex(&mut s, ":4");
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 3); // line 4 = index 3
}

#[test]
fn goto_line_still_works_after_range_parsing() {
    let mut s = setup("line1\nline2\nline3\nline4\nline5");
    dispatch_ex(&mut s, ":5");
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 4);
}

// ── Auto-indent ──────────────────────────────────

#[test]
fn autoindent_copies_whitespace() {
    let mut s = setup("    hello");
    s.options.autoindent = true;
    // Enter insert mode at end of line
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::AfterCursor,
    ));
    // Press Enter
    dispatch_intent(&mut s, Intent::InsertNewline);
    let buf = s.active_buffer().unwrap();
    // Line 1 should start with 4 spaces
    let line1 = buf.text.line_to_string(1);
    assert!(line1.starts_with("    "), "expected indent, got: {:?}", line1);
}

#[test]
fn autoindent_tabs() {
    let mut s = setup("\thello");
    s.options.autoindent = true;
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::AfterCursor,
    ));
    dispatch_intent(&mut s, Intent::InsertNewline);
    let buf = s.active_buffer().unwrap();
    let line1 = buf.text.line_to_string(1);
    assert!(line1.starts_with('\t'), "expected tab indent, got: {:?}", line1);
}

#[test]
fn no_autoindent_when_disabled() {
    let mut s = setup("    hello");
    s.options.autoindent = false;
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::EndOfLine,
    ));
    dispatch_intent(&mut s, Intent::InsertNewline);
    let buf = s.active_buffer().unwrap();
    let line1 = buf.text.line_to_string(1);
    assert!(!line1.starts_with(' '), "expected no indent, got: {:?}", line1);
}

#[test]
fn autoindent_cursor_position() {
    let mut s = setup("  test");
    s.options.autoindent = true;
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::AfterCursor,
    ));
    dispatch_intent(&mut s, Intent::InsertNewline);
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 1);
    assert_eq!(win.cursor_col, 2); // after the 2 spaces
}

// ── :d and :y without ranges ─────────────────────

#[test]
fn delete_command_no_range_deletes_current_line() {
    let mut s = setup("alpha\nbeta\ngamma");
    dispatch_ex(&mut s, ":d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
    assert_eq!(buf.text.line_to_string(0).trim(), "beta");
}

#[test]
fn yank_command_no_range_yanks_current_line() {
    let mut s = setup("alpha\nbeta");
    dispatch_ex(&mut s, ":y");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("1 lines yanked") || msg.contains("1 line yanked"));
}

// ── Insert Ctrl-t / Ctrl-d (indent/dedent) ───────

#[test]
fn insert_ctrl_t_indents() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.starts_with("    ") || line.starts_with('\t'));
}

#[test]
fn insert_ctrl_d_dedents() {
    let mut s = setup("    hello");
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    dispatch_intent(&mut s, Intent::Indent(false, 1));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    // Should have less indentation
    assert!(
        !line.starts_with("    "),
        "expected dedented, got: {:?}", line
    );
}

// ── :set option effects ──────────────────────────

#[test]
fn set_autoindent_on() {
    let mut s = setup("test");
    s.options.autoindent = false;
    dispatch_ex(&mut s, ":set autoindent");
    assert!(s.options.autoindent);
}

#[test]
fn set_noautoindent() {
    let mut s = setup("test");
    s.options.autoindent = true;
    dispatch_ex(&mut s, ":set noautoindent");
    assert!(!s.options.autoindent);
}

#[test]
fn set_smartindent_on() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":set smartindent");
    assert!(s.options.smartindent);
}

// ── Autocommands ─────────────────────────────────

#[test]
fn autocmd_add_and_display() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":autocmd BufRead *.rs echo hello");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("BufRead"));
}

#[test]
fn autocmd_clear_all() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":autocmd BufRead *.rs echo hi");
    dispatch_ex(&mut s, ":autocmd!");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("cleared"));
}

// ── Mappings ─────────────────────────────────────

#[test]
fn map_and_show() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":nmap jk :nop");
    dispatch_ex(&mut s, ":nmap");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("jk"));
}

#[test]
fn unmap_removes() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":nmap jk :nop");
    dispatch_ex(&mut s, ":nunmap jk");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("unmapped"));
}

#[test]
fn mapclear_clears_all() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":nmap a b");
    dispatch_ex(&mut s, ":nmap c d");
    dispatch_ex(&mut s, ":nmapclear");
    dispatch_ex(&mut s, ":nmap");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("No mapping"));
}

// ── Filetype ─────────────────────────────────────

#[test]
fn filetype_query() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":filetype");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("filetype="));
}

#[test]
fn filetype_set() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":filetype rs");
    let buf = s.active_buffer().unwrap();
    assert_eq!(format!("{}", buf.language), "rust");
}

// ── Config / :source ─────────────────────────────

#[test]
fn source_nonexistent_file() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":source /nonexistent/file.conf");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(!msg.is_empty());
}

// ── :pwd / :cd ───────────────────────────────────

#[test]
fn pwd_shows_directory() {
    let mut s = setup("test");
    dispatch_ex(&mut s, ":pwd");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(!msg.is_empty());
}

// ── Global substitute ────────────────────────────

#[test]
fn percent_substitute_global_flag() {
    let mut s = setup("foo bar\nfoo baz\nfoo qux");
    dispatch_ex(&mut s, ":%s/foo/FOO/g");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "FOO bar");
    assert_eq!(buf.text.line_to_string(1).trim(), "FOO baz");
    assert_eq!(buf.text.line_to_string(2).trim(), "FOO qux");
}

// ── Comprehensive workflows ─────────────────────

#[test]
fn workflow_set_then_indent() {
    let mut s = setup("hello\nworld");
    dispatch_ex(&mut s, ":set autoindent");
    dispatch_intent(&mut s, Intent::EnterInsert(
        InsertPosition::AfterCursor,
    ));
    dispatch_intent(&mut s, Intent::InsertNewline);
    // autoindent is on, but original line has no indent
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(1);
    assert!(!line.starts_with(' '));
}

#[test]
fn workflow_range_delete_then_yank() {
    let mut s = setup("a\nb\nc\nd\ne");
    dispatch_ex(&mut s, ":2,3d");
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 3);
    dispatch_ex(&mut s, ":1,2y");
    let msg = s.message.as_deref().unwrap_or("");
    assert!(msg.contains("2 lines yanked"));
}
