use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Intent, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

#[test]
fn show_file_info_via_command() {
    let mut s = setup("hello\nworld");
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::ExCommand(":file".into()));
    let msg = s.message.as_deref().unwrap();
    assert!(msg.contains("2 lines"));
}

#[test]
fn sort_lines_via_command() {
    let mut s = setup("banana\napple\ncherry");
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::ExCommand(":sort".into()));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).starts_with("apple"));
}

#[test]
fn show_marks_empty() {
    let mut s = setup("hello");
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::ExCommand(":marks".into()));
    assert_eq!(s.message.as_deref(), Some("No marks set"));
}

#[test]
fn bdelete_rejects_last() {
    let mut s = setup("hello");
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::ExCommand(":bdelete!".into()));
    assert_eq!(s.message.as_deref(), Some("Cannot delete last buffer"));
}
