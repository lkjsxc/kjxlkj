//! Regex pattern compilation and match iteration.
//!
//! Wraps the `regex` crate with Vim-compatible pattern translation
//! and provides match iteration over buffer text.

/// Compiled regex pattern with original source.
#[derive(Debug, Clone)]
pub struct CompiledPattern {
    pub source: String,
    pub case_sensitive: bool,
    inner: regex::Regex,
}

/// A match result with byte offsets and capture groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchResult {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub captures: Vec<Option<String>>,
}

/// Error from pattern compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternError {
    pub message: String,
    pub offset: Option<usize>,
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Compile a search pattern, translating basic Vim regex to Rust regex.
pub fn compile_pattern(pattern: &str, case_sensitive: bool) -> Result<CompiledPattern, PatternError> {
    let translated = translate_vim_pattern(pattern);
    let full = if case_sensitive {
        translated
    } else {
        format!("(?i){}", translated)
    };
    let inner = regex::Regex::new(&full).map_err(|e| PatternError {
        message: e.to_string(),
        offset: None,
    })?;
    Ok(CompiledPattern { source: pattern.to_string(), case_sensitive, inner })
}

/// Find all matches of a compiled pattern in the given text.
pub fn find_all_matches(pattern: &CompiledPattern, text: &str) -> Vec<MatchResult> {
    pattern.inner.captures_iter(text).map(|cap| {
        let m = cap.get(0).unwrap();
        let captures: Vec<Option<String>> = cap.iter().skip(1)
            .map(|c| c.map(|m| m.as_str().to_string()))
            .collect();
        MatchResult {
            start: m.start(),
            end: m.end(),
            text: m.as_str().to_string(),
            captures,
        }
    }).collect()
}

/// Find the first match at or after `offset`.
pub fn find_next(pattern: &CompiledPattern, text: &str, offset: usize) -> Option<MatchResult> {
    let search_text = if offset <= text.len() { &text[offset..] } else { return None; };
    pattern.inner.captures(search_text).map(|cap| {
        let m = cap.get(0).unwrap();
        let captures: Vec<Option<String>> = cap.iter().skip(1)
            .map(|c| c.map(|m| m.as_str().to_string())).collect();
        MatchResult {
            start: offset + m.start(),
            end: offset + m.end(),
            text: m.as_str().to_string(),
            captures,
        }
    })
}

/// Find the last match before `offset`.
pub fn find_prev(pattern: &CompiledPattern, text: &str, offset: usize) -> Option<MatchResult> {
    let search_text = if offset <= text.len() { &text[..offset] } else { text };
    find_all_matches(pattern, search_text).into_iter().last()
}

/// Translate basic Vim regex patterns to Rust regex.
/// Handles: `\<` → `\b`, `\>` → `\b`, `\(` → `(`, `\)` → `)`, `\+` → `+`,
/// `\{` → `{`, `\}` → `}`, and `\|` → `|` (very magic is not implemented).
fn translate_vim_pattern(pattern: &str) -> String {
    let mut out = String::with_capacity(pattern.len());
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            match chars[i + 1] {
                '<' | '>' => { out.push_str("\\b"); i += 2; }
                '(' => { out.push('('); i += 2; }
                ')' => { out.push(')'); i += 2; }
                '+' => { out.push('+'); i += 2; }
                '{' => { out.push('{'); i += 2; }
                '}' => { out.push('}'); i += 2; }
                '|' => { out.push('|'); i += 2; }
                _ => { out.push('\\'); out.push(chars[i + 1]); i += 2; }
            }
        } else {
            out.push(chars[i]); i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_simple() {
        let p = compile_pattern("hello", true).unwrap();
        assert_eq!(p.source, "hello");
    }

    #[test]
    fn find_all() {
        let p = compile_pattern("\\w+", true).unwrap();
        let matches = find_all_matches(&p, "hello world foo");
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].text, "hello");
        assert_eq!(matches[2].text, "foo");
    }

    #[test]
    fn find_next_from_offset() {
        let p = compile_pattern("bar", true).unwrap();
        let m = find_next(&p, "foo bar baz bar", 5).unwrap();
        assert_eq!(m.start, 12); // second "bar" at index 12
    }

    #[test]
    fn find_prev_before_offset() {
        let p = compile_pattern("o", true).unwrap();
        let m = find_prev(&p, "foo boo", 7).unwrap();
        assert_eq!(m.start, 6);
    }

    #[test]
    fn case_insensitive() {
        let p = compile_pattern("hello", false).unwrap();
        let matches = find_all_matches(&p, "Hello HELLO hello");
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn vim_word_boundary() {
        let p = compile_pattern("\\<word\\>", true).unwrap();
        let matches = find_all_matches(&p, "word sword wordy word");
        assert_eq!(matches.len(), 2); // first and last "word"
    }

    #[test]
    fn vim_groups() {
        let p = compile_pattern("\\(foo\\)\\|\\(bar\\)", true).unwrap();
        let matches = find_all_matches(&p, "foo bar baz");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn invalid_pattern() {
        let result = compile_pattern("[invalid", true);
        assert!(result.is_err());
    }
}
