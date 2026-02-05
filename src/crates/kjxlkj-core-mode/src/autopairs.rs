//! Auto-pairs module for automatic bracket/quote pairing.
//!
//! Implements automatic insertion of matching closing characters
//! when opening characters are typed.

use std::collections::HashMap;

/// A pair definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    /// Opening character(s).
    pub open: String,
    /// Closing character(s).
    pub close: String,
    /// Whether this is a quote pair (same open/close).
    pub is_quote: bool,
    /// Filetypes where this pair is enabled (empty = all).
    pub filetypes: Vec<String>,
}

impl Pair {
    /// Create a bracket-style pair.
    pub fn bracket(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            open: open.into(),
            close: close.into(),
            is_quote: false,
            filetypes: Vec::new(),
        }
    }

    /// Create a quote-style pair (same open/close).
    pub fn quote(char: impl Into<String>) -> Self {
        let c = char.into();
        Self {
            open: c.clone(),
            close: c,
            is_quote: true,
            filetypes: Vec::new(),
        }
    }

    /// Restrict to specific filetypes.
    pub fn for_filetypes(mut self, filetypes: Vec<String>) -> Self {
        self.filetypes = filetypes;
        self
    }

    /// Check if pair is enabled for a filetype.
    pub fn enabled_for(&self, filetype: &str) -> bool {
        self.filetypes.is_empty() || self.filetypes.iter().any(|ft| ft == filetype)
    }
}

/// Configuration for autopairs.
#[derive(Debug, Clone)]
pub struct AutoPairsConfig {
    /// Whether autopairs is enabled.
    pub enabled: bool,
    /// Defined pairs.
    pub pairs: Vec<Pair>,
    /// Whether to skip over closing characters.
    pub skip_close: bool,
    /// Whether to delete pairs on backspace.
    pub delete_pair: bool,
    /// Whether to insert newline between pairs.
    pub smart_newline: bool,
    /// Whether to enable smart quotes (context-aware).
    pub smart_quotes: bool,
    /// Characters that prevent pairing when before cursor.
    pub no_pair_after: String,
}

impl Default for AutoPairsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pairs: vec![
                Pair::bracket("(", ")"),
                Pair::bracket("[", "]"),
                Pair::bracket("{", "}"),
                Pair::bracket("<", ">"),
                Pair::quote("\""),
                Pair::quote("'"),
                Pair::quote("`"),
            ],
            skip_close: true,
            delete_pair: true,
            smart_newline: true,
            smart_quotes: true,
            no_pair_after: "\\".to_string(),
        }
    }
}

impl AutoPairsConfig {
    /// Create a new config with default pairs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable autopairs.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Add a custom pair.
    pub fn add_pair(mut self, pair: Pair) -> Self {
        self.pairs.push(pair);
        self
    }

    /// Find a pair by its opening character.
    pub fn find_by_open(&self, open: &str) -> Option<&Pair> {
        self.pairs.iter().find(|p| p.open == open)
    }

    /// Find a pair by its closing character.
    pub fn find_by_close(&self, close: &str) -> Option<&Pair> {
        self.pairs.iter().find(|p| p.close == close)
    }
}

/// Context for autopairs decisions.
#[derive(Debug, Clone)]
pub struct AutoPairsContext {
    /// Character before cursor.
    pub char_before: Option<char>,
    /// Character after cursor.
    pub char_after: Option<char>,
    /// Whether cursor is in a string.
    pub in_string: bool,
    /// Whether cursor is in a comment.
    pub in_comment: bool,
    /// Current filetype.
    pub filetype: String,
}

impl AutoPairsContext {
    /// Create context from surrounding characters.
    pub fn new(char_before: Option<char>, char_after: Option<char>) -> Self {
        Self {
            char_before,
            char_after,
            in_string: false,
            in_comment: false,
            filetype: String::new(),
        }
    }

    /// Set string context.
    pub fn with_in_string(mut self, in_string: bool) -> Self {
        self.in_string = in_string;
        self
    }

    /// Set comment context.
    pub fn with_in_comment(mut self, in_comment: bool) -> Self {
        self.in_comment = in_comment;
        self
    }

