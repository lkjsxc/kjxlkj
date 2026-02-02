//! Syntax region matching.
//!
//! Defines syntax regions for pattern-based highlighting.

use std::collections::HashMap;
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

/// Syntax definition for a file type.
#[derive(Debug, Clone, Default)]
pub struct SyntaxDefinition {
    /// Syntax patterns.
    patterns: HashMap<String, SyntaxPattern>,
    /// Syntax regions.
    regions: HashMap<String, SyntaxRegion>,
    /// Keywords by group.
    keywords: HashMap<String, Vec<String>>,
}

impl SyntaxDefinition {
    /// Creates a new syntax definition.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a pattern.
    pub fn add_pattern(&mut self, pattern: SyntaxPattern) {
        self.patterns.insert(pattern.name.clone(), pattern);
    }

    /// Adds a region.
    pub fn add_region(&mut self, region: SyntaxRegion) {
        self.regions.insert(region.name.clone(), region);
    }

    /// Adds keywords for a highlight group.
    pub fn add_keywords(&mut self, group: &str, keywords: Vec<String>) {
        self.keywords.insert(group.to_string(), keywords);
    }

    /// Returns all patterns.
    pub fn patterns(&self) -> &HashMap<String, SyntaxPattern> {
        &self.patterns
    }

    /// Returns all regions.
    pub fn regions(&self) -> &HashMap<String, SyntaxRegion> {
        &self.regions
    }

    /// Returns keywords for a group.
    pub fn keywords_for(&self, group: &str) -> Option<&Vec<String>> {
        self.keywords.get(group)
    }

    /// Checks if a word is a keyword.
    pub fn is_keyword(&self, word: &str) -> Option<&str> {
        for (group, words) in &self.keywords {
            if words.iter().any(|w| w == word) {
                return Some(group);
            }
        }
        None
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

    #[test]
    fn test_syntax_definition_patterns() {
        let mut def = SyntaxDefinition::new();
        def.add_pattern(SyntaxPattern::new("Number", r"\d+"));

        assert!(def.patterns().contains_key("Number"));
    }

    #[test]
    fn test_syntax_definition_keywords() {
        let mut def = SyntaxDefinition::new();
        def.add_keywords("Keyword", vec!["fn".to_string(), "let".to_string()]);

        assert_eq!(def.is_keyword("fn"), Some("Keyword"));
        assert_eq!(def.is_keyword("var"), None);
    }

    #[test]
    fn test_syntax_definition_regions() {
        let mut def = SyntaxDefinition::new();
        def.add_region(SyntaxRegion::new("String", "\"", "\""));

        assert!(def.regions().contains_key("String"));
    }
}
