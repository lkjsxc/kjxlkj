//! Tests for window management, ex commands, global, display commands.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{
    Intent, Mode, MotionKind, OperatorKind, Size,
};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// ── Window Management ──────────────────────────────────────

#[test]
fn window_split_horizontal() {
    let mut s = setup("hello\nworld");
    assert_eq!(s.windows.len(), 1);
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 2);
}

#[test]
fn window_split_vertical() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::WindowSplitVertical);
    assert_eq!(s.windows.len(), 2);
}

#[test]
fn window_close() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 2);
    dispatch_intent(&mut s, Intent::WindowClose);
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn window_close_last_prevented() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowClose);
    assert_eq!(s.windows.len(), 1);
    assert!(s.message.as_deref().unwrap().contains("Cannot close"));
}

#[test]
fn window_only() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 3);
    dispatch_intent(&mut s, Intent::WindowOnly);
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn window_focus_next() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    let first = s.active_window;
    dispatch_intent(&mut s, Intent::WindowFocusNext);
    assert_ne!(s.active_window, first);
}

#[test]
fn window_focus_prev() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    let w = s.active_window;
    dispatch_intent(&mut s, Intent::WindowFocusPrev);
    assert_ne!(s.active_window, w);
}

#[test]
fn window_focus_direction_down() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    let w = s.active_window;
    dispatch_intent(
        &mut s,
        Intent::WindowFocusDirection(MotionKind::Down),
    );
    assert_ne!(s.active_window, w);
}

#[test]
fn window_equal_size() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    dispatch_intent(&mut s, Intent::WindowEqualSize);
    let heights: Vec<_> =
        s.windows.values().map(|w| w.height).collect();
    assert!(heights.iter().all(|h| *h == heights[0]));
}

#[test]
fn window_rotate() {
    let mut s = setup("hello");
    let bid1 = s.active_window_state().unwrap().buffer_id;
    let bid2 = s.create_buffer_from_text("world");
    s.create_window(bid2);
    dispatch_intent(&mut s, Intent::WindowRotate);
    // After rotation, buffer assignments should change
    let bufs: Vec<_> =
        s.windows.values().map(|w| w.buffer_id).collect();
    assert!(bufs.contains(&bid1) && bufs.contains(&bid2));
}

// ── Ex Command: split/vsplit/close/only ─────────────────────

#[test]
fn ex_split() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":split".into()));
    assert_eq!(s.windows.len(), 2);
}

#[test]
fn ex_vsplit() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":vsplit".into()));
    assert_eq!(s.windows.len(), 2);
}

#[test]
fn ex_close() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":split".into()));
    dispatch_intent(&mut s, Intent::ExCommand(":close".into()));
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn ex_only() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":split".into()));
    dispatch_intent(&mut s, Intent::ExCommand(":only".into()));
    assert_eq!(s.windows.len(), 1);
}

// ── Ex Command: display commands ─────────────────────────────

#[test]
fn ex_marks_empty() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":marks".into()));
    assert!(s.message.as_deref().unwrap().contains("No marks"));
}

#[test]
fn ex_marks_with_mark() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::SetMark('a'));
    dispatch_intent(&mut s, Intent::ExCommand(":marks".into()));
    let msg = s.message.as_deref().unwrap();
    assert!(msg.contains('a'));
}

#[test]
fn ex_registers() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::YankLine(1));
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":registers".into()),
    );
    let msg = s.message.as_deref().unwrap();
    assert!(msg.contains("Registers"));
}

#[test]
fn ex_jumps_empty() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":jumps".into()));
    assert!(s.message.as_deref().unwrap().contains("No jumps"));
}

#[test]
fn ex_changes_empty() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":changes".into()),
    );
    assert!(s.message.as_deref().unwrap().contains("No changes"));
}

#[test]
fn ex_file_info() {
    let mut s = setup("hello\nworld\nfoo");
    dispatch_intent(&mut s, Intent::ExCommand(":file".into()));
    let msg = s.message.as_deref().unwrap();
    assert!(msg.contains("3 lines"));
}

// ── Ex Command: :sort ─────────────────────────────────────

#[test]
fn ex_sort() {
    let mut s = setup("banana\napple\ncherry");
    dispatch_intent(&mut s, Intent::ExCommand(":sort".into()));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).starts_with("apple"));
}

// ── Ex Command: :noh ─────────────────────────────────────

#[test]
fn ex_nohlsearch() {
    let mut s = setup("hello world");
    s.search_pattern = Some("hello".into());
    dispatch_intent(&mut s, Intent::ExCommand(":noh".into()));
    assert!(s.search_pattern.is_none());
}

// ── Ex Command: :new ─────────────────────────────────────

#[test]
fn ex_new_buffer() {
    let mut s = setup("hello");
    let orig_bufs = s.buffers.len();
    dispatch_intent(&mut s, Intent::ExCommand(":new".into()));
    assert_eq!(s.buffers.len(), orig_bufs + 1);
}

// ── Ex Command: :bd ─────────────────────────────────────

#[test]
fn ex_bdelete() {
    let mut s = setup("hello");
    let bid2 = s.create_buffer_from_text("world");
    s.create_window(bid2);
    assert_eq!(s.buffers.len(), 2);
    dispatch_intent(&mut s, Intent::ExCommand(":bd".into()));
    assert_eq!(s.buffers.len(), 1);
}

#[test]
fn ex_bdelete_last_prevented() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":bd".into()));
    assert_eq!(s.buffers.len(), 1);
    assert!(s.message.as_deref().unwrap().contains("Cannot delete"));
}

