//! Comprehensive tests for kjxlkj-core-types.

use crate::{BufferId, BufferName, Cursor, EditorEvent, KeyEvent, Mode, Modifier, Position, Range};

// ============================================================================
// Position tests
// ============================================================================

#[test]
fn position_new() {
    let pos = Position::new(5, 10);
    assert_eq!(pos.line, 5);
    assert_eq!(pos.column, 10);
}

#[test]
fn position_origin() {
    let pos = Position::origin();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn position_default() {
    let pos = Position::default();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn position_equality() {
    let a = Position::new(1, 2);
    let b = Position::new(1, 2);
    let c = Position::new(1, 3);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn position_ordering_same_line() {
    let a = Position::new(5, 2);
    let b = Position::new(5, 10);
    assert!(a < b);
    assert!(b > a);
}

#[test]
fn position_ordering_different_lines() {
    let a = Position::new(3, 100);
    let b = Position::new(5, 0);
    assert!(a < b);
}

#[test]
fn position_ordering_equal() {
    let a = Position::new(3, 5);
    let b = Position::new(3, 5);
    assert!(a <= b);
    assert!(a >= b);
}

#[test]
fn position_clone() {
    let original = Position::new(10, 20);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn position_debug() {
    let pos = Position::new(5, 10);
    let debug_str = format!("{:?}", pos);
    assert!(debug_str.contains("Position"));
}

#[test]
fn position_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Position::new(1, 2));
    set.insert(Position::new(1, 2));
    set.insert(Position::new(3, 4));
    assert_eq!(set.len(), 2);
}

#[test]
fn position_max_values() {
    let pos = Position::new(usize::MAX, usize::MAX);
    assert_eq!(pos.line, usize::MAX);
    assert_eq!(pos.column, usize::MAX);
}

// ============================================================================
// Range tests
// ============================================================================

#[test]
fn range_new() {
    let start = Position::new(1, 0);
    let end = Position::new(2, 5);
    let range = Range::new(start, end);
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

#[test]
fn range_default() {
    let range = Range::default();
    assert_eq!(range.start, Position::origin());
    assert_eq!(range.end, Position::origin());
}

#[test]
fn range_is_empty_true() {
    let pos = Position::new(5, 10);
    let range = Range::new(pos, pos);
    assert!(range.is_empty());
}

#[test]
fn range_is_empty_false() {
    let start = Position::new(1, 0);
    let end = Position::new(1, 5);
    let range = Range::new(start, end);
    assert!(!range.is_empty());
}

#[test]
fn range_contains_start_inclusive() {
    let range = Range::new(Position::new(1, 5), Position::new(1, 10));
    assert!(range.contains(Position::new(1, 5)));
}

#[test]
fn range_contains_end_exclusive() {
    let range = Range::new(Position::new(1, 5), Position::new(1, 10));
    assert!(!range.contains(Position::new(1, 10)));
}

#[test]
fn range_contains_middle() {
    let range = Range::new(Position::new(1, 5), Position::new(1, 10));
    assert!(range.contains(Position::new(1, 7)));
}

#[test]
fn range_contains_before_start() {
    let range = Range::new(Position::new(1, 5), Position::new(1, 10));
    assert!(!range.contains(Position::new(1, 4)));
}

#[test]
fn range_clone() {
    let original = Range::new(Position::new(1, 0), Position::new(2, 5));
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn range_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Range::new(Position::new(0, 0), Position::new(1, 0)));
    set.insert(Range::new(Position::new(0, 0), Position::new(1, 0)));
    set.insert(Range::new(Position::new(0, 0), Position::new(2, 0)));
    assert_eq!(set.len(), 2);
}

// ============================================================================
// Cursor tests
// ============================================================================

#[test]
fn cursor_origin() {
    let cursor = Cursor::origin();
    assert_eq!(cursor.position, Position::origin());
    assert_eq!(cursor.target_column, None);
}

#[test]
fn cursor_at() {
    let cursor = Cursor::at(5, 10);
    assert_eq!(cursor.position.line, 5);
    assert_eq!(cursor.position.column, 10);
    assert_eq!(cursor.target_column, None);
}

#[test]
fn cursor_default() {
    let cursor = Cursor::default();
    assert_eq!(cursor.position, Position::origin());
    assert_eq!(cursor.target_column, None);
}

#[test]
fn cursor_line() {
    let cursor = Cursor::at(7, 3);
    assert_eq!(cursor.line(), 7);
}

#[test]
fn cursor_column() {
    let cursor = Cursor::at(7, 3);
    assert_eq!(cursor.column(), 3);
}

#[test]
fn cursor_with_target_column() {
    let mut cursor = Cursor::at(5, 10);
    cursor.target_column = Some(20);
    assert_eq!(cursor.target_column, Some(20));
}

#[test]
fn cursor_equality() {
    let a = Cursor::at(1, 2);
    let b = Cursor::at(1, 2);
    let c = Cursor::at(1, 3);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn cursor_clone() {
    let mut original = Cursor::at(5, 10);
    original.target_column = Some(15);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn cursor_debug() {
    let cursor = Cursor::at(3, 7);
    let debug_str = format!("{:?}", cursor);
    assert!(debug_str.contains("Cursor"));
}

// ============================================================================
// Mode tests
// ============================================================================

#[test]
fn mode_default_is_normal() {
    let mode = Mode::default();
    assert_eq!(mode, Mode::Normal);
}

#[test]
fn mode_equality() {
    assert_eq!(Mode::Insert, Mode::Insert);
    assert_ne!(Mode::Insert, Mode::Normal);
}

#[test]
fn mode_all_variants_exist() {
    let modes = [
        Mode::Normal,
        Mode::Insert,
        Mode::Visual,
        Mode::VisualLine,
        Mode::VisualBlock,
        Mode::Command,
        Mode::Search,
        Mode::Replace,
    ];
    assert_eq!(modes.len(), 8);
}

#[test]
fn mode_is_visual_true() {
    assert!(Mode::Visual.is_visual());
    assert!(Mode::VisualLine.is_visual());
    assert!(Mode::VisualBlock.is_visual());
}

#[test]
fn mode_is_visual_false() {
    assert!(!Mode::Normal.is_visual());
    assert!(!Mode::Insert.is_visual());
    assert!(!Mode::Command.is_visual());
    assert!(!Mode::Search.is_visual());
    assert!(!Mode::Replace.is_visual());
}

#[test]
fn mode_is_end_inclusive_true() {
    assert!(Mode::Normal.is_end_inclusive());
    assert!(Mode::Visual.is_end_inclusive());
    assert!(Mode::VisualLine.is_end_inclusive());
    assert!(Mode::VisualBlock.is_end_inclusive());
}

#[test]
fn mode_is_end_inclusive_false() {
    assert!(!Mode::Insert.is_end_inclusive());
    assert!(!Mode::Command.is_end_inclusive());
    assert!(!Mode::Search.is_end_inclusive());
    assert!(!Mode::Replace.is_end_inclusive());
}

#[test]
fn mode_indicator_normal() {
    assert_eq!(Mode::Normal.indicator(), "NORMAL");
}

#[test]
fn mode_indicator_insert() {
    assert_eq!(Mode::Insert.indicator(), "INSERT");
}

#[test]
fn mode_indicator_visual() {
    assert_eq!(Mode::Visual.indicator(), "VISUAL");
}

#[test]
fn mode_indicator_visual_line() {
    assert_eq!(Mode::VisualLine.indicator(), "V-LINE");
}

#[test]
fn mode_indicator_visual_block() {
    assert_eq!(Mode::VisualBlock.indicator(), "V-BLOCK");
}

#[test]
fn mode_indicator_command() {
    assert_eq!(Mode::Command.indicator(), "COMMAND");
}

#[test]
fn mode_indicator_search() {
    assert_eq!(Mode::Search.indicator(), "SEARCH");
}

#[test]
fn mode_indicator_replace() {
    assert_eq!(Mode::Replace.indicator(), "REPLACE");
}

#[test]
fn mode_clone() {
    let mode = Mode::Insert;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

#[test]
fn mode_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Mode::Normal);
    set.insert(Mode::Normal);
    set.insert(Mode::Insert);
    assert_eq!(set.len(), 2);
}

// ============================================================================
// BufferId tests
// ============================================================================

#[test]
fn buffer_id_new() {
    let id = BufferId::new(42);
    assert_eq!(id.0, 42);
}

#[test]
fn buffer_id_zero() {
    let id = BufferId::new(0);
    assert_eq!(id.0, 0);
}

#[test]
fn buffer_id_max() {
    let id = BufferId::new(u64::MAX);
    assert_eq!(id.0, u64::MAX);
}

#[test]
fn buffer_id_equality() {
    let a = BufferId::new(1);
    let b = BufferId::new(1);
    let c = BufferId::new(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn buffer_id_clone() {
    let original = BufferId::new(100);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn buffer_id_debug() {
    let id = BufferId::new(42);
    let debug_str = format!("{:?}", id);
    assert!(debug_str.contains("BufferId"));
}

#[test]
fn buffer_id_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(BufferId::new(1));
    set.insert(BufferId::new(1));
    set.insert(BufferId::new(2));
    assert_eq!(set.len(), 2);
}

// ============================================================================
// BufferName tests
// ============================================================================

#[test]
fn buffer_name_new_string() {
    let name = BufferName::new("test.txt".to_string());
    assert_eq!(name.as_str(), "test.txt");
}

#[test]
fn buffer_name_new_str() {
    let name = BufferName::new("config.rs");
    assert_eq!(name.as_str(), "config.rs");
}

#[test]
fn buffer_name_unnamed() {
    let name = BufferName::unnamed();
    assert_eq!(name.as_str(), "[No Name]");
}

#[test]
fn buffer_name_empty() {
    let name = BufferName::new("");
    assert_eq!(name.as_str(), "");
}

#[test]
fn buffer_name_display() {
    let name = BufferName::new("test.txt");
    assert_eq!(format!("{}", name), "test.txt");
}

#[test]
fn buffer_name_equality() {
    let a = BufferName::new("file.txt");
    let b = BufferName::new("file.txt");
    let c = BufferName::new("other.txt");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn buffer_name_clone() {
    let original = BufferName::new("document.md");
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn buffer_name_debug() {
    let name = BufferName::new("test.rs");
    let debug_str = format!("{:?}", name);
    assert!(debug_str.contains("BufferName"));
}

#[test]
fn buffer_name_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(BufferName::new("a.txt"));
    set.insert(BufferName::new("a.txt"));
    set.insert(BufferName::new("b.txt"));
    assert_eq!(set.len(), 2);
}

#[test]
fn buffer_name_unicode() {
    let name = BufferName::new("文档.txt");
    assert_eq!(name.as_str(), "文档.txt");
    assert_eq!(format!("{}", name), "文档.txt");
}

// ============================================================================
// Modifier tests
// ============================================================================

#[test]
fn modifier_none() {
    let m = Modifier::NONE;
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn modifier_ctrl() {
    let m = Modifier::CTRL;
    assert!(m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn modifier_default() {
    let m = Modifier::default();
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
    assert_eq!(m, Modifier::NONE);
}

#[test]
fn modifier_custom() {
    let m = Modifier {
        ctrl: true,
        alt: true,
        shift: false,
    };
    assert!(m.ctrl);
    assert!(m.alt);
    assert!(!m.shift);
}

#[test]
fn modifier_all() {
    let m = Modifier {
        ctrl: true,
        alt: true,
        shift: true,
    };
    assert!(m.ctrl);
    assert!(m.alt);
    assert!(m.shift);
}

#[test]
fn modifier_equality() {
    let a = Modifier::CTRL;
    let b = Modifier::CTRL;
    let c = Modifier::NONE;
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn modifier_clone() {
    let original = Modifier::CTRL;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn modifier_debug() {
    let m = Modifier::CTRL;
    let debug_str = format!("{:?}", m);
    assert!(debug_str.contains("Modifier"));
}

#[test]
fn modifier_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Modifier::NONE);
    set.insert(Modifier::NONE);
    set.insert(Modifier::CTRL);
    assert_eq!(set.len(), 2);
}

// ============================================================================
// KeyEvent tests
// ============================================================================

#[test]
fn key_event_char() {
    let key = KeyEvent::Char('a', Modifier::NONE);
    assert!(matches!(key, KeyEvent::Char('a', _)));
}

#[test]
fn key_event_char_with_ctrl() {
    let key = KeyEvent::Char('c', Modifier::CTRL);
    if let KeyEvent::Char(c, m) = key {
        assert_eq!(c, 'c');
        assert!(m.ctrl);
    } else {
        panic!("Expected Char variant");
    }
}

#[test]
fn key_event_escape() {
    let key = KeyEvent::Escape;
    assert!(matches!(key, KeyEvent::Escape));
}

#[test]
fn key_event_enter() {
    let key = KeyEvent::Enter;
    assert!(matches!(key, KeyEvent::Enter));
}

#[test]
fn key_event_backspace() {
    let key = KeyEvent::Backspace;
    assert!(matches!(key, KeyEvent::Backspace));
}

#[test]
fn key_event_tab() {
    let key = KeyEvent::Tab;
    assert!(matches!(key, KeyEvent::Tab));
}

#[test]
fn key_event_arrows() {
    assert!(matches!(KeyEvent::Left, KeyEvent::Left));
    assert!(matches!(KeyEvent::Right, KeyEvent::Right));
    assert!(matches!(KeyEvent::Up, KeyEvent::Up));
    assert!(matches!(KeyEvent::Down, KeyEvent::Down));
}

#[test]
fn key_event_navigation() {
    assert!(matches!(KeyEvent::Home, KeyEvent::Home));
    assert!(matches!(KeyEvent::End, KeyEvent::End));
    assert!(matches!(KeyEvent::PageUp, KeyEvent::PageUp));
    assert!(matches!(KeyEvent::PageDown, KeyEvent::PageDown));
}

#[test]
fn key_event_delete() {
    let key = KeyEvent::Delete;
    assert!(matches!(key, KeyEvent::Delete));
}

#[test]
fn key_event_equality() {
    let a = KeyEvent::Char('x', Modifier::NONE);
    let b = KeyEvent::Char('x', Modifier::NONE);
    let c = KeyEvent::Char('y', Modifier::NONE);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn key_event_modifier_matters() {
    let a = KeyEvent::Char('c', Modifier::NONE);
    let b = KeyEvent::Char('c', Modifier::CTRL);
    assert_ne!(a, b);
}

#[test]
fn key_event_clone() {
    let original = KeyEvent::Char('z', Modifier::CTRL);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn key_event_debug() {
    let key = KeyEvent::Char('a', Modifier::NONE);
    let debug_str = format!("{:?}", key);
    assert!(debug_str.contains("Char"));
}

#[test]
fn key_event_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(KeyEvent::Enter);
    set.insert(KeyEvent::Enter);
    set.insert(KeyEvent::Escape);
    assert_eq!(set.len(), 2);
}

#[test]
fn key_event_unicode() {
    let key = KeyEvent::Char('日', Modifier::NONE);
    if let KeyEvent::Char(c, _) = key {
        assert_eq!(c, '日');
    } else {
        panic!("Expected Char variant");
    }
}

// ============================================================================
// EditorEvent tests
// ============================================================================

#[test]
fn editor_event_key() {
    let event = EditorEvent::Key(KeyEvent::Escape);
    assert!(matches!(event, EditorEvent::Key(KeyEvent::Escape)));
}

#[test]
fn editor_event_resize() {
    let event = EditorEvent::Resize(80, 24);
    if let EditorEvent::Resize(w, h) = event {
        assert_eq!(w, 80);
        assert_eq!(h, 24);
    } else {
        panic!("Expected Resize variant");
    }
}

#[test]
fn editor_event_quit() {
    let event = EditorEvent::Quit;
    assert!(matches!(event, EditorEvent::Quit));
}

#[test]
fn editor_event_clone() {
    let original = EditorEvent::Key(KeyEvent::Enter);
    let cloned = original.clone();
    assert!(matches!(cloned, EditorEvent::Key(KeyEvent::Enter)));
}

#[test]
fn editor_event_resize_clone() {
    let original = EditorEvent::Resize(100, 50);
    let cloned = original.clone();
    if let EditorEvent::Resize(w, h) = cloned {
        assert_eq!(w, 100);
        assert_eq!(h, 50);
    } else {
        panic!("Expected Resize variant");
    }
}

#[test]
fn editor_event_debug() {
    let event = EditorEvent::Quit;
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("Quit"));
}

#[test]
fn editor_event_resize_bounds() {
    let min_event = EditorEvent::Resize(0, 0);
    let max_event = EditorEvent::Resize(u16::MAX, u16::MAX);
    if let EditorEvent::Resize(w, h) = min_event {
        assert_eq!(w, 0);
        assert_eq!(h, 0);
    }
    if let EditorEvent::Resize(w, h) = max_event {
        assert_eq!(w, u16::MAX);
        assert_eq!(h, u16::MAX);
    }
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn range_from_cursor_positions() {
    let start_cursor = Cursor::at(0, 0);
    let end_cursor = Cursor::at(5, 10);
    let range = Range::new(start_cursor.position, end_cursor.position);
    assert_eq!(range.start.line, 0);
    assert_eq!(range.end.line, 5);
}

#[test]
fn cursor_position_in_range() {
    let range = Range::new(Position::new(0, 0), Position::new(10, 0));
    let cursor = Cursor::at(5, 5);
    assert!(range.contains(cursor.position));
}

#[test]
fn cursor_position_not_in_range() {
    let range = Range::new(Position::new(0, 0), Position::new(5, 0));
    let cursor = Cursor::at(10, 0);
    assert!(!range.contains(cursor.position));
}

#[test]
fn buffer_id_and_name_association() {
    let id = BufferId::new(1);
    let name = BufferName::new("main.rs");
    assert_eq!(id.0, 1);
    assert_eq!(name.as_str(), "main.rs");
}
