//! Unit tests for core types.

use crate::*;

#[test]
fn all_types_are_exported() {
    // Verify all public types are accessible
    let _id = BufferId::new(1);
    let _name = BufferName::new("test");
    let _version = BufferVersion::new(0);
    let _cursor = Cursor::origin();
    let _shape = CursorShape::Block;
    let _mode = Mode::Normal;
    let _pos = LineCol::origin();
    let _byte = ByteOffset::new(0);
    let _char = CharOffset::new(0);
}

#[test]
fn types_are_serializable() {
    let id = BufferId::new(42);
    let json = serde_json::to_string(&id).unwrap();
    let parsed: BufferId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, parsed);
}
