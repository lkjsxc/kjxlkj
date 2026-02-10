//! Tests for syntax highlighting and language detection.

#[cfg(test)]
mod tests {
    use crate::highlight::{default_style, HighlightGroup};
    use crate::syntax::{detect_language, highlight_line, Language};
    use std::path::Path;

    // --- Language detection ---

    #[test]
    fn detect_rust() {
        assert_eq!(detect_language(Path::new("main.rs")), Language::Rust);
    }

    #[test]
    fn detect_python() {
        assert_eq!(detect_language(Path::new("app.py")), Language::Python);
    }

    #[test]
    fn detect_typescript() {
        assert_eq!(detect_language(Path::new("index.ts")), Language::TypeScript);
    }

    #[test]
    fn detect_javascript() {
        assert_eq!(detect_language(Path::new("app.js")), Language::JavaScript);
    }

    #[test]
    fn detect_go() {
        assert_eq!(detect_language(Path::new("main.go")), Language::Go);
    }

    #[test]
    fn detect_c() {
        assert_eq!(detect_language(Path::new("main.c")), Language::C);
    }

    #[test]
    fn detect_unknown() {
        assert_eq!(detect_language(Path::new("file.xyz")), Language::Plain);
    }

    // --- Keyword highlighting ---

    #[test]
    fn highlight_rust_keyword() {
        let spans = highlight_line("fn main() {", Language::Rust);
        assert!(!spans.is_empty());
        let kw_span = &spans[0];
        assert_eq!(kw_span.group, HighlightGroup::Keyword);
        assert_eq!(&"fn main() {"[kw_span.start..kw_span.end], "fn");
    }

    #[test]
    fn highlight_rust_let_mut() {
        let spans = highlight_line("    let mut x = 42;", Language::Rust);
        let kws: Vec<_> = spans
            .iter()
            .filter(|s| s.group == HighlightGroup::Keyword)
            .collect();
        assert_eq!(kws.len(), 2); // let, mut
    }

    #[test]
    fn highlight_python_def() {
        let spans = highlight_line("def hello():", Language::Python);
        let kw = spans.iter().find(|s| s.group == HighlightGroup::Keyword);
        assert!(kw.is_some());
        let kw = kw.unwrap();
        assert_eq!(&"def hello():"[kw.start..kw.end], "def");
    }

    // --- String highlighting ---

    #[test]
    fn highlight_string_double_quotes() {
        let line = r#"let s = "hello";"#;
        let spans = highlight_line(line, Language::Rust);
        let str_span = spans.iter().find(|s| s.group == HighlightGroup::String);
        assert!(str_span.is_some());
        let str_span = str_span.unwrap();
        assert_eq!(&line[str_span.start..str_span.end], "\"hello\"");
    }

    #[test]
    fn highlight_string_with_escape() {
        let line = r#"let s = "he\"llo";"#;
        let spans = highlight_line(line, Language::Rust);
        let str_span = spans.iter().find(|s| s.group == HighlightGroup::String);
        assert!(str_span.is_some());
    }

    // --- Comment highlighting ---

    #[test]
    fn highlight_line_comment() {
        let line = "let x = 1; // comment";
        let spans = highlight_line(line, Language::Rust);
        let comment = spans.iter().find(|s| s.group == HighlightGroup::Comment);
        assert!(comment.is_some());
        let comment = comment.unwrap();
        assert_eq!(&line[comment.start..comment.end], "// comment");
    }

    #[test]
    fn highlight_python_comment() {
        let line = "x = 1  # comment";
        let spans = highlight_line(line, Language::Python);
        let comment = spans.iter().find(|s| s.group == HighlightGroup::Comment);
        assert!(comment.is_some());
    }

    // --- Number highlighting ---

    #[test]
    fn highlight_number() {
        let line = "let x = 42;";
        let spans = highlight_line(line, Language::Rust);
        let num = spans.iter().find(|s| s.group == HighlightGroup::Number);
        assert!(num.is_some());
        let num = num.unwrap();
        assert_eq!(&line[num.start..num.end], "42");
    }

    // --- Boolean highlighting ---

    #[test]
    fn highlight_boolean() {
        let line = "let b = true;";
        let spans = highlight_line(line, Language::Rust);
        let bool_span = spans.iter().find(|s| s.group == HighlightGroup::Boolean);
        assert!(bool_span.is_some());
    }

    // --- Plain text ---

    #[test]
    fn highlight_plain_returns_empty() {
        let spans = highlight_line("Hello world", Language::Plain);
        assert!(spans.is_empty());
    }

    // --- Default style ---

    #[test]
    fn default_style_keyword_is_bold() {
        let style = default_style(HighlightGroup::Keyword);
        assert!(style.bold);
    }

    #[test]
    fn default_style_comment_is_italic() {
        let style = default_style(HighlightGroup::Comment);
        assert!(style.italic);
    }
}
