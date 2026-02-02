//! Syntax region matching.
//!
//! Defines syntax regions for pattern-based highlighting.

use std::collections::HashMap;

use crate::syntax_types::{SyntaxPattern, SyntaxRegion};

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
