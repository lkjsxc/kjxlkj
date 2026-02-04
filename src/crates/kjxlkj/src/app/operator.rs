//! Operator application.

use kjxlkj_core::{EditorState, Mode, Operator, OperatorKind, Position, RegisterName};

use super::apply_motion;

/// Apply an operator to the editor state.
pub fn apply_operator(state: &mut EditorState, op: Operator) {
    match op {
        Operator::Line { kind, count } => apply_line_operator(state, kind, count),
        Operator::WithMotion { kind, motion } => {
            apply_motion_operator(state, kind, motion)
        }
        _ => {}
    }
}

fn apply_line_operator(state: &mut EditorState, kind: OperatorKind, count: usize) {
    let start_line = state.cursor.line();
    let end_line = (start_line + count).min(state.buffer.line_count());

    match kind {
        OperatorKind::Delete => delete_lines(state, start_line, end_line),
        OperatorKind::Yank => yank_lines(state, start_line, end_line, count),
        OperatorKind::Change => change_lines(state, start_line, end_line),
        OperatorKind::Indent => indent_lines(state, start_line, end_line),
        OperatorKind::Outdent => outdent_lines(state, start_line, end_line),
        _ => {}
    }
}

fn delete_lines(state: &mut EditorState, start_line: usize, end_line: usize) {
    let text = collect_lines_text(state, start_line, end_line);
    state.registers.set(RegisterName::Unnamed, text, true);

    let start = Position::new(start_line, 0);
    let end = if end_line < state.buffer.line_count() {
        Position::new(end_line, 0)
    } else {
        let last_line = state.buffer.line_count().saturating_sub(1);
        Position::new(last_line, state.buffer.line_len(last_line))
    };
    state.buffer.delete_range(start, end);
    let new_line = start_line.min(state.buffer.line_count().saturating_sub(1));
    state.cursor.position = Position::new(new_line, 0);
}

fn yank_lines(state: &mut EditorState, start_line: usize, end_line: usize, count: usize) {
    let text = collect_lines_text(state, start_line, end_line);
    state.registers.set(RegisterName::Unnamed, text, true);
    state.set_status(format!("{} lines yanked", count));
}

fn change_lines(state: &mut EditorState, start_line: usize, end_line: usize) {
    let text = collect_lines_text(state, start_line, end_line);
    state.registers.set(RegisterName::Unnamed, text, true);

    let start = Position::new(start_line, 0);
    let end = if end_line < state.buffer.line_count() {
        Position::new(end_line, 0)
    } else {
        let last_line = state.buffer.line_count().saturating_sub(1);
        Position::new(last_line, state.buffer.line_len(last_line))
    };
    state.buffer.delete_range(start, end);
    state.cursor.position = Position::new(start_line, 0);
    state.set_mode(Mode::Insert);
}

fn indent_lines(state: &mut EditorState, start_line: usize, end_line: usize) {
    for i in start_line..end_line {
        state.buffer.insert(Position::new(i, 0), "    ");
    }
}

fn outdent_lines(state: &mut EditorState, start_line: usize, end_line: usize) {
    for i in start_line..end_line {
        if let Some(line) = state.buffer.line(i) {
            let spaces: usize = line.chars().take_while(|c| *c == ' ').count().min(4);
            if spaces > 0 {
                state.buffer.delete_range(Position::new(i, 0), Position::new(i, spaces));
            }
        }
    }
}

fn collect_lines_text(state: &EditorState, start_line: usize, end_line: usize) -> String {
    let mut text = String::new();
    for i in start_line..end_line {
        if let Some(line) = state.buffer.line(i) {
            text.push_str(&line);
            text.push('\n');
        }
    }
    text
}

fn apply_motion_operator(
    state: &mut EditorState,
    kind: OperatorKind,
    motion: kjxlkj_core::Motion,
) {
    let start = state.cursor.position;
    apply_motion(state, motion);
    let end = state.cursor.position;

    let (from, to) = if start < end { (start, end) } else { (end, start) };
    let to = Position::new(to.line, to.col + if motion.inclusive { 1 } else { 0 });

    match kind {
        OperatorKind::Delete => delete_range(state, from, to),
        OperatorKind::Yank => yank_range(state, from, to),
        OperatorKind::Change => change_range(state, from, to),
        _ => {}
    }
}

fn delete_range(state: &mut EditorState, from: Position, to: Position) {
    if let Some(text) = state.buffer.text_range(from, to) {
        state.registers.set(RegisterName::Unnamed, text, false);
    }
    state.buffer.delete_range(from, to);
    state.cursor.position = from;
}

fn yank_range(state: &mut EditorState, from: Position, to: Position) {
    if let Some(text) = state.buffer.text_range(from, to) {
        state.registers.set(RegisterName::Unnamed, text, false);
    }
    state.cursor.position = from;
}

fn change_range(state: &mut EditorState, from: Position, to: Position) {
    if let Some(text) = state.buffer.text_range(from, to) {
        state.registers.set(RegisterName::Unnamed, text, false);
    }
    state.buffer.delete_range(from, to);
    state.cursor.position = from;
    state.set_mode(Mode::Insert);
}