    /// Set filetype.
    pub fn with_filetype(mut self, filetype: impl Into<String>) -> Self {
        self.filetype = filetype.into();
        self
    }

    /// Check if character before is word character.
    pub fn word_char_before(&self) -> bool {
        self.char_before
            .map(|c| c.is_alphanumeric() || c == '_')
            .unwrap_or(false)
    }
}

/// Result of autopairs processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoPairsResult {
    /// Insert the typed character normally.
    Insert(String),
    /// Insert opening and closing, cursor between.
    InsertPair { open: String, close: String },
    /// Skip over the closing character.
    Skip,
    /// Delete both characters of a pair.
    DeletePair,
    /// Insert newline with indentation between pairs.
    SmartNewline,
    /// No action.
    None,
}

/// Autopairs processor.
#[derive(Debug, Clone)]
pub struct AutoPairs {
    /// Configuration.
    config: AutoPairsConfig,
    /// Cached open -> pair map.
    open_map: HashMap<String, Pair>,
    /// Cached close -> pair map.
    close_map: HashMap<String, Pair>,
}

impl AutoPairs {
    /// Create with config.
    pub fn new(config: AutoPairsConfig) -> Self {
        let mut open_map = HashMap::new();
        let mut close_map = HashMap::new();

        for pair in &config.pairs {
            open_map.insert(pair.open.clone(), pair.clone());
            close_map.insert(pair.close.clone(), pair.clone());
        }

        Self {
            config,
            open_map,
            close_map,
        }
    }

    /// Create with default config.
    pub fn default_config() -> Self {
        Self::new(AutoPairsConfig::default())
    }

    /// Check if enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Process a typed character.
    pub fn on_char(&self, c: char, ctx: &AutoPairsContext) -> AutoPairsResult {
        if !self.config.enabled {
            return AutoPairsResult::Insert(c.to_string());
        }

        let s = c.to_string();

        // Check for skip-over closing
        if self.config.skip_close {
            if let Some(pair) = self.close_map.get(&s) {
                if ctx.char_after == Some(c) && pair.enabled_for(&ctx.filetype) {
                    return AutoPairsResult::Skip;
                }
            }
        }

        // Check for opening character
        if let Some(pair) = self.open_map.get(&s) {
            if pair.enabled_for(&ctx.filetype) {
                // Smart quotes: don't pair in certain contexts
                if pair.is_quote
                    && self.config.smart_quotes
                    && (ctx.in_string || ctx.in_comment || ctx.word_char_before())
                {
                    return AutoPairsResult::Insert(s);
                }

                // Don't pair after escape character
                if let Some(before) = ctx.char_before {
                    if self.config.no_pair_after.contains(before) {
                        return AutoPairsResult::Insert(s);
                    }
                }

                return AutoPairsResult::InsertPair {
                    open: pair.open.clone(),
                    close: pair.close.clone(),
                };
            }
        }

        AutoPairsResult::Insert(s)
    }

    /// Process backspace.
    pub fn on_backspace(&self, ctx: &AutoPairsContext) -> AutoPairsResult {
        if !self.config.enabled || !self.config.delete_pair {
            return AutoPairsResult::None;
        }

        // Check if between a pair
        if let (Some(before), Some(after)) = (ctx.char_before, ctx.char_after) {
            let before_s = before.to_string();
            if let Some(pair) = self.open_map.get(&before_s) {
                if pair.close.starts_with(after) && pair.enabled_for(&ctx.filetype) {
                    return AutoPairsResult::DeletePair;
                }
            }
        }

        AutoPairsResult::None
    }

    /// Process enter/newline.
    pub fn on_enter(&self, ctx: &AutoPairsContext) -> AutoPairsResult {
        if !self.config.enabled || !self.config.smart_newline {
            return AutoPairsResult::None;
        }

        // Check if between a pair
        if let (Some(before), Some(after)) = (ctx.char_before, ctx.char_after) {
            let before_s = before.to_string();
            if let Some(pair) = self.open_map.get(&before_s) {
                if pair.close.starts_with(after)
                    && pair.enabled_for(&ctx.filetype)
                    && !pair.is_quote
                {
                    return AutoPairsResult::SmartNewline;
                }
            }
        }

        AutoPairsResult::None
    }

