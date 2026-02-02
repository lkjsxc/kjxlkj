//! Operator execution on editor state.

use kjxlkj_core_edit::{Motion, Operator, OperatorKind, TextObject};
use kjxlkj_core_state::{EditorState, RegisterContent};
use kjxlkj_core_types::{Cursor, Mode, Position, Range};

use crate::execute_motion;

/// Executes an operator with motion.
pub fn operator_motion(state: &mut EditorState, op: Operator, motion: Motion) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get(&window.buffer_id) else {
        return;
    };

    let start = window.cursor.position;
    let end = execute_motion(&buffer.text, start, &motion);
    let range = Range::new(start, end).normalized();

    apply_operator(state, &op, range);
}

/// Executes an operator with text object.
pub fn operator_text_object(state: &mut EditorState, op: Operator, text_obj: TextObject) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    let text = buffer.text.to_string();
    let Some(range) = kjxlkj_core_edit::find_text_object(&text, pos, &text_obj) else {
        return;
    };

    apply_operator(state, &op, range);
}

/// Executes an operator on the current line.
pub fn operator_line(state: &mut EditorState, op: Operator) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get(&window.buffer_id) else {
        return;
    };

    let line = window.cursor.position.line;
    let line_content = buffer.text.line(line);
    let end_col = line_content.chars().count();
    let range = Range::new(Position::new(line, 0), Position::new(line, end_col));

    apply_operator(state, &op, range);
}

/// Applies an operator to a range.
fn apply_operator(state: &mut EditorState, op: &Operator, range: Range) {
    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };

    let text = buffer.text.slice(range);

    match op.kind {
        OperatorKind::Delete => {
            buffer.text.delete(range);
            buffer.modified = true;
            window.cursor = Cursor::new(range.start);
            state.registers.delete(RegisterContent::char(text));
        }
        OperatorKind::Change => {
            buffer.text.delete(range);
            buffer.modified = true;
            window.cursor = Cursor::new(range.start);
            state.registers.delete(RegisterContent::char(text));
            state.mode.transition(Mode::Insert);
        }
        OperatorKind::Yank => {
            state.registers.yank(RegisterContent::char(text));
        }
        OperatorKind::Uppercase => {
            let upper = text.to_uppercase();
            buffer.text.delete(range);
            buffer.text.insert(range.start, &upper);
            buffer.modified = true;
        }
        OperatorKind::Lowercase => {
            let lower = text.to_lowercase();
            buffer.text.delete(range);
            buffer.text.insert(range.start, &lower);
            buffer.modified = true;
        }
        OperatorKind::ToggleCase => {
            let toggled = toggle_case(&text);
            buffer.text.delete(range);
            buffer.text.insert(range.start, &toggled);
            buffer.modified = true;
        }
        OperatorKind::IndentRight => {
            let indented = indent_lines(&text);
            buffer.text.delete(range);
            buffer.text.insert(range.start, &indented);
            buffer.modified = true;
        }
        OperatorKind::IndentLeft => {
            let dedented = dedent_lines(&text);
            buffer.text.delete(range);
            buffer.text.insert(range.start, &dedented);
            buffer.modified = true;
        }
        _ => {}
    }
}

/// Toggles case of text.
fn toggle_case(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                c.to_uppercase().next().unwrap_or(c)
            }
        })
        .collect()
}

/// Indents lines.
fn indent_lines(text: &str) -> String {
    text.lines()
        .map(|line| format!("    {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Dedents lines.
fn dedent_lines(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.starts_with("    ") {
                &line[4..]
            } else if line.starts_with('\t') {
                &line[1..]
            } else {
                line.trim_start()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
