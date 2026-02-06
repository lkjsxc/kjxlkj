//! End-to-end integration tests: full keystroke → parse → dispatch → verify
//! pipeline. Tests simulate real editing sessions through the public API.

use kjxlkj_core::state::*;
use kjxlkj_core::types::*;

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

/// Route a key event through the correct parser method based on current mode.
fn parse_key(state: &mut EditorState, key: &KeyEvent) -> Intent {
    match state.current_mode() {
        Mode::Normal | Mode::InsertNormal => state.parser.parse_normal(key),
        Mode::Insert => state.parser.parse_insert(key),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
            state.parser.parse_visual(key)
        }
        Mode::Command => state.parser.parse_command(key),
        Mode::Replace => state.parser.parse_replace(key),
        _ => Intent::Noop,
    }
}

/// Feed a sequence of characters through the parser and dispatch results.
fn feed_keys(state: &mut EditorState, keys: &str) {
    for ch in keys.chars() {
        let key = KeyEvent::char(ch);
        let intent = parse_key(state, &key);
        dispatch_intent(state, intent);
    }
}

/// Feed a single special key.
fn feed_special(state: &mut EditorState, code: KeyCode) {
    let key = KeyEvent::special(code);
    let intent = parse_key(state, &key);
    dispatch_intent(state, intent);
}

/// Feed a ctrl+key combo.
fn feed_ctrl(state: &mut EditorState, c: char) {
    let key = KeyEvent::ctrl(c);
    let intent = parse_key(state, &key);
    dispatch_intent(state, intent);
}

fn buf_text(state: &EditorState) -> String {
    state.active_buffer().unwrap().text.text()
}

fn buf_line(state: &EditorState, line: usize) -> String {
    state.active_buffer().unwrap().text.line_to_string(line)
}

// ──────────── Basic motion sequences ────────────

#[test]
fn e2e_hjkl_navigation() {
    let mut s = setup("abc\ndef\nghi");
    feed_keys(&mut s, "ll"); // right twice
    assert_eq!(s.cursor().col, 2);
    feed_keys(&mut s, "j"); // down
    assert_eq!(s.cursor(), Position::new(1, 2));
    feed_keys(&mut s, "h"); // left
    assert_eq!(s.cursor(), Position::new(1, 1));
    feed_keys(&mut s, "k"); // up
    assert_eq!(s.cursor(), Position::new(0, 1));
}

#[test]
fn e2e_zero_and_dollar() {
    let mut s = setup("hello world");
    feed_keys(&mut s, "$"); // end of line
    assert_eq!(s.cursor().col, 10);
    feed_keys(&mut s, "0"); // start of line
    assert_eq!(s.cursor().col, 0);
}

#[test]
fn e2e_caret_first_nonblank() {
    let mut s = setup("   hello");
    feed_keys(&mut s, "^"); // first non-blank
    assert_eq!(s.cursor().col, 3);
}

#[test]
fn e2e_w_word_motion() {
    let mut s = setup("hello world test");
    feed_keys(&mut s, "w"); // next word
    assert_eq!(s.cursor().col, 6);
    feed_keys(&mut s, "w"); // next word
    assert_eq!(s.cursor().col, 12);
}

#[test]
fn e2e_b_word_backward() {
    let mut s = setup("hello world test");
    feed_keys(&mut s, "ww"); // move to "test"
    feed_keys(&mut s, "b"); // back to "world"
    assert_eq!(s.cursor().col, 6);
}

#[test]
fn e2e_gg_and_G() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    feed_keys(&mut s, "G"); // go to last line
    assert_eq!(s.cursor().line, 3);
    feed_keys(&mut s, "gg"); // go to first line
    assert_eq!(s.cursor().line, 0);
}

#[test]
fn e2e_count_motion() {
    let mut s = setup("abcdefghij");
    feed_keys(&mut s, "5l"); // right 5
    assert_eq!(s.cursor().col, 5);
}

