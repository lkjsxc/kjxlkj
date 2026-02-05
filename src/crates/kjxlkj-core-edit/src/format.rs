//! Format on type and paste.
//!
//! Provides automatic formatting during editing.

use std::collections::HashSet;

/// Format trigger type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatTrigger {
    /// Format on typing character.
    OnType,
    /// Format on paste.
    OnPaste,
    /// Format on save.
    OnSave,
    /// Manual format.
    Manual,
}

/// Format on type configuration.
#[derive(Debug, Clone)]
pub struct FormatOnTypeConfig {
    /// Enable format on type.
    pub enabled: bool,
    /// Trigger characters.
    pub trigger_chars: HashSet<char>,
    /// Debounce milliseconds.
    pub debounce_ms: u64,
    /// Languages to enable for.
    pub enabled_languages: Vec<String>,
    /// Languages to disable for.
    pub disabled_languages: Vec<String>,
}

impl Default for FormatOnTypeConfig {
    fn default() -> Self {
        let mut trigger_chars = HashSet::new();
        trigger_chars.insert(';');
        trigger_chars.insert('}');
        trigger_chars.insert('\n');

        Self {
            enabled: false,
            trigger_chars,
            debounce_ms: 150,
            enabled_languages: Vec::new(),
            disabled_languages: vec![
                "markdown".to_string(),
                "text".to_string(),
                "plaintext".to_string(),
            ],
        }
    }
}

impl FormatOnTypeConfig {
    /// Create enabled config.
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Check if should format for language.
    pub fn should_format(&self, language: &str) -> bool {
        if !self.enabled {
            return false;
        }

        if self.disabled_languages.iter().any(|l| l == language) {
            return false;
        }

        if self.enabled_languages.is_empty() {
            return true;
        }

        self.enabled_languages.iter().any(|l| l == language)
    }

    /// Check if character triggers format.
    pub fn is_trigger(&self, c: char) -> bool {
        self.trigger_chars.contains(&c)
    }

    /// Add trigger character.
    pub fn add_trigger(&mut self, c: char) {
        self.trigger_chars.insert(c);
    }

    /// Remove trigger character.
    pub fn remove_trigger(&mut self, c: char) {
        self.trigger_chars.remove(&c);
    }
}

/// Format on paste configuration.
#[derive(Debug, Clone)]
pub struct FormatOnPasteConfig {
    /// Enable format on paste.
    pub enabled: bool,
    /// Adjust indentation.
    pub adjust_indent: bool,
    /// Run full formatting.
    pub format_content: bool,
    /// Max size for formatting (bytes).
    pub max_size: usize,
    /// Languages to enable for.
    pub enabled_languages: Vec<String>,
    /// Languages to disable for.
    pub disabled_languages: Vec<String>,
}

impl Default for FormatOnPasteConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            adjust_indent: true,
            format_content: false,
            max_size: 100_000, // 100KB
            enabled_languages: Vec::new(),
            disabled_languages: vec![
                "markdown".to_string(),
                "text".to_string(),
            ],
        }
    }
}

impl FormatOnPasteConfig {
    /// Create enabled config.
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Check if should format for language and size.
    pub fn should_format(&self, language: &str, size: usize) -> bool {
        if !self.enabled {
            return false;
        }

        if size > self.max_size {
            return false;
        }

        if self.disabled_languages.iter().any(|l| l == language) {
            return false;
        }

        if self.enabled_languages.is_empty() {
            return true;
        }

        self.enabled_languages.iter().any(|l| l == language)
    }
}

/// Indentation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentStyle {
    /// Spaces.
    Spaces(usize),
    /// Tabs.
    Tabs,
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self::Spaces(4)
    }
}

impl IndentStyle {
    /// Get one level of indentation.
    pub fn one_level(&self) -> String {
        match self {
            Self::Spaces(n) => " ".repeat(*n),
            Self::Tabs => "\t".to_string(),
        }
    }

    /// Detect indentation style from text.
    pub fn detect(text: &str) -> Self {
        let mut tabs = 0;
        let mut spaces = 0;
        let mut space_counts: Vec<usize> = Vec::new();

        for line in text.lines() {
            let trimmed = line.trim_start();
            if trimmed.is_empty() {
                continue;
            }

            let indent = line.len() - trimmed.len();
            if indent == 0 {
                continue;
            }

            if line.starts_with('\t') {
                tabs += 1;
            } else if line.starts_with(' ') {
                spaces += 1;
                space_counts.push(indent);
            }
        }

        if tabs > spaces {
            Self::Tabs
        } else if !space_counts.is_empty() {
            // Find most common indent delta
            let gcd = space_counts.iter().copied().reduce(gcd_impl).unwrap_or(4);
            Self::Spaces(gcd.clamp(2, 8))
        } else {
            Self::default()
        }
    }
}

fn gcd_impl(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd_impl(b, a % b) }
}

