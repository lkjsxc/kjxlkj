//! Sentence motions.

use crate::TextBuffer;
use kjxlkj_core_types::Position;

fn is_sentence_end(c: char) -> bool {
    c == '.' || c == '?' || c == '!'
}

/// Move to next sentence start (`)` motion).
pub fn next_sentence(buf: &TextBuffer, pos: Position) -> Position {
    let max_line = buf.line_count().saturating_sub(1);
    let mut line = pos.line;
    let mut col = pos.col;

    loop {
        let chars: Vec<char> = buf.line_to_string(line).chars().collect();
        while col < chars.len() {
            if is_sentence_end(chars[col]) {
                col += 1;
                while col < chars.len()
                    && matches!(chars[col], ')' | ']' | '"' | '\'')
                { col += 1; }
                while col < chars.len() && chars[col] == ' ' { col += 1; }
                if col < chars.len() { return Position::new(line, col); }
                break;
            }
            col += 1;
        }
        line += 1;
        if line > max_line {
            return Position::new(max_line, buf.line_len(max_line).saturating_sub(1).max(0));
        }
        let next_ls = buf.line_to_string(line);
        if next_ls.trim().is_empty() {
            while line <= max_line {
                let s = buf.line_to_string(line);
                if !s.trim().is_empty() {
                    let fc = s.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    return Position::new(line, fc);
                }
                line += 1;
            }
            return Position::new(max_line, 0);
        }
        col = 0;
    }
}

/// Move to previous sentence start (`(` motion).
pub fn prev_sentence(buf: &TextBuffer, pos: Position) -> Position {
    let mut line = pos.line;
    let mut col = pos.col;

    if col > 0 { col -= 1; }
    else if line > 0 { line -= 1; col = buf.line_len(line).saturating_sub(1).max(0); }
    else { return Position::new(0, 0); }

    loop {
        let ls = buf.line_to_string(line);
        let chars: Vec<char> = ls.chars().collect();

        while col < chars.len() {
            if is_sentence_end(chars[col]) {
                let mut sc = col + 1;
                while sc < chars.len()
                    && matches!(chars[sc], ')' | ']' | '"' | '\'')
                { sc += 1; }
                while sc < chars.len() && chars[sc] == ' ' { sc += 1; }
                if sc < chars.len() { return Position::new(line, sc); }
                let nl = line + 1;
                if nl < buf.line_count() {
                    let ns = buf.line_to_string(nl);
                    let fc = ns.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    return Position::new(nl, fc);
                }
            }
            if col == 0 { break; }
            col -= 1;
        }

        if ls.trim().is_empty() {
            let mut nl = line + 1;
            while nl < buf.line_count() {
                let s = buf.line_to_string(nl);
                if !s.trim().is_empty() {
                    let fc = s.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    return Position::new(nl, fc);
                }
                nl += 1;
            }
        }

        if line == 0 { return Position::new(0, 0); }
        line -= 1;
        col = buf.line_len(line).saturating_sub(1).max(0);
    }
}
