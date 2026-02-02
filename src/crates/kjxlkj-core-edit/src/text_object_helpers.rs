//! Helper functions for text object finding.

use kjxlkj_core_types::Position;

/// Converts position to byte offset.
pub fn position_to_byte(text: &str, pos: Position) -> Option<usize> {
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in text.char_indices() {
        if line == pos.line && col == pos.col {
            return Some(i);
        }
        if ch == '\n' {
            if line == pos.line {
                return Some(i);
            }
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    if line == pos.line && col == pos.col {
        Some(text.len())
    } else {
        None
    }
}

/// Converts byte offset to position.
pub fn byte_to_position(text: &str, byte_pos: usize) -> Option<Position> {
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in text.char_indices() {
        if i >= byte_pos {
            return Some(Position::new(line, col));
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    if byte_pos == text.len() {
        Some(Position::new(line, col))
    } else {
        None
    }
}

/// Converts byte range to position range.
pub fn byte_range_to_position(
    text: &str,
    start: usize,
    end: usize,
) -> Option<kjxlkj_core_types::Range> {
    if start >= end {
        return None;
    }
    let start_pos = byte_to_position(text, start)?;
    let end_pos = byte_to_position(text, end)?;
    Some(kjxlkj_core_types::Range::new(start_pos, end_pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_byte() {
        let text = "hello\nworld";
        assert_eq!(position_to_byte(text, Position::new(0, 0)), Some(0));
        assert_eq!(position_to_byte(text, Position::new(0, 5)), Some(5));
        assert_eq!(position_to_byte(text, Position::new(1, 0)), Some(6));
        assert_eq!(position_to_byte(text, Position::new(1, 5)), Some(11));
    }

    #[test]
    fn test_byte_to_position() {
        let text = "hello\nworld";
        assert_eq!(byte_to_position(text, 0), Some(Position::new(0, 0)));
        assert_eq!(byte_to_position(text, 5), Some(Position::new(0, 5)));
        assert_eq!(byte_to_position(text, 6), Some(Position::new(1, 0)));
        assert_eq!(byte_to_position(text, 11), Some(Position::new(1, 5)));
    }

    #[test]
    fn test_byte_range_to_position() {
        let text = "hello world";
        let range = byte_range_to_position(text, 0, 5).unwrap();
        assert_eq!(range.start, Position::new(0, 0));
        assert_eq!(range.end, Position::new(0, 5));
    }
}
