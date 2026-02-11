//! Motion execution against a buffer.

use crate::cursor::Cursor;
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::Motion;
use unicode_segmentation::UnicodeSegmentation;

/// Apply a motion to a cursor, returning the new cursor position.
pub fn apply_motion(
    cursor: &Cursor,
    motion: &Motion,
    buffer: &Buffer,
) -> Cursor {
    let mut c = *cursor;
    match motion {
        Motion::Left => {
            if c.col > 0 {
                c.col -= 1;
            }
            c.desired_col = c.col;
        }
        Motion::Right => {
            let max = line_max_col(buffer, c.line);
            if c.col < max {
                c.col += 1;
            }
            c.desired_col = c.col;
        }
        Motion::Up => {
            if c.line > 0 {
                c.line -= 1;
                let max = line_max_col(buffer, c.line);
                c.col = c.desired_col.min(max);
            }
        }
        Motion::Down => {
            if c.line + 1 < buffer.line_count() {
                c.line += 1;
                let max = line_max_col(buffer, c.line);
                c.col = c.desired_col.min(max);
            }
        }
        Motion::LineStart => {
            c.col = 0;
            c.desired_col = 0;
        }
        Motion::LineEnd => {
            c.col = line_max_col(buffer, c.line);
            c.desired_col = usize::MAX;
        }
        Motion::FirstNonBlank => {
            c.col = first_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::GotoFirstLine => {
            c.line = 0;
            c.col = first_nonblank_col(buffer, 0);
            c.desired_col = c.col;
        }
        Motion::GotoLastLine => {
            c.line = buffer.line_count().saturating_sub(1);
            c.col = first_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::GotoLine(n) => {
            let target =
                (*n).min(buffer.line_count()).saturating_sub(1);
            c.line = target;
            c.col = first_nonblank_col(buffer, c.line);
            c.desired_col = c.col;
        }
        Motion::WordForward => {
            word_forward(&mut c, buffer);
        }
        Motion::WordBackward => {
            word_backward(&mut c, buffer);
        }
        _ => {
            // Other motions are progressive implementation targets.
        }
    }
    c
}

fn line_max_col(buffer: &Buffer, line: usize) -> usize {
    let gc = buffer.line_grapheme_count(line);
    // In normal mode, cursor can't go past last char.
    // Newline is not a selectable grapheme.
    let text = buffer.line(line).unwrap_or_default();
    let trimmed = text.trim_end_matches('\n');
    let visible: usize = trimmed.graphemes(true).count();
    if visible > 0 {
        visible - 1
    } else {
        0
    }
}

fn first_nonblank_col(buffer: &Buffer, line: usize) -> usize {
    let text = buffer.line(line).unwrap_or_default();
    for (i, g) in text.graphemes(true).enumerate() {
        if !g.chars().all(|c| c == ' ' || c == '\t') {
            return i;
        }
    }
    0
}

fn word_forward(c: &mut Cursor, buffer: &Buffer) {
    let text = buffer.line(c.line).unwrap_or_default();
    let graphemes: Vec<&str> = text.graphemes(true).collect();
    let mut col = c.col + 1;
    // Skip to next word boundary or next line.
    while col < graphemes.len() {
        let g = graphemes[col];
        if g == "\n" {
            break;
        }
        if is_word_boundary(
            graphemes.get(col.wrapping_sub(1)).copied(),
            Some(g),
        ) {
            c.col = col;
            c.desired_col = col;
            return;
        }
        col += 1;
    }
    // Move to next line.
    if c.line + 1 < buffer.line_count() {
        c.line += 1;
        c.col = first_nonblank_col(buffer, c.line);
        c.desired_col = c.col;
    }
}

fn word_backward(c: &mut Cursor, buffer: &Buffer) {
    if c.col > 0 {
        let text = buffer.line(c.line).unwrap_or_default();
        let graphemes: Vec<&str> = text.graphemes(true).collect();
        let mut col = c.col - 1;
        while col > 0 {
            if is_word_boundary(
                graphemes.get(col.wrapping_sub(1)).copied(),
                graphemes.get(col).copied(),
            ) {
                c.col = col;
                c.desired_col = col;
                return;
            }
            col -= 1;
        }
        c.col = 0;
        c.desired_col = 0;
    } else if c.line > 0 {
        c.line -= 1;
        let max = line_max_col(buffer, c.line);
        c.col = max;
        c.desired_col = max;
    }
}

fn is_word_boundary(
    prev: Option<&str>,
    curr: Option<&str>,
) -> bool {
    match (prev, curr) {
        (Some(p), Some(c)) => {
            let p_word = is_word_char(p);
            let c_word = is_word_char(c);
            p_word != c_word
        }
        _ => false,
    }
}

fn is_word_char(g: &str) -> bool {
    g.chars()
        .next()
        .map(|c| c.is_alphanumeric() || c == '_')
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn motion_left_at_zero() {
        let b = Buffer::from_text(BufferId(0), "t", "hello");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::Left, &b);
        assert_eq!(nc.col, 0);
    }

    #[test]
    fn motion_right_clamps() {
        let b = Buffer::from_text(BufferId(0), "t", "hi");
        let c = Cursor::new(0, 1);
        let nc = apply_motion(&c, &Motion::Right, &b);
        assert_eq!(nc.col, 1); // "hi" max col=1
    }

    #[test]
    fn motion_down() {
        let b = Buffer::from_text(BufferId(0), "t", "a\nb");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::Down, &b);
        assert_eq!(nc.line, 1);
    }

    #[test]
    fn goto_last_line() {
        let b = Buffer::from_text(BufferId(0), "t", "a\nb\nc");
        let c = Cursor::new(0, 0);
        let nc = apply_motion(&c, &Motion::GotoLastLine, &b);
        assert_eq!(nc.line, 2);
    }
}
