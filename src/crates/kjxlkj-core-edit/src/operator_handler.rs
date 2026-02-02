//! Operator execution.

use crate::{Operator, OperatorKind};
use kjxlkj_core_types::Range;

/// Result of operator execution.
#[derive(Debug, Clone)]
pub struct OperatorResult {
    /// Text that was affected.
    pub text: String,
    /// Whether the operation was linewise.
    pub linewise: bool,
    /// Register to store in.
    pub register: Option<char>,
}

impl OperatorResult {
    /// Creates a new result.
    pub fn new(text: String) -> Self {
        Self {
            text,
            linewise: false,
            register: None,
        }
    }

    /// Sets linewise flag.
    pub fn with_linewise(mut self, linewise: bool) -> Self {
        self.linewise = linewise;
        self
    }

    /// Sets the target register.
    pub fn with_register(mut self, register: Option<char>) -> Self {
        self.register = register;
        self
    }
}

/// Executes an operator on a range of text.
pub fn execute_operator(
    op: &Operator,
    range: &Range,
    lines: &[&str],
) -> Option<OperatorResult> {
    let text = extract_text(range, lines)?;
    let linewise = range.line_count() > 1;
    
    Some(OperatorResult::new(text)
        .with_linewise(linewise)
        .with_register(op.register))
}

/// Extracts text from a range.
pub fn extract_text(range: &Range, lines: &[&str]) -> Option<String> {
    let range = range.normalized();
    let start = range.start;
    let end = range.end;
    
    if start.line == end.line {
        // Single line
        let line = lines.get(start.line)?;
        let start_col = start.col.min(line.len());
        let end_col = end.col.min(line.len());
        Some(line[start_col..end_col].to_string())
    } else {
        // Multiple lines
        let mut result = String::new();
        for i in start.line..=end.line {
            let line = lines.get(i)?;
            if i == start.line {
                result.push_str(&line[start.col.min(line.len())..]);
                result.push('\n');
            } else if i == end.line {
                result.push_str(&line[..end.col.min(line.len())]);
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }
        Some(result)
    }
}

/// Applies a case transformation.
pub fn transform_case(text: &str, kind: OperatorKind) -> String {
    match kind {
        OperatorKind::Uppercase => text.to_uppercase(),
        OperatorKind::Lowercase => text.to_lowercase(),
        OperatorKind::ToggleCase => text.chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap_or(c)
                } else {
                    c.to_uppercase().next().unwrap_or(c)
                }
            })
            .collect(),
        _ => text.to_string(),
    }
}

/// Indents text.
pub fn indent_text(text: &str, indent: &str, direction: IndentDirection) -> String {
    text.lines()
        .map(|line| {
            match direction {
                IndentDirection::Right => format!("{}{}", indent, line),
                IndentDirection::Left => {
                    if line.starts_with(indent) {
                        line[indent.len()..].to_string()
                    } else {
                        line.trim_start().to_string()
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Indent direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentDirection {
    /// Indent right (add).
    Right,
    /// Indent left (remove).
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Position;

    fn lines(s: &str) -> Vec<&str> {
        s.lines().collect()
    }

    #[test]
    fn test_extract_text_single_line() {
        let range = Range::from_coords(0, 0, 0, 5);
        let text = extract_text(&range, &lines("hello world"));
        assert_eq!(text, Some("hello".to_string()));
    }

    #[test]
    fn test_extract_text_multi_line() {
        let range = Range::from_coords(0, 6, 1, 5);
        let text = extract_text(&range, &lines("hello world\nfoo bar"));
        assert_eq!(text, Some("world\nfoo b".to_string()));
    }

    #[test]
    fn test_transform_uppercase() {
        assert_eq!(transform_case("hello", OperatorKind::Uppercase), "HELLO");
    }

    #[test]
    fn test_transform_lowercase() {
        assert_eq!(transform_case("HELLO", OperatorKind::Lowercase), "hello");
    }

    #[test]
    fn test_transform_toggle_case() {
        assert_eq!(transform_case("HeLLo", OperatorKind::ToggleCase), "hEllO");
    }

    #[test]
    fn test_indent_right() {
        let text = "foo\nbar";
        let indented = indent_text(text, "  ", IndentDirection::Right);
        assert_eq!(indented, "  foo\n  bar");
    }

    #[test]
    fn test_indent_left() {
        let text = "  foo\n  bar";
        let unindented = indent_text(text, "  ", IndentDirection::Left);
        assert_eq!(unindented, "foo\nbar");
    }

    #[test]
    fn test_operator_result() {
        let result = OperatorResult::new("test".to_string())
            .with_linewise(true)
            .with_register(Some('a'));
        assert!(result.linewise);
        assert_eq!(result.register, Some('a'));
    }

    #[test]
    fn test_execute_operator() {
        let op = Operator::new(OperatorKind::Delete).with_register('a');
        let range = Range::from_coords(0, 0, 0, 5);
        let result = execute_operator(&op, &range, &lines("hello world")).unwrap();
        assert_eq!(result.text, "hello");
        assert_eq!(result.register, Some('a'));
    }
}
