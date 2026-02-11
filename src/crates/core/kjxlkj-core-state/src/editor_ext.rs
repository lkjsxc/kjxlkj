//! Extended editing operations: case transforms, join,
//! indent, and remaining operator logic.
//!
//! See /docs/spec/editing/operators/g-operators.md.

use kjxlkj_core_types::{ContentKind, Operator};

use crate::editor::EditorState;

impl EditorState {
    /// Apply operator-line for case/format/indent ops.
    pub(crate) fn apply_case_operator_line(
        &mut self,
        op: Operator,
    ) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let text =
                    buf.line(line).unwrap_or_default();
                let trimmed =
                    text.trim_end_matches('\n');
                let transformed = match op {
                    Operator::Uppercase => {
                        trimmed.to_uppercase()
                    }
                    Operator::Lowercase => {
                        trimmed.to_lowercase()
                    }
                    Operator::ToggleCase => {
                        toggle_case_str(trimmed)
                    }
                    _ => return,
                };
                let gc = buf.line_grapheme_count(line);
                if gc > 0 {
                    let _ = buf.delete(line, 0, line, gc);
                }
                let _ =
                    buf.insert(line, 0, &transformed);
            }
        }
    }

    /// Join current line with next, without inserting
    /// a space (gJ command).
    pub(crate) fn join_lines_no_space(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                if line + 1 < buf.line_count() {
                    let gc =
                        buf.line_grapheme_count(line);
                    // Delete newline at end of current
                    // line to merge with next.
                    if gc > 0 {
                        let _ = buf.delete(
                            line,
                            gc - 1,
                            line,
                            gc,
                        );
                    }
                }
            }
        }
    }

    /// Apply a case/format/indent operator over a
    /// range of text (operator + motion).
    pub(crate) fn apply_range_case_op(
        &mut self,
        op: Operator,
        sl: usize,
        sc: usize,
        el: usize,
        ec: usize,
    ) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let text = buf.text_range(sl, sc, el, ec);
                let transformed = match op {
                    Operator::Uppercase => {
                        text.to_uppercase()
                    }
                    Operator::Lowercase => {
                        text.to_lowercase()
                    }
                    Operator::ToggleCase => {
                        toggle_case_str(&text)
                    }
                    _ => return,
                };
                let _ = buf.delete(sl, sc, el, ec);
                let _ = buf.insert(sl, sc, &transformed);
            }
        }
    }
}

/// Toggle case of each character in a string.
fn toggle_case_str(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_case_mixed() {
        assert_eq!(toggle_case_str("Hello"), "hELLO");
    }

    #[test]
    fn toggle_case_empty() {
        assert_eq!(toggle_case_str(""), "");
    }

    #[test]
    fn toggle_case_digits_unchanged() {
        assert_eq!(toggle_case_str("abc123"), "ABC123");
    }
}
