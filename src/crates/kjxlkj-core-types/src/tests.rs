//! Unit tests for core types.

use super::*;

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.col, 10);
    }

    #[test]
    fn test_position_origin() {
        let pos = Position::origin();
        assert!(pos.is_origin());
        assert_eq!(pos.line, 0);
        assert_eq!(pos.col, 0);
    }

    #[test]
    fn test_position_line_start() {
        let pos = Position::line_start(3);
        assert_eq!(pos.line, 3);
        assert_eq!(pos.col, 0);
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
        let a = Position::new(1, 5);
        let b = Position::new(1, 5);
        assert_eq!(a, b);
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn test_range_new() {
        let start = Position::new(1, 0);
        let end = Position::new(1, 10);
        let range = Range::new(start, end);
        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
    }

    #[test]
    fn test_range_from_coords() {
        let range = Range::from_coords(1, 0, 2, 5);
        assert_eq!(range.start.line, 1);
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.line, 2);
        assert_eq!(range.end.col, 5);
    }

    #[test]
    fn test_range_point() {
        let pos = Position::new(5, 3);
        let range = Range::point(pos);
        assert!(range.is_empty());
        assert_eq!(range.start, range.end);
    }

    #[test]
    fn test_range_contains() {
        let range = Range::from_coords(1, 5, 1, 15);
        assert!(range.contains(Position::new(1, 5)));
        assert!(range.contains(Position::new(1, 10)));
        assert!(!range.contains(Position::new(1, 15)));
        assert!(!range.contains(Position::new(1, 4)));
        assert!(!range.contains(Position::new(0, 10)));
    }

    #[test]
    fn test_range_normalized() {
        let range = Range::new(Position::new(2, 0), Position::new(1, 0));
        let norm = range.normalized();
        assert_eq!(norm.start.line, 1);
        assert_eq!(norm.end.line, 2);
    }

    #[test]
    fn test_range_line_count() {
        let single = Range::from_coords(5, 0, 5, 10);
        assert_eq!(single.line_count(), 1);

        let multi = Range::from_coords(2, 0, 5, 0);
        assert_eq!(multi.line_count(), 4);
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn test_buffer_id_new() {
        let id = BufferId::new(42);
        assert_eq!(id.raw(), 42);
    }

    #[test]
    fn test_buffer_id_default() {
        let id = BufferId::default();
        assert_eq!(id.raw(), 0);
    }

    #[test]
    fn test_buffer_name_new() {
        let name = BufferName::new("test.rs");
        assert_eq!(name.as_str(), "test.rs");
    }

    #[test]
    fn test_buffer_name_default() {
        let name = BufferName::default();
        assert_eq!(name.as_str(), "[No Name]");
    }
}

#[cfg(test)]
mod cursor_tests {
    use super::*;

    #[test]
    fn test_cursor_new() {
        let cursor = Cursor::new(Position::new(3, 5));
        assert_eq!(cursor.line(), 3);
        assert_eq!(cursor.col(), 5);
    }

    #[test]
    fn test_cursor_at() {
        let cursor = Cursor::at(10, 20);
        assert_eq!(cursor.line(), 10);
        assert_eq!(cursor.col(), 20);
    }

    #[test]
    fn test_cursor_origin() {
        let cursor = Cursor::origin();
        assert_eq!(cursor.line(), 0);
        assert_eq!(cursor.col(), 0);
    }
}

#[cfg(test)]
mod mode_tests {
    use super::*;

    #[test]
    fn test_mode_default() {
        let mode = Mode::default();
        assert_eq!(mode, Mode::Normal);
    }

    #[test]
    fn test_mode_is_visual() {
        assert!(Mode::Visual.is_visual());
        assert!(Mode::VisualLine.is_visual());
        assert!(Mode::VisualBlock.is_visual());
        assert!(!Mode::Normal.is_visual());
    }

    #[test]
    fn test_mode_is_insert_like() {
        assert!(Mode::Insert.is_insert_like());
        assert!(Mode::Replace.is_insert_like());
        assert!(!Mode::Normal.is_insert_like());
    }

    #[test]
    fn test_mode_name() {
        assert_eq!(Mode::Normal.name(), "NORMAL");
        assert_eq!(Mode::Insert.name(), "INSERT");
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn test_version_initial() {
        let v = BufferVersion::initial();
        assert_eq!(v.raw(), 0);
    }

    #[test]
    fn test_version_next() {
        let v1 = BufferVersion::initial();
        let v2 = v1.next();
        assert!(v2.raw() > v1.raw());
    }
}

#[cfg(test)]
mod window_tests {
    use super::*;

    #[test]
    fn test_window_id_new() {
        let id = WindowId::new(5);
        assert_eq!(id.raw(), 5);
    }

    #[test]
    fn test_window_id_default() {
        let id = WindowId::default();
        assert_eq!(id.raw(), 0);
    }
}