/// Format result.
#[derive(Debug, Clone)]
pub struct FormatResult {
    /// Formatted text.
    pub text: String,
    /// Whether text was modified.
    pub modified: bool,
    /// Error message if any.
    pub error: Option<String>,
}

impl FormatResult {
    /// Create success result.
    pub fn success(text: String, modified: bool) -> Self {
        Self {
            text,
            modified,
            error: None,
        }
    }

    /// Create error result.
    pub fn error(text: String, error: impl Into<String>) -> Self {
        Self {
            text,
            modified: false,
            error: Some(error.into()),
        }
    }
}

/// Paste adjustment.
#[derive(Debug)]
pub struct PasteAdjuster {
    /// Target indentation style.
    style: IndentStyle,
}

impl Default for PasteAdjuster {
    fn default() -> Self {
        Self::new()
    }
}

impl PasteAdjuster {
    /// Create new adjuster.
    pub fn new() -> Self {
        Self {
            style: IndentStyle::default(),
        }
    }

    /// Create with style.
    pub fn with_style(style: IndentStyle) -> Self {
        Self { style }
    }

    /// Adjust pasted text indentation.
    pub fn adjust(&self, text: &str, target_indent: usize) -> String {
        if text.is_empty() {
            return String::new();
        }

        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        // Find minimum indentation in pasted content
        let min_indent = lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);

        // Build target indent string
        let indent_str = match self.style {
            IndentStyle::Spaces(n) => " ".repeat(target_indent * n),
            IndentStyle::Tabs => "\t".repeat(target_indent),
        };

        // Adjust each line
        let mut result = String::new();
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }

            if line.trim().is_empty() {
                result.push_str(line.trim());
            } else {
                let current_indent = line.len() - line.trim_start().len();
                let relative_indent = current_indent.saturating_sub(min_indent);
                let extra = match self.style {
                    IndentStyle::Spaces(n) => " ".repeat(relative_indent * n / n.max(1)),
                    IndentStyle::Tabs => "\t".repeat(relative_indent / 4),
                };
                result.push_str(&indent_str);
                result.push_str(&extra);
                result.push_str(line.trim_start());
            }
        }

        // Preserve trailing newline
        if text.ends_with('\n') {
            result.push('\n');
        }

        result
    }

    /// Normalize indentation style.
    pub fn normalize(&self, text: &str) -> String {
        let source_style = IndentStyle::detect(text);

        if source_style == self.style {
            return text.to_string();
        }

        let mut result = String::new();
        for line in text.lines() {
            if line.trim().is_empty() {
                result.push('\n');
                continue;
            }

            let indent_chars = line.len() - line.trim_start().len();
            let indent_level = match source_style {
                IndentStyle::Spaces(n) => indent_chars / n.max(1),
                IndentStyle::Tabs => line.chars().take_while(|&c| c == '\t').count(),
            };

            let new_indent = match self.style {
                IndentStyle::Spaces(n) => " ".repeat(indent_level * n),
                IndentStyle::Tabs => "\t".repeat(indent_level),
            };

            result.push_str(&new_indent);
            result.push_str(line.trim_start());
            result.push('\n');
        }

        // Remove trailing newline if original didn't have one
        if !text.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }

        result
    }
}

/// Format on type state.
#[derive(Debug)]
pub struct FormatOnType {
    /// Configuration.
    config: FormatOnTypeConfig,
    /// Pending format (line, time).
    pending: Option<(usize, std::time::Instant)>,
}

impl Default for FormatOnType {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatOnType {
    /// Create new format on type.
    pub fn new() -> Self {
        Self {
            config: FormatOnTypeConfig::default(),
            pending: None,
        }
    }

    /// Create with config.
    pub fn with_config(config: FormatOnTypeConfig) -> Self {
        Self {
            config,
            pending: None,
        }
    }

    /// Get config.
    pub fn config(&self) -> &FormatOnTypeConfig {
        &self.config
    }

    /// Get mutable config.
    pub fn config_mut(&mut self) -> &mut FormatOnTypeConfig {
        &mut self.config
    }

    /// Check if character should trigger format.
    pub fn should_trigger(&self, c: char, language: &str) -> bool {
        self.config.should_format(language) && self.config.is_trigger(c)
    }

    /// Request format for line.
    pub fn request(&mut self, line: usize) {
        self.pending = Some((line, std::time::Instant::now()));
    }

    /// Check if format should run (after debounce).
    pub fn should_format(&self) -> Option<usize> {
        let (line, time) = self.pending?;
        if time.elapsed().as_millis() >= self.config.debounce_ms as u128 {
            Some(line)
        } else {
            None
        }
    }

    /// Clear pending format.
    pub fn clear(&mut self) {
        self.pending = None;
    }
}

/// Format on paste state.
#[derive(Debug)]
pub struct FormatOnPaste {
    /// Configuration.
    config: FormatOnPasteConfig,
    /// Adjuster.
    adjuster: PasteAdjuster,
}

impl Default for FormatOnPaste {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatOnPaste {
    /// Create new format on paste.
    pub fn new() -> Self {
        Self {
            config: FormatOnPasteConfig::default(),
            adjuster: PasteAdjuster::new(),
        }
    }

