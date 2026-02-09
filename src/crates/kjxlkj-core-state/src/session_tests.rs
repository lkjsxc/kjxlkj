use crate::session::{SessionData, SessionFile, SessionLayout, SessionManager};
use std::path::PathBuf;

#[test]
fn test_serialize_deserialize_roundtrip() {
    let data = SessionData {
        files: vec![SessionFile {
            path: PathBuf::from("/tmp/test.rs"),
            cursor_line: 42,
            cursor_col: 10,
            was_modified: false,
            local_marks: Vec::new(),
        }],
        cwd: Some(PathBuf::from("/home/user")),
        layout: SessionLayout::Single,
        marks: vec![('A', 5, 3)],
        active_buffer: 0,
        tab_count: 1,
        active_tab: 0,
        tab_layouts: Vec::new(),
    };

    let serialized = SessionManager::serialize(&data);
    let restored = SessionManager::deserialize(&serialized);

    assert_eq!(restored.files.len(), 1);
    assert_eq!(restored.files[0].path, PathBuf::from("/tmp/test.rs"));
    assert_eq!(restored.files[0].cursor_line, 42);
    assert_eq!(restored.files[0].cursor_col, 10);
    assert_eq!(restored.cwd, Some(PathBuf::from("/home/user")));
    assert_eq!(restored.active_buffer, 0);
    assert_eq!(restored.marks.len(), 1);
    assert_eq!(restored.marks[0], ('A', 5, 3));
}

#[test]
fn test_deserialize_empty() {
    let data = SessionManager::deserialize("");
    assert!(data.files.is_empty());
    assert!(data.cwd.is_none());
}

#[test]
fn test_deserialize_comments() {
    let input = "# comment\ncwd /tmp\n# another comment\n";
    let data = SessionManager::deserialize(input);
    assert_eq!(data.cwd, Some(PathBuf::from("/tmp")));
}
