//! Operator execution for text manipulation.

use kjxlkj_core_edit::{Edit, Operator, OperatorKind, Transaction};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position, Range};

/// Result of an operator execution.
#[derive(Debug, Clone)]
pub struct OperatorResult {
    /// The transaction to apply.
    pub transaction: Transaction,
    /// Text that was yanked/deleted (if applicable).
    pub yanked_text: Option<String>,
    /// New cursor position.
    pub cursor: Position,
}

/// Executes an operator on a range.
pub fn execute_operator(
    buffer: &TextBuffer,
    buffer_id: BufferId,
    range: Range,
    operator: &Operator,
) -> OperatorResult {
    let range = range.normalized();
    let text = buffer.slice(range);
    let mut transaction = Transaction::new(buffer.version());
    let cursor = range.start;

    match operator.kind {
        OperatorKind::Delete => {
            transaction.push(Edit::delete(buffer_id, range));
            OperatorResult {
                transaction,
                yanked_text: Some(text),
                cursor,
            }
        }
        OperatorKind::Change => {
            transaction.push(Edit::delete(buffer_id, range));
            OperatorResult {
                transaction,
                yanked_text: Some(text),
                cursor,
            }
        }
        OperatorKind::Yank => OperatorResult {
            transaction,
            yanked_text: Some(text),
            cursor,
        },
        OperatorKind::Uppercase => {
            let upper = text.to_uppercase();
            transaction.push(Edit::replace(buffer_id, range, upper));
            OperatorResult {
                transaction,
                yanked_text: None,
                cursor,
            }
        }
        OperatorKind::Lowercase => {
            let lower = text.to_lowercase();
            transaction.push(Edit::replace(buffer_id, range, lower));
            OperatorResult {
                transaction,
                yanked_text: None,
                cursor,
            }
        }
        OperatorKind::ToggleCase => {
            let toggled = toggle_case(&text);
            transaction.push(Edit::replace(buffer_id, range, toggled));
            OperatorResult {
                transaction,
                yanked_text: None,
                cursor,
            }
        }
        OperatorKind::IndentRight => {
            let indented = indent_lines(&text, "    ");
            transaction.push(Edit::replace(buffer_id, range, indented));
            OperatorResult {
                transaction,
                yanked_text: None,
                cursor,
            }
        }
        OperatorKind::IndentLeft => {
            let dedented = dedent_lines(&text, 4);
            transaction.push(Edit::replace(buffer_id, range, dedented));
            OperatorResult {
                transaction,
                yanked_text: None,
                cursor,
            }
        }
        _ => OperatorResult {
            transaction,
            yanked_text: None,
            cursor,
        },
    }
}

fn toggle_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                c.to_uppercase().next().unwrap_or(c)
            }
        })
        .collect()
}

fn indent_lines(text: &str, indent: &str) -> String {
    text.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn dedent_lines(text: &str, spaces: usize) -> String {
    text.lines()
        .map(|line| {
            let whitespace = line.len() - line.trim_start().len();
            let remove = whitespace.min(spaces);
            &line[remove..]
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_case() {
        assert_eq!(toggle_case("Hello"), "hELLO");
        assert_eq!(toggle_case("WORLD"), "world");
    }

    #[test]
    fn test_indent_lines() {
        let text = "line1\nline2";
        let indented = indent_lines(text, "  ");
        assert_eq!(indented, "  line1\n  line2");
    }

    #[test]
    fn test_dedent_lines() {
        let text = "    line1\n    line2";
        let dedented = dedent_lines(text, 2);
        assert_eq!(dedented, "  line1\n  line2");
    }

    #[test]
    fn test_delete_operator() {
        let buf = TextBuffer::from_str("hello world");
        let buf_id = BufferId::new(1);
        let range = Range::from_coords(0, 0, 0, 5);
        let op = Operator::new(OperatorKind::Delete);
        
        let result = execute_operator(&buf, buf_id, range, &op);
        assert_eq!(result.yanked_text, Some("hello".to_string()));
        assert!(!result.transaction.is_empty());
    }

    #[test]
    fn test_yank_operator() {
        let buf = TextBuffer::from_str("hello world");
        let buf_id = BufferId::new(1);
        let range = Range::from_coords(0, 6, 0, 11);
        let op = Operator::new(OperatorKind::Yank);
        
        let result = execute_operator(&buf, buf_id, range, &op);
        assert_eq!(result.yanked_text, Some("world".to_string()));
        assert!(result.transaction.is_empty());
    }

    #[test]
    fn test_uppercase_operator() {
        let buf = TextBuffer::from_str("hello");
        let buf_id = BufferId::new(1);
        let range = Range::from_coords(0, 0, 0, 5);
        let op = Operator::new(OperatorKind::Uppercase);
        
        let result = execute_operator(&buf, buf_id, range, &op);
        assert!(!result.transaction.is_empty());
    }
}