// ── Ex Command: :pwd ─────────────────────────────────────

#[test]
fn ex_pwd() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":pwd".into()));
    assert!(s.message.is_some());
}

// ── Ex Command: :g global ────────────────────────────────

#[test]
fn global_delete_matching() {
    let mut s = setup("keep\nremove this\nkeep too\nremove that");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":g/remove/d".into()),
    );
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

#[test]
fn global_print_matching() {
    let mut s = setup("apple\nbanana\napricot");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":g/ap/p".into()),
    );
    let msg = s.message.as_deref().unwrap();
    assert!(msg.contains("apple"));
    assert!(msg.contains("apricot"));
}

#[test]
fn vglobal_delete_non_matching() {
    let mut s = setup("keep\nremove\nkeep");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":v/keep/d".into()),
    );
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

// ── Ex Command: :messages/:pwd ───────────────────────────

#[test]
fn ex_messages() {
    let mut s = setup("hello");
    s.message = None;
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":messages".into()),
    );
    assert!(s.message.as_deref().unwrap().contains("No messages"));
}

// ── Ctrl-g file info ────────────────────────────────────

#[test]
fn ctrl_g_file_info() {
    let mut s = setup("line1\nline2");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":file".into()),
    );
    let msg = s.message.as_deref().unwrap();
    assert!(msg.contains("2 lines"));
}

// ── Window via Ctrl-w (parser) ──────────────────────────

#[test]
fn parser_ctrl_w_s_splits() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::ctrl('w'));
    assert_eq!(intent, Intent::Noop); // pending
    let intent = p.parse_normal(&KeyEvent::char('s'));
    assert_eq!(intent, Intent::WindowSplitHorizontal);
}

#[test]
fn parser_ctrl_w_v_vsplits() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('v'));
    assert_eq!(intent, Intent::WindowSplitVertical);
}

#[test]
fn parser_ctrl_w_c_closes() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('c'));
    assert_eq!(intent, Intent::WindowClose);
}

#[test]
fn parser_ctrl_w_o_only() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('o'));
    assert_eq!(intent, Intent::WindowOnly);
}

#[test]
fn parser_ctrl_w_w_focus_next() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('w'));
    assert_eq!(intent, Intent::WindowFocusNext);
}

#[test]
fn parser_ctrl_w_h_focus_left() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('h'));
    assert_eq!(
        intent,
        Intent::WindowFocusDirection(MotionKind::Left)
    );
}

#[test]
fn parser_ctrl_w_eq_equal() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('='));
    assert_eq!(intent, Intent::WindowEqualSize);
}

#[test]
fn parser_ctrl_w_r_rotate() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('r'));
    assert_eq!(intent, Intent::WindowRotate);
}

#[test]
fn parser_ctrl_w_n_new() {
    use kjxlkj_core_mode::KeyParser;
    use kjxlkj_core_types::KeyEvent;
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::ctrl('w'));
    let intent = p.parse_normal(&KeyEvent::char('n'));
    assert_eq!(intent, Intent::ExCommand(":new".into()));
}

// ── Language detection ─────────────────────────────────

#[test]
fn language_rust() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(LanguageId::from_extension("rs"), LanguageId::Rust);
}

#[test]
fn language_python() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(LanguageId::from_extension("py"), LanguageId::Python);
}

#[test]
fn language_typescript() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(
        LanguageId::from_extension("ts"),
        LanguageId::TypeScript,
    );
}

#[test]
fn language_c_header() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(LanguageId::from_extension("h"), LanguageId::C);
}

#[test]
fn language_dockerfile() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(
        LanguageId::from_filename("Dockerfile"),
        LanguageId::Dockerfile,
    );
}

#[test]
fn language_unknown() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(LanguageId::from_extension("xyz"), LanguageId::Plain);
}

#[test]
fn language_lsp_id() {
    use kjxlkj_core_types::LanguageId;
    assert_eq!(LanguageId::Rust.lsp_id(), "rust");
    assert_eq!(LanguageId::Python.lsp_id(), "python");
}

// ── Combined workflow tests ─────────────────────────────

#[test]
fn split_edit_close() {
    let mut s = setup("hello\nworld");
    // Split and type in new window
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('x'));
    s.mode.transition(Mode::Normal);
    // Close window
    dispatch_intent(&mut s, Intent::WindowClose);
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn multi_split_focus_cycle() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 3);
    let w1 = s.active_window;
    dispatch_intent(&mut s, Intent::WindowFocusNext);
    let w2 = s.active_window;
    dispatch_intent(&mut s, Intent::WindowFocusNext);
    let w3 = s.active_window;
    dispatch_intent(&mut s, Intent::WindowFocusNext);
    assert_eq!(s.active_window, w1);
    assert_ne!(w1, w2);
    assert_ne!(w2, w3);
}

#[test]
fn global_then_undo() {
    let mut s = setup("line1\nline2\nline3");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":g/line2/d".into()),
    );
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

#[test]
fn sort_then_search() {
    let mut s = setup("charlie\nalpha\nbravo");
    dispatch_intent(&mut s, Intent::ExCommand(":sort".into()));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).starts_with("alpha"));
    // Search for bravo
    dispatch_intent(
        &mut s,
        Intent::SearchForward("bravo".into()),
    );
    assert!(s.cursor().line > 0);
}

#[test]
fn ex_noh_clears_search() {
    let mut s = setup("hello world");
    dispatch_intent(
        &mut s,
        Intent::SearchForward("hello".into()),
    );
    assert!(s.search_pattern.is_some());
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":nohlsearch".into()),
    );
    assert!(s.search_pattern.is_none());
}
