//! Word motion helpers.

use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;

use crate::cursor::CursorPosition;

/// Move cursor forward by one word boundary.
pub fn move_word_forward(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    let line = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line);
    let mut offset = cursor.grapheme_offset;
    // Skip current word chars
    while offset < graphemes.len() {
        let c = graphemes[offset].chars().next().unwrap_or(' ');
        if c.is_whitespace() {
            break;
        }
        offset += 1;
    }
    // Skip whitespace
    while offset < graphemes.len() {
        let c = graphemes[offset].chars().next().unwrap_or(' ');
        if !c.is_whitespace() {
            break;
        }
        offset += 1;
    }
    if offset >= graphemes.len() && cursor.line < buffer.line_count() - 1 {
        cursor.line += 1;
        cursor.grapheme_offset = 0;
        // Skip leading whitespace on next line
        let next = buffer.line(cursor.line).unwrap_or_default();
        let ng = line_graphemes(&next);
        let first_non_ws = ng
            .iter()
            .position(|g: &&str| !g.chars().all(|c: char| c.is_whitespace()))
            .unwrap_or(0);
        cursor.grapheme_offset = first_non_ws;
    } else {
        cursor.grapheme_offset = offset.min(if graphemes.is_empty() {
            0
        } else {
            graphemes.len() - 1
        });
    }
}

/// Move cursor backward by one word boundary.
pub fn move_word_backward(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    if cursor.grapheme_offset == 0 {
        if cursor.line > 0 {
            cursor.line -= 1;
            let line = buffer.line(cursor.line).unwrap_or_default();
            let g = line_graphemes(&line).len();
            cursor.grapheme_offset = if g == 0 { 0 } else { g - 1 };
        }
        return;
    }
    let line = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line);
    let mut offset = cursor.grapheme_offset;
    // Skip whitespace backward
    while offset > 0 {
        let c = graphemes[offset - 1].chars().next().unwrap_or(' ');
        if !c.is_whitespace() {
            break;
        }
        offset -= 1;
    }
    // Skip word chars backward
    while offset > 0 {
        let c = graphemes[offset - 1].chars().next().unwrap_or(' ');
        if c.is_whitespace() {
            break;
        }
        offset -= 1;
    }
    cursor.grapheme_offset = offset;
}