#[test]
fn e2e_count_down() {
    let mut s = setup("a\nb\nc\nd\ne\nf");
    feed_keys(&mut s, "3j"); // down 3
    assert_eq!(s.cursor().line, 3);
}

// ──────────── Insert mode ────────────

#[test]
fn e2e_i_insert_text() {
    let mut s = setup("world");
    feed_keys(&mut s, "i"); // enter insert mode
    assert_eq!(s.current_mode(), Mode::Insert);
    // In insert mode, parser returns InsertChar for normal chars
    feed_keys(&mut s, "hello ");
    assert_eq!(buf_line(&s, 0), "hello world");
}

#[test]
fn e2e_a_append_text() {
    let mut s = setup("hell");
    feed_keys(&mut s, "$"); // go to last char
    feed_keys(&mut s, "a"); // append after cursor
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_keys(&mut s, "o");
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(buf_line(&s, 0), "hello");
}

#[test]
fn e2e_A_append_eol() {
    let mut s = setup("hello");
    feed_keys(&mut s, "A"); // append at end of line
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_keys(&mut s, "!");
    assert_eq!(buf_line(&s, 0), "hello!");
}

#[test]
fn e2e_I_insert_first_nonblank() {
    let mut s = setup("   hello");
    feed_keys(&mut s, "llll"); // move cursor right
    feed_keys(&mut s, "I"); // insert at first non-blank
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().col, 3);
}

#[test]
fn e2e_escape_from_insert() {
    let mut s = setup("hello");
    feed_keys(&mut s, "i"); // enter insert
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(s.current_mode(), Mode::Normal);
}

#[test]
fn e2e_insert_and_escape_back() {
    let mut s = setup("hllo");
    feed_keys(&mut s, "l"); // move to col 1
    feed_keys(&mut s, "i"); // insert mode
    feed_keys(&mut s, "e"); // type 'e'
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(s.current_mode(), Mode::Normal);
    assert_eq!(buf_line(&s, 0), "hello");
}

// ──────────── Delete operations ────────────

#[test]
fn e2e_x_delete_char() {
    let mut s = setup("hello");
    feed_keys(&mut s, "x"); // delete char under cursor
    assert_eq!(buf_line(&s, 0), "ello");
}

#[test]
fn e2e_x_count() {
    let mut s = setup("hello");
    feed_keys(&mut s, "x"); // delete 1 char
    feed_keys(&mut s, "x"); // delete another
    feed_keys(&mut s, "x"); // delete third
    assert_eq!(buf_line(&s, 0), "lo");
}

#[test]
fn e2e_dd_delete_line() {
    let mut s = setup("aaa\nbbb\nccc");
    feed_keys(&mut s, "dd"); // delete first line
    assert_eq!(buf_line(&s, 0), "bbb");
}

#[test]
fn e2e_dd_middle_line() {
    let mut s = setup("aaa\nbbb\nccc");
    feed_keys(&mut s, "j"); // move to line 2
    feed_keys(&mut s, "dd"); // delete
    assert_eq!(buf_line(&s, 0), "aaa");
    assert_eq!(buf_line(&s, 1), "ccc");
}

#[test]
fn e2e_dw_delete_word() {
    let mut s = setup("hello world");
    // Use dispatch directly since parser operator pending is a multi-step interaction
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1),
    );
    let t = buf_text(&s);
    // Deletes from cursor through motion target (word boundary region)
    assert!(!t.starts_with("hello"));
}

#[test]
fn e2e_D_delete_to_end() {
    let mut s = setup("hello world");
    feed_keys(&mut s, "ll"); // move to col 2
    feed_keys(&mut s, "D"); // delete to end
    assert_eq!(buf_line(&s, 0), "he");
}

// ──────────── Change operations ────────────

#[test]
fn e2e_cc_change_line() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "cc"); // change line
    assert_eq!(s.current_mode(), Mode::Insert);
}

