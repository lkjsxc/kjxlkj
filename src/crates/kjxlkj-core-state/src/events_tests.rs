//! Tests for event registry.
use crate::events::EventRegistry;
use crate::events_types::{EventData, EventKind};

#[test]
fn test_register_and_fire() {
    let mut reg = EventRegistry::new();
    reg.register(
        EventKind::BufferNew,
        "echo 'new buffer'".to_string(),
        None,
        None,
    );
    let cmds = reg.fire(EventKind::BufferNew, &EventData::default());
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0], "echo 'new buffer'");
}

#[test]
fn test_fire_wrong_event() {
    let mut reg = EventRegistry::new();
    reg.register(EventKind::BufferNew, "cmd".to_string(), None, None);
    let cmds =
        reg.fire(EventKind::BufferLeave, &EventData::default());
    assert!(cmds.is_empty());
}

#[test]
fn test_pattern_filter() {
    let mut reg = EventRegistry::new();
    reg.register(
        EventKind::FileType,
        "set syntax=rust".to_string(),
        Some("*.rs".to_string()),
        None,
    );
    let data_match = EventData {
        file: Some("main.rs".to_string()),
        ..Default::default()
    };
    let data_nomatch = EventData {
        file: Some("main.py".to_string()),
        ..Default::default()
    };
    assert_eq!(reg.fire(EventKind::FileType, &data_match).len(), 1);
    assert_eq!(
        reg.fire(EventKind::FileType, &data_nomatch).len(),
        0
    );
}

#[test]
fn test_clear_group() {
    let mut reg = EventRegistry::new();
    reg.register(
        EventKind::BufferNew,
        "cmd1".to_string(),
        None,
        Some("mygroup".to_string()),
    );
    reg.register(
        EventKind::BufferNew,
        "cmd2".to_string(),
        None,
        None,
    );
    reg.clear_group("mygroup");
    assert_eq!(reg.len(), 1);
}

#[test]
fn test_remove_by_id() {
    let mut reg = EventRegistry::new();
    let id = reg.register(
        EventKind::BufferNew,
        "cmd".to_string(),
        None,
        None,
    );
    assert!(reg.remove(id));
    assert!(reg.is_empty());
}

#[test]
fn test_reentry_guard() {
    let mut reg = EventRegistry::new();
    reg.set_max_depth(0);
    reg.register(
        EventKind::BufferNew,
        "cmd".to_string(),
        None,
        None,
    );
    let cmds = reg.fire(EventKind::BufferNew, &EventData::default());
    assert!(cmds.is_empty());
}

#[test]
fn test_glob_match() {
    use crate::events::glob_match;
    assert!(glob_match("*.rs", "main.rs"));
    assert!(!glob_match("*.rs", "main.py"));
    assert!(glob_match("src/*", "src/lib.rs"));
    assert!(glob_match("*", "anything"));
    assert!(glob_match("exact", "exact"));
    assert!(!glob_match("exact", "other"));
}
