//! Unit tests for core types - part 1.

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
        assert!(a < b && b < c);
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn test_range_new() {
        let range = Range::new(Position::new(1, 0), Position::new(1, 10));
        assert_eq!(range.start.line, 1);
    }

    #[test]
    fn test_range_from_coords() {
        let range = Range::from_coords(1, 0, 2, 5);
        assert_eq!(range.end.line, 2);
    }

    #[test]
    fn test_range_point() {
        let range = Range::point(Position::new(5, 3));
        assert!(range.is_empty());
    }

    #[test]
    fn test_range_contains() {
        let range = Range::from_coords(1, 5, 1, 15);
        assert!(range.contains(Position::new(1, 10)));
        assert!(!range.contains(Position::new(1, 15)));
    }

    #[test]
    fn test_range_normalized() {
        let range = Range::new(Position::new(2, 0), Position::new(1, 0));
        assert_eq!(range.normalized().start.line, 1);
    }

    #[test]
    fn test_range_line_count() {
        let range = Range::from_coords(2, 0, 5, 0);
        assert_eq!(range.line_count(), 4);
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn test_buffer_id() {
        let id = BufferId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(BufferId::default().raw(), 0);
    }

    #[test]
    fn test_buffer_name() {
        let name = BufferName::new("test.rs");
        assert_eq!(name.as_str(), "test.rs");
        assert_eq!(BufferName::default().as_str(), "[No Name]");
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
    }
}