#[test]
fn e2e_C_change_to_end() {
    let mut s = setup("hello world");
    feed_keys(&mut s, "ll"); // col 2
    feed_keys(&mut s, "C"); // change to end
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(buf_line(&s, 0), "he");
}

// ──────────── Yank/Paste ────────────

#[test]
fn e2e_yy_yank_line() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "yy"); // yank line
    // Buffer unchanged
    assert_eq!(buf_line(&s, 0), "hello");
    assert!(s.registers.unnamed_text().is_some());
}

#[test]
fn e2e_p_paste_after() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "yy"); // yank line 1
    feed_keys(&mut s, "p"); // paste after
    let count = s.active_buffer().unwrap().text.line_count();
    assert!(count >= 3); // original 2 + pasted 1
}

#[test]
fn e2e_P_paste_before() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "yy"); // yank
    feed_keys(&mut s, "j"); // down
    feed_keys(&mut s, "P"); // paste before
    let count = s.active_buffer().unwrap().text.line_count();
    assert!(count >= 3);
}

// ──────────── Open line ────────────

#[test]
fn e2e_o_open_below() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "o"); // open line below
    assert_eq!(s.current_mode(), Mode::Insert);
    let count = s.active_buffer().unwrap().text.line_count();
    assert_eq!(count, 3);
}

#[test]
fn e2e_O_open_above() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "O"); // open line above
    assert_eq!(s.current_mode(), Mode::Insert);
    let count = s.active_buffer().unwrap().text.line_count();
    assert_eq!(count, 3);
}

// ──────────── Join lines ────────────

#[test]
fn e2e_J_join() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "J"); // join lines
    assert_eq!(buf_line(&s, 0), "hello world");
}

// ──────────── Replace ────────────

#[test]
fn e2e_r_replace_char() {
    let mut s = setup("hello");
    feed_keys(&mut s, "rX"); // replace with X
    assert_eq!(buf_line(&s, 0), "Xello");
    assert_eq!(s.current_mode(), Mode::Normal);
}

// ──────────── Toggle case ────────────

#[test]
fn e2e_tilde_toggle() {
    let mut s = setup("hello");
    feed_keys(&mut s, "~"); // toggle case
    assert_eq!(
        s.active_buffer().unwrap().text.char_at(Position::new(0, 0)),
        Some('H')
    );
}

#[test]
fn e2e_tilde_multiple() {
    let mut s = setup("hello");
    feed_keys(&mut s, "~~~~~"); // toggle all 5
    assert_eq!(buf_line(&s, 0), "HELLO");
}

// ──────────── Indent / Outdent ────────────

#[test]
fn e2e_indent_normal() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    assert_eq!(buf_line(&s, 0), "    hello");
}

#[test]
fn e2e_outdent_normal() {
    let mut s = setup("    hello");
    dispatch_intent(&mut s, Intent::Indent(false, 1));
    assert_eq!(buf_line(&s, 0), "hello");
}

// ──────────── Substitute ────────────

#[test]
fn e2e_s_substitute() {
    let mut s = setup("hello");
    feed_keys(&mut s, "s"); // substitute char
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(buf_line(&s, 0), "ello");
}

#[test]
fn e2e_S_substitute_line() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "S"); // substitute line
    assert_eq!(s.current_mode(), Mode::Insert);
}

// ──────────── Scroll ────────────

#[test]
fn e2e_ctrl_d_scroll() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    feed_ctrl(&mut s, 'd');
    assert!(s.cursor().line > 0);
}

#[test]
fn e2e_ctrl_u_scroll() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    feed_ctrl(&mut s, 'd'); // scroll down first
    let line_after_down = s.cursor().line;
    feed_ctrl(&mut s, 'u'); // scroll back up
    assert!(s.cursor().line < line_after_down);
}

// ──────────── Command mode ────────────

#[test]
fn e2e_enter_command_mode() {
    let mut s = setup("hello");
    feed_keys(&mut s, ":"); // enter command mode
    assert_eq!(s.current_mode(), Mode::Command);
}

