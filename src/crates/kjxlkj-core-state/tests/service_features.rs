//! Tests for expanded service + session + session commands.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Intent, KeyEvent, Size};

fn make_state() -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("hello world\nfoo bar\nbaz qux\n");
    let wid = s.create_window(bid);
    s.active_window = Some(wid);
    s
}

fn ex(s: &mut EditorState, cmd: &str) {
    kjxlkj_core_state::dispatch_intent(s, Intent::ExCommand(cmd.into()));
}

// ──────────── Session commands ────────────

#[test]
fn mksession_default() {
    let mut s = make_state();
    ex(&mut s, ":mksession");
    let msg = s.message.as_ref().unwrap();
    assert!(msg.contains("Session saved"));
    assert!(msg.contains("Session.vim"));
}

#[test]
fn mksession_custom_path() {
    let mut s = make_state();
    ex(&mut s, ":mks /tmp/my_session.vim");
    let msg = s.message.as_ref().unwrap();
    assert!(msg.contains("/tmp/my_session.vim"));
}

#[test]
fn oldfiles_empty() {
    let mut s = make_state();
    ex(&mut s, ":oldfiles");
    assert_eq!(s.message.as_ref().unwrap(), "No old files");
}

#[test]
fn oldfiles_with_entries() {
    let mut s = make_state();
    s.recent_files.push("/home/user/a.txt", 10, 0);
    s.recent_files.push("/home/user/b.rs", 5, 3);
    ex(&mut s, ":ol");
    let msg = s.message.as_ref().unwrap();
    assert!(msg.contains("b.rs"));
    assert!(msg.contains("a.txt"));
}

// ──────────── Swap/undo file paths ────────────

#[test]
fn swap_file_path_encoding() {
    let p = kjxlkj_core_state::SwapFile::path_for(
        "/home/user/project/main.rs",
        std::path::Path::new("/tmp/swap"),
    );
    let s = p.to_string_lossy();
    assert!(s.ends_with(".swp"));
    assert!(s.contains("%home%user%project%main.rs"));
}

#[test]
fn undo_file_path_encoding() {
    let p = kjxlkj_core_state::UndoFile::path_for(
        "/home/user/file.rs",
        std::path::Path::new("/tmp/undo"),
    );
    let s = p.to_string_lossy();
    assert!(s.ends_with(".un~"));
}

// ──────────── Workspace type ────────────

#[test]
fn workspace_creation() {
    let ws = kjxlkj_core_state::Workspace {
        name: "my-project".into(),
        folders: vec![],
    };
    assert_eq!(ws.name, "my-project");
    assert!(ws.folders.is_empty());
}

// ──────────── Session layout ────────────

#[test]
fn session_layout_split() {
    use kjxlkj_core_state::{SessionLayout, SplitDirection};
    let layout = SessionLayout::Split {
        direction: SplitDirection::Vertical,
        children: vec![SessionLayout::Single, SessionLayout::Single],
    };
    assert!(matches!(layout, SessionLayout::Split { .. }));
}

// ──────────── Messages command ────────────

#[test]
fn messages_no_messages() {
    let mut s = make_state();
    s.message = None;
    ex(&mut s, ":messages");
    assert_eq!(s.message.as_ref().unwrap(), "No messages");
}

#[test]
fn messages_preserves_existing() {
    let mut s = make_state();
    s.message = Some("hello".into());
    ex(&mut s, ":mes");
    assert_eq!(s.message.as_ref().unwrap(), "hello");
}

// ──────────── Pwd/cd ────────────

#[test]
fn pwd_shows_directory() {
    let mut s = make_state();
    ex(&mut s, ":pwd");
    assert!(s.message.is_some());
}

// ──────────── Feature command wiring ────────────

#[test]
fn feature_commands_are_wired() {
    let mut s = make_state();
    ex(&mut s, ":explorer");
    assert!(s.message.as_ref().unwrap().contains("Explorer opened"));
    assert!(s.active_buffer().unwrap().scratch);

    ex(&mut s, ":terminal");
    assert!(s.message.as_ref().unwrap().contains("Terminal opened"));
    assert!(s.active_buffer().unwrap().scratch);

    ex(&mut s, ":find");
    assert!(s.message.as_ref().unwrap().contains("pending"));

    ex(&mut s, ":livegrep");
    assert!(s.message.as_ref().unwrap().contains("pending"));

    ex(&mut s, ":undotree");
    assert!(s.message.as_ref().unwrap().contains("undo entries:"));
}
