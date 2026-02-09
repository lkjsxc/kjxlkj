//! Surround operations: add, delete, change
//! surrounding delimiters.
//!
//! Implements `ds`, `cs`, `ys` surround commands.

use kjxlkj_core_text::BufferContent;

/// Get the pair characters for a surround char.
pub fn surround_pair(ch: char) -> Option<(char, char)> {
    match ch {
        '(' | ')' | 'b' => Some(('(', ')')),
        '{' | '}' | 'B' => Some(('{', '}')),
        '[' | ']' | 'r' => Some(('[', ']')),
        '<' | '>' | 'a' => Some(('<', '>')),
        '"' => Some(('"', '"')),
        '\'' => Some(('\'', '\'')),
        '`' => Some(('`', '`')),
        _ => None,
    }
}

/// Delete surrounding pair on the given line.
pub fn delete_surround(content: &mut BufferContent, line: usize, col: usize, ch: char) -> bool {
    let (open, close) = match surround_pair(ch) {
        Some(p) => p,
        None => return false,
    };
    let lc = content.line_content(line);
    let chars: Vec<char> = lc.chars().collect();
    // Find the open and close brackets around col
    let mut open_pos = None;
    for i in (0..=col.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == open {
            open_pos = Some(i);
            break;
        }
    }
    let open_pos = match open_pos {
        Some(p) => p,
        None => return false,
    };
    let mut close_pos = None;
    for i in (open_pos + 1)..chars.len() {
        if chars[i] == close {
            close_pos = Some(i);
            break;
        }
    }
    let close_pos = match close_pos {
        Some(p) => p,
        None => return false,
    };
    // Remove close first (higher index), then open
    let mut result: Vec<char> = chars.clone();
    result.remove(close_pos);
    result.remove(open_pos);
    let new_line: String = result.into_iter().collect();
    content.replace_line(line, &new_line);
    true
}

/// Change surrounding pair on the given line.
pub fn change_surround(
    content: &mut BufferContent,
    line: usize,
    col: usize,
    from: char,
    to: char,
) -> bool {
    let (old_open, old_close) = match surround_pair(from) {
        Some(p) => p,
        None => return false,
    };
    let (new_open, new_close) = match surround_pair(to) {
        Some(p) => p,
        None => return false,
    };
    let lc = content.line_content(line);
    let mut chars: Vec<char> = lc.chars().collect();
    // Find positions
    let mut open_pos = None;
    for i in (0..=col.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == old_open {
            open_pos = Some(i);
            break;
        }
    }
    let open_pos = match open_pos {
        Some(p) => p,
        None => return false,
    };
    let mut close_pos = None;
    for i in (open_pos + 1)..chars.len() {
        if chars[i] == old_close {
            close_pos = Some(i);
            break;
        }
    }
    let close_pos = match close_pos {
        Some(p) => p,
        None => return false,
    };
    chars[open_pos] = new_open;
    chars[close_pos] = new_close;
    let new_line: String = chars.into_iter().collect();
    content.replace_line(line, &new_line);
    true
}

/// Add surround pair around a range on a single line.
pub fn add_surround(
    content: &mut BufferContent,
    line: usize,
    start_col: usize,
    end_col: usize,
    ch: char,
) -> bool {
    let (open, close) = match surround_pair(ch) {
        Some(p) => p,
        None => return false,
    };
    let lc = content.line_content(line);
    let mut chars: Vec<char> = lc.chars().collect();
    let end = (end_col + 1).min(chars.len());
    let start = start_col.min(chars.len());
    chars.insert(end, close);
    chars.insert(start, open);
    let new_line: String = chars.into_iter().collect();
    content.replace_line(line, &new_line);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surround_pair_basics() {
        assert_eq!(surround_pair('('), Some(('(', ')')));
        assert_eq!(surround_pair('"'), Some(('"', '"')));
    }

    #[test]
    fn delete_surround_quotes() {
        let mut c = BufferContent::from_str("\"hello\"\n");
        assert!(delete_surround(&mut c, 0, 3, '"'));
        assert_eq!(c.line_content(0).trim(), "hello");
    }

    #[test]
    fn change_surround_parens_to_brackets() {
        let mut c = BufferContent::from_str("(hello)\n");
        assert!(change_surround(&mut c, 0, 3, ')', ']'));
        assert_eq!(c.line_content(0).trim(), "[hello]");
    }

    #[test]
    fn add_surround_word() {
        let mut c = BufferContent::from_str("hello\n");
        assert!(add_surround(&mut c, 0, 0, 4, '"'));
        assert_eq!(c.line_content(0).trim(), "\"hello\"");
    }
}
