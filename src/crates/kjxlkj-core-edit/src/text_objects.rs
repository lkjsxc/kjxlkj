//! Text object selection logic.
//!
//! Text objects define regions for operators to act upon.

use kjxlkj_core_types::{LineCol, TextObject};

/// Result of finding a text object range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextObjectRange {
    pub start: LineCol,
    pub end: LineCol,
}

impl TextObjectRange {
    pub fn new(start: LineCol, end: LineCol) -> Self {
        Self { start, end }
    }
}

/// Find the range of a text object at the given cursor position.
/// Returns None if the text object cannot be found.
pub fn find_text_object_range(
    text_object: TextObject,
    cursor: LineCol,
    line_content: Option<&str>,
    _full_content: &str,
) -> Option<TextObjectRange> {
    let line = line_content?;
    
    match text_object {
        TextObject::InnerWord => find_inner_word(line, cursor),
        TextObject::AroundWord => find_around_word(line, cursor),
        TextObject::InnerWORD => find_inner_word_big(line, cursor),
        TextObject::AroundWORD => find_around_word_big(line, cursor),
        TextObject::InnerDoubleQuote => find_inner_quote(line, cursor, '"'),
        TextObject::AroundDoubleQuote => find_around_quote(line, cursor, '"'),
        TextObject::InnerSingleQuote => find_inner_quote(line, cursor, '\''),
        TextObject::AroundSingleQuote => find_around_quote(line, cursor, '\''),
        TextObject::InnerParen => find_inner_pair(line, cursor, '(', ')'),
        TextObject::AroundParen => find_around_pair(line, cursor, '(', ')'),
        TextObject::InnerBracket => find_inner_pair(line, cursor, '[', ']'),
        TextObject::AroundBracket => find_around_pair(line, cursor, '[', ']'),
        TextObject::InnerBrace => find_inner_pair(line, cursor, '{', '}'),
        TextObject::AroundBrace => find_around_pair(line, cursor, '{', '}'),
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Find inner word (iw) - word characters only.
fn find_inner_word(line: &str, cursor: LineCol) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    if col >= chars.len() {
        return None;
    }
    
    let current = chars[col];
    
    // Determine word type at cursor
    let is_word = is_word_char(current);
    let is_space = current.is_whitespace();
    
    // Find start of this "word" (or sequence)
    let mut start = col;
    while start > 0 {
        let prev = chars[start - 1];
        if is_space {
            if !prev.is_whitespace() {
                break;
            }
        } else if is_word {
            if !is_word_char(prev) {
                break;
            }
        } else {
            // Punctuation
            if is_word_char(prev) || prev.is_whitespace() {
                break;
            }
        }
        start -= 1;
    }
    
    // Find end of this "word" (or sequence)
    let mut end = col;
    while end < chars.len() {
        let c = chars[end];
        if is_space {
            if !c.is_whitespace() {
                break;
            }
        } else if is_word {
            if !is_word_char(c) {
                break;
            }
        } else {
            if is_word_char(c) || c.is_whitespace() {
                break;
            }
        }
        end += 1;
    }
    
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, start as u32),
        LineCol::new(cursor.line, end as u32),
    ))
}

/// Find around word (aw) - word plus trailing (or leading) whitespace.
fn find_around_word(line: &str, cursor: LineCol) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    if col >= chars.len() {
        return None;
    }
    
    // First get the inner word
    let inner = find_inner_word(line, cursor)?;
    let mut start = inner.start.col as usize;
    let mut end = inner.end.col as usize;
    
    // Include trailing whitespace if any
    while end < chars.len() && chars[end].is_whitespace() {
        end += 1;
    }
    
    // If no trailing whitespace was included, try leading
    if end == inner.end.col as usize {
        while start > 0 && chars[start - 1].is_whitespace() {
            start -= 1;
        }
    }
    
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, start as u32),
        LineCol::new(cursor.line, end as u32),
    ))
}

/// Find inner WORD (iW) - non-whitespace sequence.
fn find_inner_word_big(line: &str, cursor: LineCol) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    if col >= chars.len() {
        return None;
    }
    
    if chars[col].is_whitespace() {
        // On whitespace, select whitespace
        let mut start = col;
        while start > 0 && chars[start - 1].is_whitespace() {
            start -= 1;
        }
        let mut end = col;
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
        return Some(TextObjectRange::new(
            LineCol::new(cursor.line, start as u32),
            LineCol::new(cursor.line, end as u32),
        ));
    }
    
    // Find start of non-whitespace
    let mut start = col;
    while start > 0 && !chars[start - 1].is_whitespace() {
        start -= 1;
    }
    
    // Find end of non-whitespace
    let mut end = col;
    while end < chars.len() && !chars[end].is_whitespace() {
        end += 1;
    }
    
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, start as u32),
        LineCol::new(cursor.line, end as u32),
    ))
}

/// Find around WORD (aW) - WORD plus trailing (or leading) whitespace.
fn find_around_word_big(line: &str, cursor: LineCol) -> Option<TextObjectRange> {
    let inner = find_inner_word_big(line, cursor)?;
    let chars: Vec<char> = line.chars().collect();
    let mut start = inner.start.col as usize;
    let mut end = inner.end.col as usize;
    
    // Include trailing whitespace
    while end < chars.len() && chars[end].is_whitespace() {
        end += 1;
    }
    
    // If no trailing, include leading
    if end == inner.end.col as usize {
        while start > 0 && chars[start - 1].is_whitespace() {
            start -= 1;
        }
    }
    
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, start as u32),
        LineCol::new(cursor.line, end as u32),
    ))
}

