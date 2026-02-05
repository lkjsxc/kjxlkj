//! Comprehensive tests for kjxlkj-core-types.

use kjxlkj_core_types::*;

mod position_tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.column, 10);
    }

    #[test]
    fn test_position_origin() {
        let pos = Position::origin();
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_position_ordering() {
        let a = Position::new(1, 5);
        let b = Position::new(1, 10);
        let c = Position::new(2, 0);
        assert!(a < b);
        assert!(b < c);
        assert!(a < c);
    }

    #[test]
    fn test_position_equality() {
        let a = Position::new(3, 7);
        let b = Position::new(3, 7);
        assert_eq!(a, b);
    }

    #[test]
    fn test_position_debug() {
        let pos = Position::new(10, 20);
        let debug = format!("{:?}", pos);
        assert!(debug.len() > 0);
    }

    #[test]
    fn test_position_clone() {
        let a = Position::new(5, 5);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_position_copy() {
        let a = Position::new(1, 2);
        let b = a;
        assert_eq!(a.line, b.line);
    }
}

mod range_tests {
    use super::*;

    #[test]
    fn test_range_new() {
        let r = Range::new(Position::new(0, 0), Position::new(1, 5));
        assert_eq!(r.start.line, 0);
        assert_eq!(r.end.line, 1);
    }

    #[test]
    fn test_range_point() {
        let r = Range::point(Position::new(5, 10));
        assert_eq!(r.start, Position::new(5, 10));
        assert_eq!(r.end, Position::new(5, 11)); // half-open interval
    }

    #[test]
    fn test_range_line() {
        let r = Range::line(3);
        assert_eq!(r.start.line, 3);
        assert_eq!(r.start.column, 0);
        assert_eq!(r.end.line, 4); // half-open interval, ends at next line
        assert_eq!(r.end.column, 0);
    }

    #[test]
    fn test_range_from_coords() {
        let r = Range::from_coords(1, 2, 3, 4);
        assert_eq!(r.start.line, 1);
        assert_eq!(r.start.column, 2);
        assert_eq!(r.end.line, 3);
        assert_eq!(r.end.column, 4);
    }

    #[test]
    fn test_range_is_empty() {
        // A point range in half-open interval is NOT empty (covers one char)
        let point = Range::point(Position::origin());
        assert!(!point.is_empty());
        
        // A truly empty range has start == end
        let empty = Range::new(Position::new(0, 5), Position::new(0, 5));
        assert!(empty.is_empty());
        
        let line = Range::from_coords(0, 0, 0, 5);
        assert!(!line.is_empty());
    }

    #[test]
    fn test_range_contains() {
        let r = Range::from_coords(1, 0, 3, 10);
        assert!(r.contains(Position::new(2, 5)));
        assert!(!r.contains(Position::new(0, 5)));
    }
}

mod cursor_tests {
    use super::*;

    #[test]
    fn test_cursor_new() {
        let c = Cursor::new(5, 10);
        assert_eq!(c.line, 5);
        assert_eq!(c.column, 10);
    }

    #[test]
    fn test_cursor_origin() {
        let c = Cursor::origin();
        assert_eq!(c.line, 0);
        assert_eq!(c.column, 0);
    }

    #[test]
    fn test_cursor_position() {
        let c = Cursor::new(3, 7);
        let p = c.to_position();
        assert_eq!(p.line, 3);
        assert_eq!(p.column, 7);
    }

    #[test]
    fn test_cursor_from_position() {
        let p = Position::new(10, 20);
        let c = Cursor::from(p);
        assert_eq!(c.line, 10);
        assert_eq!(c.column, 20);
    }
}

mod mode_tests {
    use super::*;

    #[test]
    fn test_mode_normal() {
        assert!(matches!(Mode::Normal, Mode::Normal));
    }

    #[test]
    fn test_mode_insert() {
        assert!(matches!(Mode::Insert, Mode::Insert));
    }

    #[test]
    fn test_mode_visual() {
        assert!(matches!(Mode::Visual, Mode::Visual));
    }

    #[test]
    fn test_mode_equality() {
        assert_eq!(Mode::Normal, Mode::Normal);
        assert_ne!(Mode::Normal, Mode::Insert);
    }
}

mod key_event_tests {
    use super::*;

    #[test]
    fn test_key_event_char_helper() {
        let k = KeyEvent::char('a');
        assert!(matches!(k, KeyEvent::Char('a', _)));
    }

    #[test]
    fn test_key_event_ctrl_helper() {
        let k = KeyEvent::ctrl('c');
        if let KeyEvent::Char(c, m) = k {
            assert_eq!(c, 'c');
            assert!(m.ctrl);
        }
    }

    #[test]
    fn test_key_event_escape() {
        assert!(matches!(KeyEvent::Escape, KeyEvent::Escape));
    }

    #[test]
    fn test_key_event_enter() {
        assert!(matches!(KeyEvent::Enter, KeyEvent::Enter));
    }

    #[test]
    fn test_key_event_backspace() {
        assert!(matches!(KeyEvent::Backspace, KeyEvent::Backspace));
    }

    #[test]
    fn test_key_event_arrows() {
        assert!(matches!(KeyEvent::Up, KeyEvent::Up));
        assert!(matches!(KeyEvent::Down, KeyEvent::Down));
        assert!(matches!(KeyEvent::Left, KeyEvent::Left));
        assert!(matches!(KeyEvent::Right, KeyEvent::Right));
    }
}

mod modifier_edge_cases {
    use super::*;

    #[test]
    fn test_modifier_none() {
        let m = Modifier::NONE;
        assert!(!m.ctrl);
        assert!(!m.alt);
        assert!(!m.shift);
    }

    #[test]
    fn test_modifier_ctrl() {
        let m = Modifier::CTRL;
        assert!(m.ctrl);
    }

    #[test]
    fn test_modifier_alt() {
        let m = Modifier::ALT;
        assert!(m.alt);
    }

    #[test]
    fn test_modifier_shift() {
        let m = Modifier::SHIFT;
        assert!(m.shift);
    }
}

mod buffer_id_tests {
    use super::*;

    #[test]
    fn test_buffer_id_new() {
        let id = BufferId::new(42);
        assert_eq!(id.as_u64(), 42);
    }

    #[test]
    fn test_buffer_id_equality() {
        let a = BufferId::new(1);
        let b = BufferId::new(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_buffer_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BufferId::new(1));
        set.insert(BufferId::new(2));
        assert!(set.contains(&BufferId::new(1)));
        assert!(!set.contains(&BufferId::new(3)));
    }
}

mod intent_tests {
    use super::*;

    #[test]
    fn test_intent_quit() {
        let i = Intent::Quit;
        assert!(matches!(i, Intent::Quit));
    }

    #[test]
    fn test_intent_enter_mode() {
        let i = Intent::EnterMode(Mode::Insert);
        assert!(matches!(i, Intent::EnterMode(Mode::Insert)));
    }

    #[test]
    fn test_intent_move_up() {
        let i = Intent::MoveUp(5);
        assert!(matches!(i, Intent::MoveUp(5)));
    }

    #[test]
    fn test_intent_move_down() {
        let i = Intent::MoveDown(3);
        assert!(matches!(i, Intent::MoveDown(3)));
    }

    #[test]
    fn test_intent_insert_char() {
        let i = Intent::InsertChar('x');
        assert!(matches!(i, Intent::InsertChar('x')));
    }
}

mod editor_event_tests {
    use super::*;

    #[test]
    fn test_editor_event_key() {
        let e = EditorEvent::Key(KeyEvent::char('a'));
        assert!(matches!(e, EditorEvent::Key(_)));
    }

    #[test]
    fn test_editor_event_resize() {
        let e = EditorEvent::Resize { width: 80, height: 24 };
        assert!(matches!(e, EditorEvent::Resize { .. }));
    }

    #[test]
    fn test_editor_event_quit() {
        let e = EditorEvent::Quit;
        assert!(matches!(e, EditorEvent::Quit));
    }
}

mod version_tests {
    use super::*;

    #[test]
    fn test_buffer_version_new() {
        let v = BufferVersion::new(5);
        assert_eq!(v.as_u64(), 5);
    }

    #[test]
    fn test_buffer_version_initial() {
        let v = BufferVersion::initial();
        assert_eq!(v.as_u64(), 0);
    }

    #[test]
    fn test_buffer_version_next() {
        let v = BufferVersion::new(5);
        let next = v.next();
        assert_eq!(next.as_u64(), 6);
    }

    #[test]
    fn test_buffer_version_equality() {
        let a = BufferVersion::new(10);
        let b = BufferVersion::new(10);
        assert_eq!(a, b);
    }

    #[test]
    fn test_buffer_version_ordering() {
        let a = BufferVersion::new(1);
        let b = BufferVersion::new(2);
        assert!(a < b);
    }
}

// Additional edge case tests for Position
mod position_extra {
    use super::*;

    #[test]
    fn test_position_max_line() {
        let pos = Position::new(usize::MAX, 0);
        assert_eq!(pos.line, usize::MAX);
    }

    #[test]
    fn test_position_max_column() {
        let pos = Position::new(0, usize::MAX);
        assert_eq!(pos.column, usize::MAX);
    }

    #[test]
    fn test_position_clone() {
        let pos = Position::new(10, 20);
        let cloned = pos.clone();
        assert_eq!(pos, cloned);
    }

    #[test]
    fn test_position_copy() {
        let pos = Position::new(5, 10);
        let copied = pos;
        assert_eq!(pos, copied);
    }

    #[test]
    fn test_position_ordering_many() {
        for col in 0..10 {
            let a = Position::new(5, col);
            let b = Position::new(5, col + 1);
            assert!(a < b);
        }
    }
}

// Additional edge case tests for Range
mod range_extra {
    use super::*;

    #[test]
    fn test_range_entire_line() {
        let range = Range::from_coords(0, 0, 0, 100);
        assert!(!range.is_empty());
    }

    #[test]
    fn test_range_multiline_large() {
        let range = Range::from_coords(0, 0, 1000, 1000);
        assert!(!range.is_empty());
    }

    #[test]
    fn test_range_reversed() {
        let range = Range::from_coords(10, 10, 0, 0);
        let norm = range.normalized();
        assert!(norm.start <= norm.end);
    }

    #[test]
    fn test_range_clone() {
        let range = Range::from_coords(1, 2, 3, 4);
        let cloned = range.clone();
        assert_eq!(range, cloned);
    }

    #[test]
    fn test_range_copy() {
        let range = Range::from_coords(1, 2, 3, 4);
        let copied = range;
        assert_eq!(range, copied);
    }
}

// Additional edge case tests for Cursor
mod cursor_extra {
    use super::*;

    #[test]
    fn test_cursor_max_values() {
        let cursor = Cursor::new(usize::MAX, usize::MAX);
        assert_eq!(cursor.line, usize::MAX);
        assert_eq!(cursor.column, usize::MAX);
    }

    #[test]
    fn test_cursor_roundtrip() {
        let cursor = Cursor::new(7, 13);
        let pos = cursor.to_position();
        let cursor2 = Cursor::from(pos);
        assert_eq!(cursor, cursor2);
    }

    #[test]
    fn test_cursor_clone() {
        let cursor = Cursor::new(5, 10);
        let cloned = cursor.clone();
        assert_eq!(cursor, cloned);
    }

    #[test]
    fn test_cursor_copy() {
        let cursor = Cursor::new(5, 10);
        let copied = cursor;
        assert_eq!(cursor, copied);
    }
}

// Additional edge case tests for Mode
mod mode_extra {
    use super::*;

    #[test]
    fn test_mode_visual_end_exclusive() {
        assert!(Mode::Visual.is_end_exclusive());
        assert!(Mode::VisualLine.is_end_exclusive());
        assert!(Mode::VisualBlock.is_end_exclusive());
    }

    #[test]
    fn test_mode_normal_insert() {
        assert!(Mode::Normal.is_end_exclusive());
        assert!(Mode::Insert.is_end_inclusive());
    }

    #[test]
    fn test_mode_clone_all() {
        let modes = [
            Mode::Normal, Mode::Insert, Mode::Visual,
            Mode::VisualLine, Mode::VisualBlock, Mode::Command, Mode::Replace,
        ];
        for mode in modes {
            let cloned = mode.clone();
            assert_eq!(mode, cloned);
        }
    }

    #[test]
    fn test_mode_inequality_pairwise() {
        let modes = [
            Mode::Normal, Mode::Insert, Mode::Visual,
            Mode::VisualLine, Mode::VisualBlock, Mode::Command, Mode::Replace,
        ];
        for i in 0..modes.len() {
            for j in i+1..modes.len() {
                assert_ne!(modes[i], modes[j]);
            }
        }
    }
}

// Additional edge case tests for KeyEvent
mod key_event_extra {
    use super::*;

    #[test]
    fn test_key_event_all_special() {
        let keys = [
            KeyEvent::Escape, KeyEvent::Enter, KeyEvent::Backspace,
            KeyEvent::Delete, KeyEvent::Tab, KeyEvent::BackTab,
            KeyEvent::Home, KeyEvent::End, KeyEvent::PageUp, KeyEvent::PageDown,
        ];
        for key in keys {
            let debug = format!("{:?}", key);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_key_event_function_keys() {
        for n in 1..=12 {
            let key = KeyEvent::F(n);
            let debug = format!("{:?}", key);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_key_event_char_variety() {
        for c in 'a'..='z' {
            let key = KeyEvent::char(c);
            match key {
                KeyEvent::Char(ch, _) => assert_eq!(ch, c),
                _ => panic!("Expected Char"),
            }
        }
    }

    #[test]
    fn test_key_event_ctrl_variety() {
        for c in 'a'..='z' {
            let key = KeyEvent::ctrl(c);
            match key {
                KeyEvent::Char(ch, mods) => {
                    assert_eq!(ch, c);
                    assert!(mods.ctrl);
                }
                _ => panic!("Expected Char with ctrl"),
            }
        }
    }

    #[test]
    fn test_key_event_clone() {
        let key = KeyEvent::ctrl('c');
        let cloned = key.clone();
        assert_eq!(key, cloned);
    }
}

// Additional edge case tests for Modifier
mod modifier_extra {
    use super::*;

    #[test]
    fn test_modifier_constants() {
        assert!(!Modifier::NONE.ctrl && !Modifier::NONE.alt && !Modifier::NONE.shift);
        assert!(Modifier::CTRL.ctrl && !Modifier::CTRL.alt && !Modifier::CTRL.shift);
        assert!(!Modifier::ALT.ctrl && Modifier::ALT.alt && !Modifier::ALT.shift);
        assert!(!Modifier::SHIFT.ctrl && !Modifier::SHIFT.alt && Modifier::SHIFT.shift);
    }

    #[test]
    fn test_modifier_default_is_none() {
        let def = Modifier::default();
        assert_eq!(def, Modifier::NONE);
    }

    #[test]
    fn test_modifier_copy() {
        let mods = Modifier::CTRL;
        let copied = mods;
        assert_eq!(mods, copied);
    }

    #[test]
    fn test_modifier_clone() {
        let mods = Modifier::ALT;
        let cloned = mods.clone();
        assert_eq!(mods, cloned);
    }
}

// Additional edge case tests for Intent
mod intent_extra {
    use super::*;

    #[test]
    fn test_intent_move_variety() {
        let moves = [
            Intent::MoveUp(1), Intent::MoveUp(10), Intent::MoveUp(100),
            Intent::MoveDown(1), Intent::MoveDown(10), Intent::MoveDown(100),
            Intent::MoveLeft(1), Intent::MoveLeft(10), Intent::MoveLeft(100),
            Intent::MoveRight(1), Intent::MoveRight(10), Intent::MoveRight(100),
        ];
        for m in moves {
            let debug = format!("{:?}", m);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_intent_enter_mode_all() {
        let modes = [
            Mode::Normal, Mode::Insert, Mode::Visual,
            Mode::VisualLine, Mode::VisualBlock, Mode::Command, Mode::Replace,
        ];
        for mode in modes {
            let intent = Intent::EnterMode(mode);
            match intent {
                Intent::EnterMode(m) => assert_eq!(m, mode),
                _ => panic!("Expected EnterMode"),
            }
        }
    }

    #[test]
    fn test_intent_insert_char_variety() {
        for c in 'A'..='Z' {
            let intent = Intent::InsertChar(c);
            match intent {
                Intent::InsertChar(ch) => assert_eq!(ch, c),
                _ => panic!("Expected InsertChar"),
            }
        }
    }

    #[test]
    fn test_intent_clone() {
        let intent = Intent::MoveDown(5);
        let cloned = intent.clone();
        assert_eq!(intent, cloned);
    }
}

// Additional edge case tests for BufferVersion
mod buffer_version_extra {
    use super::*;

    #[test]
    fn test_buffer_version_next_many() {
        let mut v = BufferVersion::initial();
        for i in 0..100 {
            assert_eq!(v.as_u64(), i);
            v = v.next();
        }
        assert_eq!(v.as_u64(), 100);
    }

    #[test]
    fn test_buffer_version_clone() {
        let v = BufferVersion::new(42);
        let cloned = v.clone();
        assert_eq!(v.as_u64(), cloned.as_u64());
    }

    #[test]
    fn test_buffer_version_copy() {
        let v = BufferVersion::new(50);
        let copied = v;
        assert_eq!(v.as_u64(), copied.as_u64());
    }

    #[test]
    fn test_buffer_version_ordering_sequence() {
        let v1 = BufferVersion::new(1);
        let v2 = BufferVersion::new(2);
        let v3 = BufferVersion::new(3);
        assert!(v1 < v2);
        assert!(v2 < v3);
    }
}
