//! Cursor operations for the editor state.

use kjxlkj_core_edit::{clamp_cursor, CursorPosition};
use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Mode;

/// Adjust cursor for `a` command (append after cursor).
pub fn cursor_append(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    let line = buffer.line(cursor.line).unwrap_or_default();
    let g_count = line_graphemes(&line).len();
    cursor.grapheme_offset = (cursor.grapheme_offset + 1).min(g_count);
}

/// Adjust cursor for `A` command (append at end of line).
pub fn cursor_append_eol(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    let line = buffer.line(cursor.line).unwrap_or_default();
    let g_count = line_graphemes(&line).len();
    cursor.grapheme_offset = g_count;
}

/// Adjust cursor for `I` command (insert at first non-blank).
pub fn cursor_insert_first_nonblank(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    let line = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line);
    let offset = graphemes
        .iter()
        .position(|g| !g.chars().all(|c| c == ' ' || c == '\t'))
        .unwrap_or(0);
    cursor.grapheme_offset = offset;
}

/// Clamp cursor when leaving Insert mode (end-exclusive).
pub fn cursor_leave_insert(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    if cursor.grapheme_offset > 0 {
        cursor.grapheme_offset -= 1;
    }
    clamp_cursor(cursor, buffer, &Mode::Normal);
}

/// Ensure viewport follows cursor.
pub fn scroll_to_cursor(
    cursor: &CursorPosition,
    top_line: &mut usize,
    height: usize,
    scrolloff: usize,
) {
    if height == 0 {
        return;
    }
    let effective_scrolloff = scrolloff.min(height / 2);
    if cursor.line < *top_line + effective_scrolloff {
        *top_line = cursor.line.saturating_sub(effective_scrolloff);
    }
    let bottom = *top_line + height - 1;
    if cursor.line + effective_scrolloff > bottom {
        *top_line = (cursor.line + effective_scrolloff).saturating_sub(height - 1);
    }
}

/// Horizontal viewport follow (no-wrap only).
pub fn scroll_horizontal(
    cursor_col: usize,
    left_col: &mut usize,
    width: usize,
    sidescrolloff: usize,
) {
    if width == 0 {
        return;
    }
    let h_margin = sidescrolloff.min(width.saturating_sub(1) / 2);
    let c_x = cursor_col.saturating_sub(*left_col);
    if c_x < h_margin {
        *left_col = cursor_col.saturating_sub(h_margin);
    }
    let max_x = width.saturating_sub(1).saturating_sub(h_margin);
    if c_x > max_x {
        *left_col = cursor_col.saturating_sub(max_x);
    }
}

/// Position viewport so cursor line is at center (zz).
pub fn viewport_center(cursor_line: usize, top_line: &mut usize, height: usize) {
    if height == 0 {
        return;
    }
    let half = height / 2;
    *top_line = cursor_line.saturating_sub(half);
}

/// Position viewport so cursor line is at top (zt).
pub fn viewport_top(cursor_line: usize, top_line: &mut usize) {
    *top_line = cursor_line;
}

/// Position viewport so cursor line is at bottom (zb).
pub fn viewport_bottom(cursor_line: usize, top_line: &mut usize, height: usize) {
    if height == 0 {
        return;
    }
    *top_line = cursor_line.saturating_sub(height - 1);
}