#[test]
fn e2e_quit_command() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
    assert!(s.should_quit);
}

// ──────────── Visual mode ────────────

#[test]
fn e2e_enter_visual() {
    let mut s = setup("hello");
    feed_keys(&mut s, "v"); // enter visual mode
    assert_eq!(s.current_mode(), Mode::Visual);
}

#[test]
fn e2e_enter_visual_line() {
    let mut s = setup("hello");
    feed_keys(&mut s, "V"); // enter visual line mode
    assert_eq!(s.current_mode(), Mode::VisualLine);
}

// ──────────── Complex sequences ────────────

#[test]
fn e2e_delete_word_insert() {
    let mut s = setup("hello world");
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1),
    );
    let t = buf_text(&s);
    // Partial word deletion across boundary
    assert!(!t.starts_with("hello"));
}

#[test]
fn e2e_navigate_and_insert() {
    let mut s = setup("abc\ndef\nghi");
    feed_keys(&mut s, "jl"); // line 1, col 1
    feed_keys(&mut s, "i");  // insert mode
    feed_keys(&mut s, "X");  // insert X
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(buf_line(&s, 1), "dXef");
}

#[test]
fn e2e_multiple_deletes() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    feed_keys(&mut s, "dd"); // delete line 1
    feed_keys(&mut s, "dd"); // delete line 2 (was "bbb")
    assert_eq!(buf_line(&s, 0), "ccc");
}

#[test]
fn e2e_yank_paste_duplicate() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "yy"); // yank "hello"
    feed_keys(&mut s, "G");  // go to last line
    feed_keys(&mut s, "p");  // paste after
    let count = s.active_buffer().unwrap().text.line_count();
    assert!(count >= 3);
}

#[test]
fn e2e_insert_newline_and_text() {
    let mut s = setup("hello");
    feed_keys(&mut s, "A"); // append at end
    dispatch_intent(&mut s, Intent::InsertNewline);
    feed_keys(&mut s, "world");
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(buf_line(&s, 1), "world");
}

#[test]
fn e2e_backspace_in_insert() {
    let mut s = setup("hello");
    feed_keys(&mut s, "A"); // append at end
    feed_keys(&mut s, "X"); // type X
    dispatch_intent(&mut s, Intent::DeleteCharBefore);
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(buf_line(&s, 0), "hello");
}

#[test]
fn e2e_replace_multiple() {
    let mut s = setup("aaa");
    feed_keys(&mut s, "rX"); // replace first char
    feed_keys(&mut s, "l"); // move right
    feed_keys(&mut s, "rY"); // replace second char
    assert_eq!(
        s.active_buffer().unwrap().text.char_at(Position::new(0, 0)),
        Some('X')
    );
    assert_eq!(
        s.active_buffer().unwrap().text.char_at(Position::new(0, 1)),
        Some('Y')
    );
}

// ──────────── Edge cases ────────────

#[test]
fn e2e_empty_buffer_insert() {
    let mut s = setup("");
    feed_keys(&mut s, "i");
    feed_keys(&mut s, "hello");
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(buf_line(&s, 0), "hello");
}

#[test]
fn e2e_motion_clamp_right() {
    let mut s = setup("abc");
    feed_keys(&mut s, "llllllll"); // try to go way past end
    assert!(s.cursor().col <= 2);
}

#[test]
fn e2e_motion_clamp_left() {
    let mut s = setup("abc");
    feed_keys(&mut s, "hhhh"); // try to go before start
    assert_eq!(s.cursor().col, 0);
}

#[test]
fn e2e_motion_clamp_up() {
    let mut s = setup("abc\ndef");
    feed_keys(&mut s, "kkkk"); // try to go above first line
    assert_eq!(s.cursor().line, 0);
}

