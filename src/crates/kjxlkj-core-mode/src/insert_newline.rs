/// Insert-mode newline handling — Enter key, auto-indent, open line.

/// Newline behavior after Enter in insert mode.
#[derive(Debug, Clone, PartialEq)]
pub struct NewlineResult {
    pub new_lines: Vec<String>,
    pub cursor_line_offset: usize,
    pub cursor_col: usize,
}

/// How auto-indent is determined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoIndentMode { None, CopyIndent, SmartIndent }

/// Compute newline insertion result at position in text.
pub fn insert_newline(
    lines: &[&str], line: usize, col: usize, indent_mode: AutoIndentMode,
) -> NewlineResult {
    let current = lines.get(line).copied().unwrap_or("");
    let before = &current[..col.min(current.len())];
    let after = &current[col.min(current.len())..];
    let indent = match indent_mode {
        AutoIndentMode::None => String::new(),
        AutoIndentMode::CopyIndent => copy_indent(current),
        AutoIndentMode::SmartIndent => smart_indent(before),
    };
    let cursor_col = indent.len();
    let new_line = format!("{}{}", indent, after);
    NewlineResult {
        new_lines: vec![before.to_string(), new_line],
        cursor_line_offset: 1,
        cursor_col,
    }
}

/// Copy leading whitespace from the current line.
fn copy_indent(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
}

/// Smart indent: copy indent + add extra if line ends with `{`, `:`, `(`.
fn smart_indent(before_cursor: &str) -> String {
    let base = copy_indent(before_cursor);
    let trimmed = before_cursor.trim_end();
    if trimmed.ends_with('{') || trimmed.ends_with(':') || trimmed.ends_with('(') {
        format!("{}    ", base)
    } else {
        base
    }
}

/// Open a new line above (O) or below (o) the current line.
pub fn open_line(
    lines: &[&str], line: usize, above: bool, indent_mode: AutoIndentMode,
) -> (usize, String) {
    let ref_line = lines.get(line).copied().unwrap_or("");
    let indent = match indent_mode {
        AutoIndentMode::None => String::new(),
        AutoIndentMode::CopyIndent | AutoIndentMode::SmartIndent => copy_indent(ref_line),
    };
    let insert_at = if above { line } else { line + 1 };
    (insert_at, indent)
}

/// Compute what text remains on the current line after splitting.
pub fn line_after_split(line: &str, col: usize) -> &str {
    &line[..col.min(line.len())]
}

/// Count leading whitespace characters.
pub fn leading_whitespace(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

/// Detect if we should increase indent based on trailing char.
pub fn should_increase_indent(line: &str) -> bool {
    let t = line.trim_end();
    t.ends_with('{') || t.ends_with('(') || t.ends_with('[') || t.ends_with(':')
}

/// Detect if we should decrease indent based on first non-blank char.
pub fn should_decrease_indent(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('}') || t.starts_with(')') || t.starts_with(']')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newline_no_indent() {
        let lines = vec!["hello world"];
        let r = insert_newline(&lines, 0, 5, AutoIndentMode::None);
        assert_eq!(r.new_lines, vec!["hello", " world"]);
        assert_eq!(r.cursor_col, 0);
    }

    #[test]
    fn newline_copy_indent() {
        let lines = vec!["    hello"];
        let r = insert_newline(&lines, 0, 9, AutoIndentMode::CopyIndent);
        assert_eq!(r.new_lines[1], "    ");
        assert_eq!(r.cursor_col, 4);
    }

    #[test]
    fn newline_smart_indent_brace() {
        let lines = vec!["    fn foo() {"];
        let r = insert_newline(&lines, 0, 14, AutoIndentMode::SmartIndent);
        assert_eq!(r.cursor_col, 8);
    }

    #[test]
    fn open_line_below() {
        let lines = vec!["    hello"];
        let (at, indent) = open_line(&lines, 0, false, AutoIndentMode::CopyIndent);
        assert_eq!(at, 1);
        assert_eq!(indent, "    ");
    }

    #[test]
    fn open_line_above() {
        let lines = vec!["    hello"];
        let (at, _) = open_line(&lines, 0, true, AutoIndentMode::CopyIndent);
        assert_eq!(at, 0);
    }

    #[test]
    fn leading_ws() {
        assert_eq!(leading_whitespace("   abc"), 3);
        assert_eq!(leading_whitespace("abc"), 0);
    }

    #[test]
    fn increase_indent_detect() {
        assert!(should_increase_indent("if x {"));
        assert!(should_increase_indent("def foo():"));
        assert!(!should_increase_indent("hello"));
    }

    #[test]
    fn decrease_indent_detect() {
        assert!(should_decrease_indent("    }"));
        assert!(!should_decrease_indent("    x"));
    }

    #[test]
    fn line_after_split_at_col() {
        assert_eq!(line_after_split("hello world", 5), "hello");
    }
}