    /// Get the config.
    pub fn config(&self) -> &AutoPairsConfig {
        &self.config
    }
}

impl Default for AutoPairs {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_bracket() {
        let pair = Pair::bracket("(", ")");
        assert_eq!(pair.open, "(");
        assert_eq!(pair.close, ")");
        assert!(!pair.is_quote);
    }

    #[test]
    fn test_pair_quote() {
        let pair = Pair::quote("\"");
        assert_eq!(pair.open, "\"");
        assert_eq!(pair.close, "\"");
        assert!(pair.is_quote);
    }

    #[test]
    fn test_pair_filetypes() {
        let pair = Pair::bracket("<", ">").for_filetypes(vec!["html".to_string(), "xml".to_string()]);
        assert!(pair.enabled_for("html"));
        assert!(pair.enabled_for("xml"));
        assert!(!pair.enabled_for("rust"));
    }

    #[test]
    fn test_config_default() {
        let config = AutoPairsConfig::default();
        assert!(config.enabled);
        assert!(!config.pairs.is_empty());
    }

    #[test]
    fn test_config_find_pair() {
        let config = AutoPairsConfig::default();
        let pair = config.find_by_open("(");
        assert!(pair.is_some());
        assert_eq!(pair.unwrap().close, ")");
    }

    #[test]
    fn test_autopairs_insert_pair() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(None, None);

        let result = ap.on_char('(', &ctx);
        assert!(matches!(result, AutoPairsResult::InsertPair { open, close } if open == "(" && close == ")"));
    }

    #[test]
    fn test_autopairs_skip_close() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some('('), Some(')'));

        let result = ap.on_char(')', &ctx);
        assert!(matches!(result, AutoPairsResult::Skip));
    }

    #[test]
    fn test_autopairs_smart_quote_in_string() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(None, None).with_in_string(true);

        let result = ap.on_char('"', &ctx);
        assert!(matches!(result, AutoPairsResult::Insert(_)));
    }

    #[test]
    fn test_autopairs_smart_quote_after_word() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some('a'), None);

        let result = ap.on_char('"', &ctx);
        assert!(matches!(result, AutoPairsResult::Insert(_)));
    }

    #[test]
    fn test_autopairs_quote_pair() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some(' '), None);

        let result = ap.on_char('"', &ctx);
        assert!(matches!(result, AutoPairsResult::InsertPair { .. }));
    }

    #[test]
    fn test_autopairs_delete_pair() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some('('), Some(')'));

        let result = ap.on_backspace(&ctx);
        assert!(matches!(result, AutoPairsResult::DeletePair));
    }

    #[test]
    fn test_autopairs_smart_newline() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some('{'), Some('}'));

        let result = ap.on_enter(&ctx);
        assert!(matches!(result, AutoPairsResult::SmartNewline));
    }

    #[test]
    fn test_autopairs_no_newline_for_quotes() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some('"'), Some('"'));

        let result = ap.on_enter(&ctx);
        assert!(matches!(result, AutoPairsResult::None));
    }

    #[test]
    fn test_autopairs_disabled() {
        let ap = AutoPairs::new(AutoPairsConfig::disabled());
        let ctx = AutoPairsContext::new(None, None);

        let result = ap.on_char('(', &ctx);
        assert!(matches!(result, AutoPairsResult::Insert(_)));
    }

    #[test]
    fn test_autopairs_escape_prevents_pair() {
        let ap = AutoPairs::default_config();
        let ctx = AutoPairsContext::new(Some('\\'), None);

        let result = ap.on_char('(', &ctx);
        assert!(matches!(result, AutoPairsResult::Insert(_)));
    }

    #[test]
    fn test_context_word_char_before() {
        let ctx = AutoPairsContext::new(Some('a'), None);
        assert!(ctx.word_char_before());

        let ctx = AutoPairsContext::new(Some(' '), None);
        assert!(!ctx.word_char_before());

        let ctx = AutoPairsContext::new(None, None);
        assert!(!ctx.word_char_before());
    }
}
