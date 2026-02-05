//! Operator implementations (delete, yank, change, etc.).

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, Position, Range};
use kjxlkj_core_undo::{Edit, UndoHistory};

/// Operators that act on a range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    Uppercase,
    Lowercase,
    ToggleCase,
}

/// Apply an operator to a range in the buffer.
pub fn apply_operator(
    op: Operator,
    range: Range,
    buffer: &mut TextBuffer,
    history: &mut UndoHistory,
    register: &mut String,
) -> Option<Cursor> {
    let range = range.normalized();

    match op {
        Operator::Delete => {
            let version_before = buffer.version();
            let cursor_before = range.start;

            if let Ok((deleted, version_after)) = buffer.delete(range) {
                register.clone_from(&deleted);
                let edit = Edit::delete(
                    range,
                    deleted,
                    version_before,
                    version_after,
                    range.start,
                );
                history.record(edit);
                Some(Cursor::from(range.start))
            } else {
                None
            }
        }

        Operator::Yank => {
            if let Ok(text) = buffer.slice(range) {
                register.clone_from(&text);
            }
            Some(Cursor::from(range.start))
        }

        Operator::Change => {
            let version_before = buffer.version();

            if let Ok((deleted, version_after)) = buffer.delete(range) {
                register.clone_from(&deleted);
                let edit = Edit::delete(
                    range,
                    deleted,
                    version_before,
                    version_after,
                    range.start,
                );
                history.record(edit);
                Some(Cursor::from(range.start))
            } else {
                None
            }
        }

        Operator::Indent => {
            // Indent each line in range
            let start_line = range.start.line;
            let end_line = if range.end.column == 0 {
                range.end.line.saturating_sub(1)
            } else {
                range.end.line
            };

            for line in start_line..=end_line {
                let _ = buffer.insert(Position::new(line, 0), "\t");
            }
            Some(Cursor::from(range.start))
        }

        Operator::Outdent => {
            // Outdent each line in range
            let start_line = range.start.line;
            let end_line = if range.end.column == 0 {
                range.end.line.saturating_sub(1)
            } else {
                range.end.line
            };

            for line in start_line..=end_line {
                if let Ok(text) = buffer.line(line) {
                    if text.starts_with('\t') {
                        let _ = buffer.delete(Range::from_coords(line, 0, line, 1));
                    } else if text.starts_with("    ") {
                        let _ = buffer.delete(Range::from_coords(line, 0, line, 4));
                    }
                }
            }
            Some(Cursor::from(range.start))
        }

        Operator::Uppercase => {
            if let Ok(text) = buffer.slice(range) {
                let upper = text.to_uppercase();
                let _ = buffer.replace(range, &upper);
            }
            Some(Cursor::from(range.start))
        }

        Operator::Lowercase => {
            if let Ok(text) = buffer.slice(range) {
                let lower = text.to_lowercase();
                let _ = buffer.replace(range, &lower);
            }
            Some(Cursor::from(range.start))
        }

        Operator::ToggleCase => {
            if let Ok(text) = buffer.slice(range) {
                let toggled: String = text
                    .chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            c.to_lowercase().next().unwrap_or(c)
                        } else {
                            c.to_uppercase().next().unwrap_or(c)
                        }
                    })
                    .collect();
                let _ = buffer.replace(range, &toggled);
            }
            Some(Cursor::from(range.start))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_operator() {
        let mut buf = TextBuffer::from_str("hello world");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 6);

        let cursor = apply_operator(
            Operator::Delete,
            range,
            &mut buf,
            &mut history,
            &mut register,
        );

        assert_eq!(buf.line(0).unwrap(), "world");
        assert_eq!(register, "hello ");
        assert!(cursor.is_some());
    }

    #[test]
    fn test_yank_operator() {
        let mut buf = TextBuffer::from_str("hello world");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);

        apply_operator(
            Operator::Yank,
            range,
            &mut buf,
            &mut history,
            &mut register,
        );

        assert_eq!(register, "hello");
        assert_eq!(buf.line(0).unwrap(), "hello world");
    }

    #[test]
    fn test_uppercase_operator() {
        let mut buf = TextBuffer::from_str("hello");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);

        apply_operator(
            Operator::Uppercase,
            range,
            &mut buf,
            &mut history,
            &mut register,
        );

        assert_eq!(buf.line(0).unwrap(), "HELLO");
    }
}
