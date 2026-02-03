//! Tests for core types.

use super::*;

mod position_tests {
    use super::*;

    #[test]
    fn test_position_ordering() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 1);
        let p3 = Position::new(1, 0);

        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p1 < p3);
        assert!(p1.is_before(&p2));
        assert!(p3.is_after(&p1));
    }

    #[test]
    fn test_position_equality() {
        let p1 = Position::new(5, 10);
        let p2 = Position::new(5, 10);
        assert_eq!(p1, p2);
    }
}

mod range_tests {
    use super::*;

    #[test]
    fn test_range_normalized() {
        let r1 = Range::new(Position::new(1, 5), Position::new(0, 0));
        let norm = r1.normalized();
        assert_eq!(norm.start, Position::new(0, 0));
        assert_eq!(norm.end, Position::new(1, 5));
    }

    #[test]
    fn test_range_contains() {
        let r = Range::from_coords(1, 0, 1, 10);
        assert!(r.contains(Position::new(1, 5)));
        assert!(!r.contains(Position::new(0, 5)));
        assert!(!r.contains(Position::new(1, 10)));
    }

    #[test]
    fn test_range_empty() {
        let r = Range::empty(Position::new(5, 5));
        assert!(r.is_empty());
        assert!(!r.is_multiline());
    }
}

mod cursor_tests {
    use super::*;

    #[test]
    fn test_cursor_move() {
        let mut cursor = Cursor::new(0, 0);
        cursor.move_to(5, 10);
        assert_eq!(cursor.line(), 5);
        assert_eq!(cursor.col(), 10);
        assert_eq!(cursor.desired_col, 10);
    }

    #[test]
    fn test_cursor_vertical_preserves_desired_col() {
        let mut cursor = Cursor::new(0, 10);
        cursor.move_vertical(1, 5);
        assert_eq!(cursor.line(), 1);
        assert_eq!(cursor.col(), 5);
        assert_eq!(cursor.desired_col, 10);

        cursor.move_vertical(2, 15);
        assert_eq!(cursor.col(), 10);
    }
}

mod mode_tests {
    use super::*;

    #[test]
    fn test_mode_cursor_styles() {
        assert_eq!(Mode::Normal.cursor_style(), CursorStyle::Block);
        assert_eq!(Mode::Insert.cursor_style(), CursorStyle::Bar);
        assert_eq!(Mode::Visual.cursor_style(), CursorStyle::Hollow);
        assert_eq!(Mode::Replace.cursor_style(), CursorStyle::Underline);
    }

    #[test]
    fn test_mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
        assert!(!Mode::Insert.is_visual());
    }
}

mod key_tests {
    use super::*;

    #[test]
    fn test_key_creation() {
        let k = Key::char('a');
        assert_eq!(k.code, KeyCode::Char('a'));
        assert_eq!(k.mods, KeyModifiers::NONE);

        let k2 = Key::ctrl(KeyCode::Char('c'));
        assert!(k2.mods.ctrl());
    }

    #[test]
    fn test_key_modifiers() {
        let mods = KeyModifiers::CTRL.union(KeyModifiers::SHIFT);
        assert!(mods.ctrl());
        assert!(mods.shift());
        assert!(!mods.alt());
    }
}

mod register_tests {
    use super::*;

    #[test]
    fn test_register_name_from_char() {
        assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
        assert_eq!(RegisterName::from_char('a'), Some(RegisterName::Named('a')));
        assert_eq!(RegisterName::from_char('0'), Some(RegisterName::Numbered(0)));
        assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
        assert_eq!(RegisterName::from_char('+'), Some(RegisterName::Clipboard));
        assert_eq!(RegisterName::from_char('~'), None);
    }
}

mod viewport_tests {
    use super::*;

    #[test]
    fn test_viewport_visibility() {
        let vp = Viewport::new(10, 0, 20, 80);
        assert!(vp.is_line_visible(10));
        assert!(vp.is_line_visible(29));
        assert!(!vp.is_line_visible(9));
        assert!(!vp.is_line_visible(30));
    }

    #[test]
    fn test_viewport_ensure_visible() {
        let mut vp = Viewport::new(10, 0, 20, 80);
        vp.ensure_line_visible(5, 3);
        assert_eq!(vp.top_line, 2);

        vp.ensure_line_visible(50, 3);
        assert!(vp.is_line_visible(50));
    }
}