    /// Create with config.
    pub fn with_config(config: FormatOnPasteConfig) -> Self {
        Self {
            config,
            adjuster: PasteAdjuster::new(),
        }
    }

    /// Get config.
    pub fn config(&self) -> &FormatOnPasteConfig {
        &self.config
    }

    /// Get mutable config.
    pub fn config_mut(&mut self) -> &mut FormatOnPasteConfig {
        &mut self.config
    }

    /// Set indent style.
    pub fn set_indent_style(&mut self, style: IndentStyle) {
        self.adjuster = PasteAdjuster::with_style(style);
    }

    /// Process paste.
    pub fn process(&self, text: &str, target_indent: usize, language: &str) -> FormatResult {
        if !self.config.should_format(language, text.len()) {
            return FormatResult::success(text.to_string(), false);
        }

        let adjusted = if self.config.adjust_indent {
            self.adjuster.adjust(text, target_indent)
        } else {
            text.to_string()
        };

        let modified = adjusted != text;
        FormatResult::success(adjusted, modified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_on_type_config() {
        let config = FormatOnTypeConfig::default();
        assert!(!config.enabled);
        assert!(config.is_trigger(';'));
        assert!(config.is_trigger('}'));
    }

    #[test]
    fn test_format_on_type_should_format() {
        let config = FormatOnTypeConfig::enabled();
        assert!(config.should_format("rust"));
        assert!(!config.should_format("markdown"));
    }

    #[test]
    fn test_format_on_paste_config() {
        let config = FormatOnPasteConfig::default();
        assert!(!config.enabled);
        assert!(config.adjust_indent);
    }

    #[test]
    fn test_format_on_paste_should_format() {
        let config = FormatOnPasteConfig::enabled();
        assert!(config.should_format("rust", 100));
        assert!(!config.should_format("rust", 200_000));
        assert!(!config.should_format("markdown", 100));
    }

    #[test]
    fn test_indent_style_one_level() {
        assert_eq!(IndentStyle::Spaces(4).one_level(), "    ");
        assert_eq!(IndentStyle::Tabs.one_level(), "\t");
    }

    #[test]
    fn test_indent_style_detect_spaces() {
        let text = "fn main() {\n    let x = 1;\n    let y = 2;\n}";
        let style = IndentStyle::detect(text);
        assert!(matches!(style, IndentStyle::Spaces(_)));
    }

    #[test]
    fn test_indent_style_detect_tabs() {
        let text = "fn main() {\n\tlet x = 1;\n\tlet y = 2;\n}";
        let style = IndentStyle::detect(text);
        assert_eq!(style, IndentStyle::Tabs);
    }

    #[test]
    fn test_format_result() {
        let result = FormatResult::success("hello".to_string(), true);
        assert!(result.modified);
        assert!(result.error.is_none());

        let error = FormatResult::error("hello".to_string(), "failed");
        assert!(!error.modified);
        assert!(error.error.is_some());
    }

    #[test]
    fn test_paste_adjuster_adjust() {
        let adjuster = PasteAdjuster::with_style(IndentStyle::Spaces(4));
        let text = "let x = 1;\nlet y = 2;";
        let adjusted = adjuster.adjust(text, 1);
        assert!(adjusted.starts_with("    "));
    }

    #[test]
    fn test_paste_adjuster_preserve_relative() {
        let adjuster = PasteAdjuster::with_style(IndentStyle::Spaces(4));
        let text = "if true {\n    inner();\n}";
        let adjusted = adjuster.adjust(text, 1);
        let lines: Vec<&str> = adjusted.lines().collect();
        assert!(lines[0].starts_with("    if"));
        assert!(lines[1].starts_with("        inner")); // Extra indent preserved
    }

    #[test]
    fn test_paste_adjuster_normalize() {
        let adjuster = PasteAdjuster::with_style(IndentStyle::Spaces(4));
        let text = "fn main() {\n\tx\n}";
        let normalized = adjuster.normalize(text);
        assert!(!normalized.contains('\t'));
    }

    #[test]
    fn test_format_on_type_trigger() {
        let fot = FormatOnType::with_config(FormatOnTypeConfig::enabled());
        assert!(fot.should_trigger(';', "rust"));
        assert!(!fot.should_trigger('a', "rust"));
        assert!(!fot.should_trigger(';', "markdown"));
    }

    #[test]
    fn test_format_on_paste_process() {
        let fop = FormatOnPaste::with_config(FormatOnPasteConfig::enabled());
        let result = fop.process("let x = 1;", 1, "rust");
        assert!(result.modified);
        assert!(result.text.starts_with("    "));
    }

    #[test]
    fn test_format_on_paste_disabled() {
        let fop = FormatOnPaste::new();
        let result = fop.process("let x = 1;", 1, "rust");
        assert!(!result.modified);
    }
}
