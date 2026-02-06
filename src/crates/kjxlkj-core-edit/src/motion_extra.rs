//! Motion helpers: find-char, bracket matching, paragraph/sentence.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

pub fn find_char_forward(
    buf: &TextBuffer,
    pos: Position,
    c: char,
) -> Position {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    for i in (pos.col + 1)..chars.len() {
        if chars[i] == c {
            return Position::new(pos.line, i);
        }
    }
    pos
}

pub fn find_char_backward(
    buf: &TextBuffer,
    pos: Position,
    c: char,
) -> Position {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    for i in (0..pos.col).rev() {
        if chars[i] == c {
            return Position::new(pos.line, i);
        }
    }
    pos
}

pub fn till_char_forward(
    buf: &TextBuffer,
    pos: Position,
    c: char,
) -> Position {
    let found = find_char_forward(buf, pos, c);
    if found != pos {
        Position::new(found.line, found.col.saturating_sub(1))
    } else {
        pos
    }
}

pub fn till_char_backward(
    buf: &TextBuffer,
    pos: Position,
    c: char,
) -> Position {
    let found = find_char_backward(buf, pos, c);
    if found != pos {
        Position::new(found.line, found.col + 1)
    } else {
        pos
    }
}

pub fn matching_bracket(
    buf: &TextBuffer,
    pos: Position,
) -> Position {
    let ch = match buf.char_at(pos) {
        Some(c) => c,
        None => return pos,
    };
    let (target, forward) = match ch {
        '(' => (')', true),
        ')' => ('(', false),
        '[' => (']', true),
        ']' => ('[', false),
        '{' => ('}', true),
        '}' => ('{', false),
        '<' => ('>', true),
        '>' => ('<', false),
        _ => return pos,
    };
    let mut depth = 1i32;
    let mut p = pos;
    loop {
        if forward {
            p.col += 1;
            if p.col >= buf.line_len(p.line) + 1 {
                p.line += 1;
                p.col = 0;
                if p.line >= buf.line_count() {
                    return pos;
                }
            }
        } else {
            if p.col == 0 {
                if p.line == 0 {
                    return pos;
                }
                p.line -= 1;
                p.col = buf.line_len(p.line);
            } else {
                p.col -= 1;
            }
        }
        if let Some(c) = buf.char_at(p) {
            if c == ch {
                depth += 1;
            }
            if c == target {
                depth -= 1;
            }
            if depth == 0 {
                return p;
            }
        } else {
            return pos;
        }
    }
}

pub fn next_paragraph(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let max_line = buf.line_count().saturating_sub(1);
    let mut line = pos.line;
    let mut found = 0;
    while line < max_line && found < count {
        while line < max_line && buf.line_len(line) > 0 {
            line += 1;
        }
        while line < max_line && buf.line_len(line) == 0 {
            line += 1;
        }
        found += 1;
    }
    Position::new(line.min(max_line), 0)
}

pub fn prev_paragraph(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let mut line = pos.line;
    let mut found = 0;
    while line > 0 && found < count {
        while line > 0 && buf.line_len(line) > 0 {
            line -= 1;
        }
        while line > 0 && buf.line_len(line) == 0 {
            line -= 1;
        }
        found += 1;
    }
    Position::new(line, 0)
}

pub fn next_non_blank_line(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let max = buf.line_count().saturating_sub(1);
    let new_line = (pos.line + count).min(max);
    crate::motion::first_non_blank(buf, Position::new(new_line, 0))
}

pub fn prev_non_blank_line(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let new_line = pos.line.saturating_sub(count);
    crate::motion::first_non_blank(buf, Position::new(new_line, 0))
}

pub fn middle_of_line(
    buf: &TextBuffer,
    pos: Position,
) -> Position {
    let len = buf.line_len(pos.line);
    Position::new(pos.line, len / 2)
}

pub fn goto_percent(buf: &TextBuffer, pct: usize) -> Position {
    let pct = pct.min(100);
    let total = buf.line_count();
    let line = ((total as u64 * pct as u64) / 100)
        .min((total - 1) as u64) as usize;
    crate::motion::first_non_blank(buf, Position::new(line, 0))
}
