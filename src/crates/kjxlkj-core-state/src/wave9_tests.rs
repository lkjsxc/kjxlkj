//! Wave 9 tests: sentence/tag text objects, visual range,
//! option/buffer completion, change marks, hlsearch, macro-register.
use kjxlkj_core_types::{Key, Mode, VisualKind};

use crate::editor::EditorState;

fn ed() -> EditorState {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "Hello world. Foo bar.\nSecond line.\n");
    e
}

fn buf_text(e: &EditorState) -> String {
    let bid = e.current_buffer_id();
    e.buffers.get(bid).unwrap().content.to_string()
}

#[test]
fn sentence_inner_selects_sentence() {
    let mut e = ed();
    // dis = delete inner sentence from cursor
    for k in [Key::char('d'), Key::char('i'), Key::char('s')] {
        e.handle_key(k);
    }
    // Should delete "Hello world." from the start.
    let t = buf_text(&e);
    assert!(!t.starts_with("Hello"));
}

#[test]
fn tag_inner_selects_content() {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.html", "<div>hello</div>\n");
    // Move cursor inside "hello" — col 5 is 'h'
    e.windows.focused_mut().cursor.grapheme = 5;
    for k in [Key::char('d'), Key::char('i'), Key::char('t')] {
        e.handle_key(k);
    }
    let t = buf_text(&e);
    assert!(t.contains("<div></div>"));
}

#[test]
fn tag_around_includes_tags() {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.html", "<div>hello</div>\n");
    e.windows.focused_mut().cursor.grapheme = 5;
    for k in [Key::char('d'), Key::char('a'), Key::char('t')] {
        e.handle_key(k);
    }
    let t = buf_text(&e);
    assert!(!t.contains("<div>"));
}

#[test]
fn visual_colon_inserts_range() {
    let mut e = ed();
    e.handle_key(Key::char('v'));
    assert!(matches!(e.mode, Mode::Visual(VisualKind::Char)));
    e.handle_key(Key::char(':'));
    assert!(matches!(e.mode, Mode::Command(_)));
    assert_eq!(e.cmdline.content, "'<,'>");
}

#[test]
fn option_completion_for_set() {
    let mut e = ed();
    e.cmdline.open(':');
    e.cmdline.content = "set hl".to_string();
    e.cmdline.cursor_pos = 6;
    e.mode = Mode::Command(kjxlkj_core_types::CommandKind::Ex);
    e.cmdline_complete_next();
    assert!(e.cmdline.content.contains("hlsearch"));
}

#[test]
fn buffer_completion_for_b() {
    let mut e = ed();
    e.cmdline.open(':');
    e.cmdline.content = "b ".to_string();
    e.cmdline.cursor_pos = 2;
    e.mode = Mode::Command(kjxlkj_core_types::CommandKind::Ex);
    e.cmdline_complete_next();
    let cs = &e.cmdline.completion;
    assert!(!cs.candidates.is_empty());
}

#[test]
fn change_marks_set_on_delete() {
    let mut e = ed();
    // dd sets [ ] marks
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('d'));
    let bid = e.current_buffer_id().0 as usize;
    assert!(e.marks.get('[', bid).is_some());
    assert!(e.marks.get(']', bid).is_some());
}

#[test]
fn hlsearch_populates_ranges() {
    let mut e = ed();
    e.search.pattern = Some("world".to_string());
    e.search.active = true;
    e.options
        .set("hlsearch", crate::options::OptionValue::Bool(true));
    let snap = e.snapshot();
    assert!(!snap.search.highlight_ranges.is_empty());
    // First match should be on line 0
    assert_eq!(snap.search.highlight_ranges[0].0, 0);
}

#[test]
fn macro_register_unification() {
    let mut e = ed();
    // Record macro: qa iX <Esc> q
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('a'));
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('X'));
    e.handle_key(Key::esc());
    e.handle_key(Key::char('q'));
    // Register 'a' should have content from macro
    use kjxlkj_core_edit::RegisterName;
    let reg = e.registers.get(RegisterName::Named('a'));
    assert!(reg.is_some());
    let content = &reg.unwrap().content;
    assert!(content.contains('X'));
}
