//! Line-position motion execution and desired-column
//! tracking.

use kjxlkj_core_text::BufferContent;

use crate::cursor::CursorPosition;

/// Execute a line-position motion.
pub(crate) fn exec_line_start(cursor: &mut CursorPosition) {
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

pub(crate) fn exec_first_non_blank(cursor: &mut CursorPosition, content: &BufferContent) {
    let line = content.line_content(cursor.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
    let mut idx = 0;
    for i in 0..lg.count() {
        if let Some(g) = lg.get(i) {
            let c = g.chars().next().unwrap_or(' ');
            if !c.is_whitespace() {
                idx = i;
                break;
            }
        }
    }
    cursor.grapheme_offset = idx;
    cursor.clear_desired_col();
}

pub(crate) fn exec_line_end(cursor: &mut CursorPosition, content: &BufferContent) {
    let gc = content.line_graphemes(cursor.line).count();
    cursor.grapheme_offset = if gc > 0 { gc - 1 } else { 0 };
    cursor.clear_desired_col();
}

pub(crate) fn exec_last_non_blank(cursor: &mut CursorPosition, content: &BufferContent) {
    let line = content.line_content(cursor.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
    let mut idx = if lg.count() > 0 { lg.count() - 1 } else { 0 };
    for i in (0..lg.count()).rev() {
        if let Some(g) = lg.get(i) {
            let c = g.chars().next().unwrap_or(' ');
            if !c.is_whitespace() {
                idx = i;
                break;
            }
        }
    }
    cursor.grapheme_offset = idx;
    cursor.clear_desired_col();
}

pub(crate) fn exec_goto_first_line(cursor: &mut CursorPosition) {
    cursor.line = 0;
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

pub(crate) fn exec_goto_last_line(cursor: &mut CursorPosition, line_count: usize) {
    cursor.line = if line_count > 0 { line_count - 1 } else { 0 };
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

pub(crate) fn exec_goto_line(cursor: &mut CursorPosition, n: usize, line_count: usize) {
    let target = n.min(line_count.saturating_sub(1));
    cursor.line = target;
    cursor.grapheme_offset = 0;
    cursor.clear_desired_col();
}

pub(crate) fn exec_goto_column(cursor: &mut CursorPosition, col: usize, content: &BufferContent) {
    let gc = content.line_graphemes(cursor.line).count();
    cursor.grapheme_offset = col.min(if gc > 0 { gc - 1 } else { 0 });
    cursor.clear_desired_col();
}

/// Apply desired column after vertical movement.
pub(crate) fn apply_desired_col(cursor: &mut CursorPosition, content: &BufferContent) {
    let lg = content.line_graphemes(cursor.line);
    let gc = lg.count();
    if let Some(desired) = cursor.desired_col {
        cursor.grapheme_offset = lg
            .grapheme_at_col(desired)
            .min(if gc > 0 { gc - 1 } else { 0 });
    } else {
        let current_lg = content.line_graphemes(cursor.line);
        let col = current_lg
            .display_col_at(cursor.grapheme_offset)
            .unwrap_or(0);
        cursor.desired_col = Some(col);
        cursor.grapheme_offset = cursor.grapheme_offset.min(if gc > 0 { gc - 1 } else { 0 });
    }
}
