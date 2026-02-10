//! Operator application.

use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{MotionAction, Operator};

use crate::cursor::CursorPosition;
use crate::motion::apply_motion;

/// Apply an operator with a motion to the buffer.
pub fn apply_operator(
    op: &Operator,
    motion: &MotionAction,
    count: usize,
    cursor: &mut CursorPosition,
    buffer: &mut TextBuffer,
    register: &mut String,
) {
    let start = *cursor;
    let mut end = *cursor;
    apply_motion(motion, &mut end, buffer, count);

    // Normalize: start <= end
    let (s, e) = if (start.line, start.grapheme_offset) <= (end.line, end.grapheme_offset) {
        (start, end)
    } else {
        (end, start)
    };

    match op {
        Operator::Delete => {
            let text = extract_range(buffer, &s, &e);
            *register = text;
            delete_range(buffer, &s, &e);
            *cursor = s;
        }
        Operator::Yank => {
            let text = extract_range(buffer, &s, &e);
            *register = text;
        }
        Operator::Change => {
            let text = extract_range(buffer, &s, &e);
            *register = text;
            delete_range(buffer, &s, &e);
            *cursor = s;
            // Mode change to Insert happens in caller
        }
        Operator::Indent => {
            indent_range(buffer, s.line, e.line, true);
        }
        Operator::Dedent => {
            indent_range(buffer, s.line, e.line, false);
        }
        _ => {}
    }
}

fn extract_range(buffer: &TextBuffer, start: &CursorPosition, end: &CursorPosition) -> String {
    let start_char = line_grapheme_to_char(buffer, start);
    let end_char = line_grapheme_to_char(buffer, end);
    // Include the end grapheme
    let end_line = buffer.line(end.line).unwrap_or_default();
    let end_gs = line_graphemes(&end_line);
    let end_g_chars = end_gs
        .get(end.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .unwrap_or(1);
    let actual_end = (end_char + end_g_chars).min(buffer.content.len_chars());
    buffer.content.slice(start_char..actual_end).to_string()
}

fn delete_range(buffer: &mut TextBuffer, start: &CursorPosition, end: &CursorPosition) {
    let start_char = line_grapheme_to_char(buffer, start);
    let end_char = line_grapheme_to_char(buffer, end);
    let end_line = buffer.line(end.line).unwrap_or_default();
    let end_gs = line_graphemes(&end_line);
    let end_g_chars = end_gs
        .get(end.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .unwrap_or(1);
    let actual_end = (end_char + end_g_chars).min(buffer.content.len_chars());
    buffer.remove_char_range(start_char, actual_end);
}

fn line_grapheme_to_char(buffer: &TextBuffer, pos: &CursorPosition) -> usize {
    let line_start = buffer.line_to_char(pos.line);
    let line_str = buffer.line(pos.line).unwrap_or_default();
    let graphemes = line_graphemes(&line_str);
    let char_offset: usize = graphemes
        .iter()
        .take(pos.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .sum();
    line_start + char_offset
}

fn indent_range(buffer: &mut TextBuffer, start_line: usize, end_line: usize, indent: bool) {
    // Process lines in reverse to keep positions stable
    let end = end_line.min(buffer.line_count().saturating_sub(1));
    for line_idx in (start_line..=end).rev() {
        let line_start = buffer.line_to_char(line_idx);
        if indent {
            buffer.insert_at_char(line_start, "    ");
        } else {
            let line_str = buffer.line(line_idx).unwrap_or_default();
            let spaces: usize = line_str.chars().take(4).take_while(|c| *c == ' ').count();
            if spaces > 0 {
                buffer.remove_char_range(line_start, line_start + spaces);
            }
        }
    }
}
