//! Tests for UI module.

use super::*;
use kjxlkj_core_types::{BufferMeta, BufferName, BufferVersion, Cursor, LineEnding, Mode, BufferId};

#[test]
fn test_status_left() {
    let meta = BufferMeta {
        id: BufferId::new(),
        name: BufferName::new("test.txt"),
        path: None,
        modified: true,
        version: BufferVersion::new(0),
        line_ending: LineEnding::Lf,
    };

    let left = build_status_left(Mode::Normal, &meta);
    assert!(left.contains("NORMAL"));
    assert!(left.contains("test.txt"));
    assert!(left.contains("[+]"));
}

#[test]
fn test_status_right() {
    let cursor = Cursor::new(5, 10);
    let right = build_status_right(&cursor, 100);
    assert!(right.contains("6:11")); // 1-indexed
    assert!(right.contains("100"));
}

#[test]
fn test_minimal_snapshot() {
    let snap = EditorSnapshot::minimal();
    assert_eq!(snap.mode, Mode::Normal);
}
