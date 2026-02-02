//! Operator execution for delete, change, yank, etc.

use kjxlkj_core_types::position::Position;
use kjxlkj_core_types::operator::Operator;

/// Context needed for operator execution.
pub trait OperatorContext {
    /// Gets the cursor position.
    fn cursor(&self) -> Position;
    /// Gets text in range.
    fn text_range(&self, start: Position, end: Position) -> String;
    /// Deletes text in range.
    fn delete_range(&mut self, start: Position, end: Position);
    /// Inserts text at position.
    fn insert_at(&mut self, pos: Position, text: &str);
    /// Sets cursor position.
    fn set_cursor(&mut self, pos: Position);
    /// Yanks text to register.
    fn yank(&mut self, text: &str, register: char);
}

/// Result of operator execution.
#[derive(Debug, Clone)]
pub struct OperatorResult {
    /// Deleted text (for undo).
    pub deleted: Option<String>,
    /// Whether to enter insert mode.
    pub enter_insert: bool,
    /// New cursor position.
    pub new_cursor: Position,
}

/// Executes operators on text ranges.
pub struct OperatorExecutor;

impl OperatorExecutor {
    /// Executes an operator on a range.
    pub fn execute<C: OperatorContext>(
        ctx: &mut C,
        op: Operator,
        start: Position,
        end: Position,
        register: char,
    ) -> OperatorResult {
        let (start, end) = if start > end { (end, start) } else { (start, end) };
        
        match op {
            Operator::Delete => {
                let text = ctx.text_range(start, end);
                ctx.yank(&text, register);
                ctx.delete_range(start, end);
                ctx.set_cursor(start);
                OperatorResult {
                    deleted: Some(text),
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            Operator::Change => {
                let text = ctx.text_range(start, end);
                ctx.yank(&text, register);
                ctx.delete_range(start, end);
                ctx.set_cursor(start);
                OperatorResult {
                    deleted: Some(text),
                    enter_insert: true,
                    new_cursor: start,
                }
            }
            Operator::Yank => {
                let text = ctx.text_range(start, end);
                ctx.yank(&text, register);
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: ctx.cursor(),
                }
            }
            Operator::IndentRight => {
                Self::indent_lines(ctx, start, end, true);
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            Operator::IndentLeft => {
                Self::indent_lines(ctx, start, end, false);
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            Operator::Uppercase => {
                let text = ctx.text_range(start, end);
                ctx.delete_range(start, end);
                ctx.insert_at(start, &text.to_uppercase());
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            Operator::Lowercase => {
                let text = ctx.text_range(start, end);
                ctx.delete_range(start, end);
                ctx.insert_at(start, &text.to_lowercase());
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            Operator::ToggleCase => {
                let text = ctx.text_range(start, end);
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
                ctx.delete_range(start, end);
                ctx.insert_at(start, &toggled);
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            Operator::Format => {
                let text = ctx.text_range(start, end);
                let formatted = Self::format_text(&text, 80);
                ctx.delete_range(start, end);
                ctx.insert_at(start, &formatted);
                OperatorResult {
                    deleted: None,
                    enter_insert: false,
                    new_cursor: start,
                }
            }
            _ => OperatorResult {
                deleted: None,
                enter_insert: false,
                new_cursor: ctx.cursor(),
            },
        }
    }

    /// Formats text to fit within a maximum line width.
    fn format_text(text: &str, max_width: usize) -> String {
        let mut result = String::new();
        let mut current_line = String::new();
        
        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                result.push_str(&current_line);
                result.push('\n');
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            result.push_str(&current_line);
        }
        
        result
    }

    fn indent_lines<C: OperatorContext>(
        ctx: &mut C,
        start: Position,
        end: Position,
        indent: bool,
    ) {
        // Indent each line in range
        let start_line = start.line.as_usize();
        let end_line = end.line.as_usize();
        for line in start_line..=end_line {
            let line_start = Position::new(line, 0);
            if indent {
                ctx.insert_at(line_start, "\t");
            } else {
                let text = ctx.text_range(line_start, Position::new(line, 1));
                if text == "\t" || text == " " {
                    ctx.delete_range(line_start, Position::new(line, 1));
                }
            }
        }
    }
}