/// Find inner quote (i" or i').
fn find_inner_quote(line: &str, cursor: LineCol, quote: char) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    // Find opening quote (to the left or at cursor)
    let mut open = None;
    for i in (0..=col.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == quote {
            open = Some(i);
            break;
        }
    }
    
    let open = open?;
    
    // Find closing quote (to the right)
    let mut close = None;
    for i in (open + 1)..chars.len() {
        if chars[i] == quote {
            close = Some(i);
            break;
        }
    }
    
    let close = close?;
    
    // Inner is between quotes (exclusive)
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, (open + 1) as u32),
        LineCol::new(cursor.line, close as u32),
    ))
}

/// Find around quote (a" or a').
fn find_around_quote(line: &str, cursor: LineCol, quote: char) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    // Find opening quote
    let mut open = None;
    for i in (0..=col.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == quote {
            open = Some(i);
            break;
        }
    }
    
    let open = open?;
    
    // Find closing quote
    let mut close = None;
    for i in (open + 1)..chars.len() {
        if chars[i] == quote {
            close = Some(i);
            break;
        }
    }
    
    let close = close?;
    
    // Around includes the quotes
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, open as u32),
        LineCol::new(cursor.line, (close + 1) as u32),
    ))
}

/// Find inner pair (i( i[ i{).
fn find_inner_pair(line: &str, cursor: LineCol, open_ch: char, close_ch: char) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    // Find opening delimiter
    let mut open = None;
    let mut depth = 0i32;
    for i in (0..=col.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == close_ch {
            depth += 1;
        } else if chars[i] == open_ch {
            if depth == 0 {
                open = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    
    let open = open?;
    
    // Find closing delimiter
    let mut close = None;
    depth = 0;
    for i in (open + 1)..chars.len() {
        if chars[i] == open_ch {
            depth += 1;
        } else if chars[i] == close_ch {
            if depth == 0 {
                close = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    
    let close = close?;
    
    // Inner is between delimiters
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, (open + 1) as u32),
        LineCol::new(cursor.line, close as u32),
    ))
}

/// Find around pair (a( a[ a{).
fn find_around_pair(line: &str, cursor: LineCol, open_ch: char, close_ch: char) -> Option<TextObjectRange> {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col as usize;
    
    // Find opening delimiter
    let mut open = None;
    let mut depth = 0i32;
    for i in (0..=col.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == close_ch {
            depth += 1;
        } else if chars[i] == open_ch {
            if depth == 0 {
                open = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    
    let open = open?;
    
    // Find closing delimiter
    let mut close = None;
    depth = 0;
    for i in (open + 1)..chars.len() {
        if chars[i] == open_ch {
            depth += 1;
        } else if chars[i] == close_ch {
            if depth == 0 {
                close = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    
    let close = close?;
    
    // Around includes the delimiters
    Some(TextObjectRange::new(
        LineCol::new(cursor.line, open as u32),
        LineCol::new(cursor.line, (close + 1) as u32),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_word_simple() {
        let line = "hello world";
        let range = find_inner_word(line, LineCol::new(0, 0)).unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 5); // "hello"
    }

    #[test]
    fn inner_word_middle() {
        let line = "hello world";
        let range = find_inner_word(line, LineCol::new(0, 2)).unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 5); // "hello"
    }

    #[test]
    fn inner_word_second() {
        let line = "hello world";
        let range = find_inner_word(line, LineCol::new(0, 7)).unwrap();
        assert_eq!(range.start.col, 6);
        assert_eq!(range.end.col, 11); // "world"
    }

    #[test]
    fn around_word_with_trailing() {
        let line = "hello world";
        let range = find_around_word(line, LineCol::new(0, 0)).unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 6); // "hello " including space
    }

    #[test]
    fn inner_word_big() {
        let line = "hello-world foo";
        let range = find_inner_word_big(line, LineCol::new(0, 0)).unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 11); // "hello-world"
    }

    #[test]
    fn inner_double_quote() {
        let line = r#"say "hello" end"#;
        let range = find_inner_quote(line, LineCol::new(0, 6), '"').unwrap();
        assert_eq!(range.start.col, 5); // after first "
        assert_eq!(range.end.col, 10); // before second "
    }

    #[test]
    fn around_double_quote() {
        let line = r#"say "hello" end"#;
        let range = find_around_quote(line, LineCol::new(0, 6), '"').unwrap();
        assert_eq!(range.start.col, 4); // first "
        assert_eq!(range.end.col, 11); // after second "
    }

    #[test]
    fn inner_paren() {
        let line = "fn(a, b)";
        let range = find_inner_pair(line, LineCol::new(0, 4), '(', ')').unwrap();
        assert_eq!(range.start.col, 3); // after (
        assert_eq!(range.end.col, 7); // before )
    }

    #[test]
    fn around_paren() {
        let line = "fn(a, b)";
        let range = find_around_pair(line, LineCol::new(0, 4), '(', ')').unwrap();
        assert_eq!(range.start.col, 2); // (
        assert_eq!(range.end.col, 8); // after )
    }

    #[test]
    fn nested_paren() {
        let line = "fn(a, (b, c))";
        // Cursor inside inner parens
        let range = find_inner_pair(line, LineCol::new(0, 8), '(', ')').unwrap();
        assert_eq!(range.start.col, 7); // after inner (
        assert_eq!(range.end.col, 11); // before inner )
    }
}
