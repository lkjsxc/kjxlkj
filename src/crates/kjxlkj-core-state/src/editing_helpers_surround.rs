//! Surround operation types and helpers.

/// Surround operation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurroundAction {
    Add,
    Delete,
    Change,
}

/// Get the matching surround pair for a character.
pub fn surround_pair(ch: char) -> (String, String) {
    match ch {
        '(' | ')' => ("(".to_string(), ")".to_string()),
        '[' | ']' => ("[".to_string(), "]".to_string()),
        '{' | '}' => ("{".to_string(), "}".to_string()),
        '<' | '>' => ("<".to_string(), ">".to_string()),
        '"' => ("\"".to_string(), "\"".to_string()),
        '\'' => ("'".to_string(), "'".to_string()),
        '`' => ("`".to_string(), "`".to_string()),
        _ => (ch.to_string(), ch.to_string()),
    }
}

/// Apply surround add to text.
pub fn surround_add(text: &str, open: &str, close: &str) -> String {
    format!("{open}{text}{close}")
}

/// Remove surround characters from text.
pub fn surround_delete(text: &str, open: &str, close: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.starts_with(open) && trimmed.ends_with(close) {
        let inner = &trimmed[open.len()..trimmed.len() - close.len()];
        Some(inner.to_string())
    } else {
        None
    }
}
