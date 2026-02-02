//! Text object finder functions.

use super::text_object::{TextObject, TextObjectKind};
use crate::text_object_helpers::{byte_range_to_position, position_to_byte};
use kjxlkj_core_types::{Position, Range};

/// Finds the range of a text object in the given text.
pub fn find_text_object(text: &str, pos: Position, obj: &TextObject) -> Option<Range> {
    let byte_pos = position_to_byte(text, pos)?;
    let around = obj.is_around();

    match obj.kind {
        TextObjectKind::Word => find_word(text, byte_pos, around),
        TextObjectKind::BigWord => find_bigword(text, byte_pos, around),
        TextObjectKind::Parentheses => find_pair(text, byte_pos, '(', ')', around),
        TextObjectKind::Brackets => find_pair(text, byte_pos, '[', ']', around),
        TextObjectKind::Braces => find_pair(text, byte_pos, '{', '}', around),
        TextObjectKind::AngleBrackets => find_pair(text, byte_pos, '<', '>', around),
        TextObjectKind::SingleQuote => find_quote(text, byte_pos, '\'', around),
        TextObjectKind::DoubleQuote => find_quote(text, byte_pos, '"', around),
        TextObjectKind::BackQuote => find_quote(text, byte_pos, '`', around),
        _ => None,
    }
}

fn find_word(text: &str, byte_pos: usize, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    if byte_pos >= bytes.len() {
        return None;
    }

    let is_word_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let start = (0..=byte_pos)
        .rev()
        .find(|&i| i == 0 || !is_word_char(bytes[i - 1]))
        .unwrap_or(0);
    let end = (byte_pos..bytes.len())
        .find(|&i| !is_word_char(bytes[i]))
        .unwrap_or(bytes.len());

    if start >= end {
        return None;
    }

    let (final_start, final_end) = if around {
        let ws_end = (end..bytes.len())
            .find(|&i| !bytes[i].is_ascii_whitespace() || bytes[i] == b'\n')
            .unwrap_or(bytes.len());
        (start, ws_end)
    } else {
        (start, end)
    };

    byte_range_to_position(text, final_start, final_end)
}

fn find_bigword(text: &str, byte_pos: usize, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    if byte_pos >= bytes.len() {
        return None;
    }

    let is_word_char = |b: u8| !b.is_ascii_whitespace();
    let start = (0..=byte_pos)
        .rev()
        .find(|&i| i == 0 || !is_word_char(bytes[i - 1]))
        .unwrap_or(0);
    let end = (byte_pos..bytes.len())
        .find(|&i| !is_word_char(bytes[i]))
        .unwrap_or(bytes.len());

    if start >= end {
        return None;
    }

    let (final_start, final_end) = if around {
        let ws_end = (end..bytes.len())
            .find(|&i| !bytes[i].is_ascii_whitespace() || bytes[i] == b'\n')
            .unwrap_or(bytes.len());
        (start, ws_end)
    } else {
        (start, end)
    };

    byte_range_to_position(text, final_start, final_end)
}

fn find_pair(text: &str, byte_pos: usize, open: char, close: char, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    let (open_b, close_b) = (open as u8, close as u8);

    let mut depth = 0i32;
    let mut start = None;
    for i in (0..=byte_pos.min(bytes.len().saturating_sub(1))).rev() {
        if bytes[i] == close_b {
            depth += 1;
        } else if bytes[i] == open_b {
            if depth == 0 {
                start = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    let start = start?;

    depth = 0;
    let mut end = None;
    for i in (byte_pos.max(start + 1))..bytes.len() {
        if bytes[i] == open_b {
            depth += 1;
        } else if bytes[i] == close_b {
            if depth == 0 {
                end = Some(i);
                break;
            }
            depth -= 1;
        }
    }

    let (final_start, final_end) = if around {
        (start, end? + 1)
    } else {
        (start + 1, end?)
    };

    byte_range_to_position(text, final_start, final_end)
}

fn find_quote(text: &str, byte_pos: usize, quote: char, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    let quote_b = quote as u8;
    let line_start = text[..byte_pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = text[byte_pos..]
        .find('\n')
        .map(|i| byte_pos + i)
        .unwrap_or(text.len());

    let quotes: Vec<usize> = (line_start..line_end)
        .filter(|&i| bytes.get(i) == Some(&quote_b))
        .collect();

    for pair in quotes.chunks(2) {
        if pair.len() == 2 && pair[0] <= byte_pos && byte_pos <= pair[1] {
            let (final_start, final_end) = if around {
                (pair[0], pair[1] + 1)
            } else {
                (pair[0] + 1, pair[1])
            };
            return byte_range_to_position(text, final_start, final_end);
        }
    }

    None
}
