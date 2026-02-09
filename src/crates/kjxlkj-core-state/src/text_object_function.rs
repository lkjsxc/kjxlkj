//! Function and class text objects.
//!
//! Provides `if`/`af` (inner/around function) and
//! `ic`/`ac` (inner/around class) text objects using
//! indentation-based heuristics.

use kjxlkj_core_text::BufferContent;

/// Function boundary detected by indentation heuristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionBounds {
    /// Start line (0-indexed).
    pub start: usize,
    /// End line (inclusive, 0-indexed).
    pub end: usize,
}

/// Find the function surrounding the cursor line.
///
/// Uses keyword + indentation heuristics:
/// - Look backward for `fn`, `def`, `function`, `func`, `sub`
/// - The function extends to the matching indent level or closing brace.
pub fn find_function_bounds(content: &BufferContent, cursor_line: usize) -> Option<FunctionBounds> {
    let total = content.line_count();
    if total == 0 {
        return None;
    }
    let keywords = [
        "fn ",
        "def ",
        "function ",
        "function(",
        "func ",
        "sub ",
        "pub fn ",
        "async fn ",
        "pub async fn ",
    ];

    // Search backward for a function keyword
    let mut start = cursor_line;
    loop {
        let line = content.line_str(start);
        let trimmed = line.trim_start();
        if keywords.iter().any(|kw| trimmed.starts_with(kw)) {
            break;
        }
        if start == 0 {
            return None;
        }
        start -= 1;
    }

    // Determine the indentation level of the function header
    let header = content.line_str(start);
    let indent = header.len() - header.trim_start().len();

    // Find the end: look for closing brace or return to
    // same indent level
    let mut end = start;
    let mut brace_depth: i32 = 0;
    let mut found_open = false;

    for i in start..total {
        let line = content.line_str(i);
        for ch in line.chars() {
            if ch == '{' {
                brace_depth += 1;
                found_open = true;
            } else if ch == '}' {
                brace_depth -= 1;
            }
        }
        end = i;
        if found_open && brace_depth <= 0 {
            break;
        }
        // For languages without braces, check indent
        if i > start && !line.trim().is_empty() && !found_open {
            let this_indent = line.len() - line.trim_start().len();
            if this_indent <= indent {
                end = i.saturating_sub(1);
                break;
            }
        }
    }

    if end < start {
        end = start;
    }

    Some(FunctionBounds { start, end })
}

/// Find class/struct/impl bounds surrounding cursor.
pub fn find_class_bounds(content: &BufferContent, cursor_line: usize) -> Option<FunctionBounds> {
    let total = content.line_count();
    let keywords = [
        "class ",
        "struct ",
        "impl ",
        "trait ",
        "interface ",
        "enum ",
        "module ",
    ];

    let mut start = cursor_line;
    loop {
        let line = content.line_str(start);
        let trimmed = line.trim_start();
        if keywords.iter().any(|kw| trimmed.starts_with(kw)) {
            break;
        }
        if start == 0 {
            return None;
        }
        start -= 1;
    }

    let header = content.line_str(start);
    let indent = header.len() - header.trim_start().len();
    let mut end = start;
    let mut brace_depth: i32 = 0;
    let mut found_open = false;

    for i in start..total {
        let line = content.line_str(i);
        for ch in line.chars() {
            if ch == '{' {
                brace_depth += 1;
                found_open = true;
            } else if ch == '}' {
                brace_depth -= 1;
            }
        }
        end = i;
        if found_open && brace_depth <= 0 {
            break;
        }
        if i > start && !line.trim().is_empty() && !found_open {
            let this_indent = line.len() - line.trim_start().len();
            if this_indent <= indent {
                end = i.saturating_sub(1);
                break;
            }
        }
    }

    Some(FunctionBounds { start, end })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_rust_function() {
        let content = BufferContent::from_str("line1\nfn foo() {\n    bar();\n}\nline5\n");
        let bounds = find_function_bounds(&content, 2).unwrap();
        assert_eq!(bounds.start, 1);
        assert_eq!(bounds.end, 3);
    }

    #[test]
    fn find_struct() {
        let content = BufferContent::from_str("struct Foo {\n    x: i32,\n}\n");
        let bounds = find_class_bounds(&content, 1).unwrap();
        assert_eq!(bounds.start, 0);
        assert_eq!(bounds.end, 2);
    }

    #[test]
    fn no_function_found() {
        let content = BufferContent::from_str("just some text\n");
        assert!(find_function_bounds(&content, 0).is_none());
    }
}