#[test]
fn e2e_single_char_buffer() {
    let mut s = setup("x");
    feed_keys(&mut s, "x"); // delete the one char
    // Buffer should still have at least an empty line
    let count = s.active_buffer().unwrap().text.line_count();
    assert!(count >= 1);
}

#[test]
fn e2e_long_line_navigation() {
    let mut s = setup(&"x".repeat(200));
    feed_keys(&mut s, "$"); // end of line
    assert!(s.cursor().col > 100);
    feed_keys(&mut s, "0"); // start
    assert_eq!(s.cursor().col, 0);
}

// ──────────── Mode transitions ────────────

#[test]
fn e2e_normal_to_insert_to_normal() {
    let mut s = setup("abc");
    assert_eq!(s.current_mode(), Mode::Normal);
    feed_keys(&mut s, "i");
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(s.current_mode(), Mode::Normal);
}

#[test]
fn e2e_normal_to_visual_to_normal() {
    let mut s = setup("abc");
    feed_keys(&mut s, "v");
    assert_eq!(s.current_mode(), Mode::Visual);
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(s.current_mode(), Mode::Normal);
}

#[test]
fn e2e_normal_to_command_to_normal() {
    let mut s = setup("abc");
    feed_keys(&mut s, ":");
    assert_eq!(s.current_mode(), Mode::Command);
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(s.current_mode(), Mode::Normal);
}

#[test]
fn e2e_insert_positions() {
    // Test all insert entry points
    let mut s = setup("  hello");

    // i - before cursor
    feed_keys(&mut s, "i");
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_special(&mut s, KeyCode::Escape);

    // a - after cursor
    feed_keys(&mut s, "a");
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_special(&mut s, KeyCode::Escape);

    // A - end of line
    feed_keys(&mut s, "A");
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_special(&mut s, KeyCode::Escape);

    // I - first non-blank
    feed_keys(&mut s, "I");
    assert_eq!(s.current_mode(), Mode::Insert);
    feed_special(&mut s, KeyCode::Escape);
}

// ──────────── Multi-line editing ────────────

#[test]
fn e2e_delete_multiple_lines() {
    let mut s = setup("aaa\nbbb\nccc\nddd\neee");
    feed_keys(&mut s, "3dd"); // delete 3 lines
    assert_eq!(buf_line(&s, 0), "ddd");
}

#[test]
fn e2e_open_line_type_escape() {
    let mut s = setup("hello\nworld");
    feed_keys(&mut s, "o"); // open below
    feed_keys(&mut s, "inserted");
    feed_special(&mut s, KeyCode::Escape);
    assert_eq!(buf_line(&s, 1), "inserted");
    assert_eq!(buf_line(&s, 2), "world");
}

#[test]
fn e2e_join_multiple() {
    let mut s = setup("a\nb\nc");
    feed_keys(&mut s, "J"); // join a+b
    feed_keys(&mut s, "J"); // join result+c
    assert_eq!(s.active_buffer().unwrap().text.line_count(), 1);
}

// ──────────── Miscellaneous ────────────

#[test]
fn e2e_parser_reset_on_escape() {
    let mut s = setup("hello");
    feed_keys(&mut s, "d"); // start operator pending
    feed_special(&mut s, KeyCode::Escape); // cancel
    assert_eq!(s.current_mode(), Mode::Normal);
    // Buffer should be unchanged
    assert_eq!(buf_line(&s, 0), "hello");
}

#[test]
fn e2e_noop_keys() {
    let mut s = setup("hello");
    let before = buf_text(&s);
    feed_keys(&mut s, "q"); // macro record (noop)
    assert_eq!(buf_text(&s), before);
}

#[test]
fn e2e_numeric_prefix_motion() {
    let mut s = setup("a\nb\nc\nd\ne\nf\ng\nh\ni\nj");
    feed_keys(&mut s, "5j"); // down 5
    assert_eq!(s.cursor().line, 5);
    feed_keys(&mut s, "3k"); // up 3
    assert_eq!(s.cursor().line, 2);
}
