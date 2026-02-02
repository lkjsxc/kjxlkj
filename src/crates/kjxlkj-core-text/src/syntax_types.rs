//! Syntax pattern and region types.

use std::ops::Range;

/// A syntax pattern.
#[derive(Debug, Clone)]
pub struct SyntaxPattern {
    /// Pattern name.
    pub name: String,
    /// Regex pattern.
    pub pattern: String,
    /// Whether this is a multiline pattern.
    pub multiline: bool,
}

impl SyntaxPattern {
    /// Creates a new syntax pattern.
    pub fn new(name: &str, pattern: &str) -> Self {
        Self {
            name: name.to_string(),
            pattern: pattern.to_string(),
            multiline: false,
        }
    }

    /// Sets multiline flag.
    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }
}

/// A syntax region defined by start/end patterns.
#[derive(Debug, Clone)]
pub struct SyntaxRegion {
    /// Region name.
    pub name: String,
    /// Start pattern.
    pub start: String,
    /// End pattern.
    pub end: String,
    /// Highlight group.
    pub highlight: String,
    /// Whether the region can span multiple lines.
    pub multiline: bool,
}

impl SyntaxRegion {
    /// Creates a new syntax region.
    pub fn new(name: &str, start: &str, end: &str) -> Self {
        Self {
            name: name.to_string(),
            start: start.to_string(),
            end: end.to_string(),
            highlight: name.to_string(),
            multiline: true,
        }
    }

    /// Sets the highlight group.
    pub fn with_highlight(mut self, group: &str) -> Self {
        self.highlight = group.to_string();
        self
    }

    /// Sets single-line only.
    pub fn single_line(mut self) -> Self {
        self.multiline = false;
        self
    }
}

/// A matched region in text.
#[derive(Debug, Clone)]
pub struct MatchedRegion {
    /// Region name.
    pub name: String,
    /// Byte range.
    pub range: Range<usize>,
    /// Highlight group.
    pub highlight: String,
}

impl MatchedRegion {
    /// Creates a new matched region.
    pub fn new(name: &str, range: Range<usize>, highlight: &str) -> Self {
        Self {
            name: name.to_string(),
            range,
            highlight: highlight.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_pattern() {
        let pat = SyntaxPattern::new("Comment", r"//.*").multiline();
        assert_eq!(pat.name, "Comment");
        assert!(pat.multiline);
    }

    #[test]
    fn test_syntax_region() {
        let region = SyntaxRegion::new("String", "\"", "\"")
            .with_highlight("String")
            .single_line();
        assert_eq!(region.start, "\"");
        assert!(!region.multiline);
    }

    #[test]
    fn test_matched_region() {
        let m = MatchedRegion::new("Comment", 0..10, "Comment");
        assert_eq!(m.name, "Comment");
        assert_eq!(m.range, 0..10);
    }
}
