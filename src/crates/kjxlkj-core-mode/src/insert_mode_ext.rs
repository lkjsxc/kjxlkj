//! Extended Insert mode actions: Ctrl-W, Ctrl-U, Ctrl-H, Ctrl-T, Ctrl-D.
//!
//! Provides word-delete, line-kill, and indent adjustment within Insert mode.

/// Insert mode special action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertAction {
    /// Delete word before cursor (Ctrl-W).
    DeleteWordBack,
    /// Delete to start of line (Ctrl-U).
    DeleteToLineStart,
    /// Delete char before cursor (Ctrl-H / Backspace).
    DeleteCharBack,
    /// Increase indent (Ctrl-T).
    IndentLine,
    /// Decrease indent (Ctrl-D).
    DedentLine,
    /// Insert literal next char (Ctrl-V).
    InsertLiteral,
    /// Complete word (Ctrl-N / Ctrl-P).
    CompleteNext,
    CompletePrev,
}

/// Result of applying an insert action to a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsertActionResult {
    pub new_text: String,
    pub new_cursor_col: usize,
}

/// Delete the word before cursor on the given line text.
pub fn delete_word_back(line: &str, cursor_col: usize) -> InsertActionResult {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor_col.min(chars.len());
    if col == 0 { return InsertActionResult { new_text: line.to_string(), new_cursor_col: 0 }; }
    let mut i = col;
    // Skip whitespace backward
    while i > 0 && chars[i - 1].is_whitespace() { i -= 1; }
    // Skip word chars backward
    if i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') {
        while i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') { i -= 1; }
    } else if i > 0 {
        while i > 0 && !chars[i - 1].is_alphanumeric() && chars[i - 1] != '_' && !chars[i - 1].is_whitespace() { i -= 1; }
    }
    let new_text: String = chars[..i].iter().chain(chars[col..].iter()).collect();
    InsertActionResult { new_text, new_cursor_col: i }
}

/// Delete from start of line to cursor.
pub fn delete_to_line_start(line: &str, cursor_col: usize) -> InsertActionResult {
    let chars: Vec<char> = line.chars().collect();
    let col = cursor_col.min(chars.len());
    let new_text: String = chars[col..].iter().collect();
    InsertActionResult { new_text, new_cursor_col: 0 }
}

/// Increase indentation of the line by `shiftwidth` spaces.
pub fn indent_line(line: &str, cursor_col: usize, shiftwidth: usize) -> InsertActionResult {
    let indent: String = " ".repeat(shiftwidth);
    let new_text = format!("{}{}", indent, line);
    InsertActionResult { new_text, new_cursor_col: cursor_col + shiftwidth }
}

/// Decrease indentation of the line by up to `shiftwidth` spaces.
pub fn dedent_line(line: &str, cursor_col: usize, shiftwidth: usize) -> InsertActionResult {
    let spaces = line.chars().take_while(|c| *c == ' ').count();
    let remove = spaces.min(shiftwidth);
    let new_text = line[remove..].to_string();
    InsertActionResult { new_text, new_cursor_col: cursor_col.saturating_sub(remove) }
}

/// Collect word completions from a buffer text (simple prefix matching).
pub fn collect_completions(text: &str, prefix: &str, max: usize) -> Vec<String> {
    if prefix.is_empty() { return Vec::new(); }
    let mut seen = std::collections::HashSet::new();
    let mut results = Vec::new();
    for word in text.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if word.starts_with(prefix) && word != prefix && seen.insert(word.to_string()) {
            results.push(word.to_string());
            if results.len() >= max { break; }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_word_back_simple() {
        let r = delete_word_back("hello world", 11);
        assert_eq!(r.new_text, "hello ");
        assert_eq!(r.new_cursor_col, 6);
    }

    #[test]
    fn delete_word_back_with_spaces() {
        let r = delete_word_back("hello   world", 8);
        assert_eq!(r.new_text, "world");
        assert_eq!(r.new_cursor_col, 0);
    }

    #[test]
    fn delete_word_back_at_start() {
        let r = delete_word_back("hello", 0);
        assert_eq!(r.new_text, "hello");
        assert_eq!(r.new_cursor_col, 0);
    }

    #[test]
    fn delete_to_line_start_middle() {
        let r = delete_to_line_start("hello world", 6);
        assert_eq!(r.new_text, "world");
        assert_eq!(r.new_cursor_col, 0);
    }

    #[test]
    fn indent_line_basic() {
        let r = indent_line("hello", 0, 4);
        assert_eq!(r.new_text, "    hello");
        assert_eq!(r.new_cursor_col, 4);
    }

    #[test]
    fn dedent_line_basic() {
        let r = dedent_line("    hello", 4, 4);
        assert_eq!(r.new_text, "hello");
        assert_eq!(r.new_cursor_col, 0);
    }

    #[test]
    fn dedent_partial() {
        let r = dedent_line("  hello", 2, 4);
        assert_eq!(r.new_text, "hello");
        assert_eq!(r.new_cursor_col, 0);
    }

    #[test]
    fn completions_basic() {
        let text = "hello help world helper hello";
        let comps = collect_completions(text, "hel", 10);
        assert!(comps.contains(&"hello".to_string()));
        assert!(comps.contains(&"help".to_string()));
        assert!(comps.contains(&"helper".to_string()));
    }

    #[test]
    fn completions_max() {
        let text = "aa ab ac ad ae af";
        let comps = collect_completions(text, "a", 3);
        assert_eq!(comps.len(), 3);
    }
}
