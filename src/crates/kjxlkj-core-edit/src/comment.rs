//! Comment toggling.
//!
//! Provides line and block comment operations.

use std::collections::HashMap;

/// Comment style for a language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentStyle {
    /// Line comment prefix (e.g., "//").
    pub line: Option<String>,
    /// Block comment (open, close) (e.g., ("/*", "*/")).
    pub block: Option<(String, String)>,
    /// Add space after line comment prefix.
    pub padding: bool,
}

impl CommentStyle {
    /// Create line-only comment style.
    pub fn line(prefix: impl Into<String>) -> Self {
        Self {
            line: Some(prefix.into()),
            block: None,
            padding: true,
        }
    }

    /// Create block-only comment style.
    pub fn block(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            line: None,
            block: Some((open.into(), close.into())),
            padding: true,
        }
    }

    /// Create style with both line and block.
    pub fn both(
        line: impl Into<String>,
        block_open: impl Into<String>,
        block_close: impl Into<String>,
    ) -> Self {
        Self {
            line: Some(line.into()),
            block: Some((block_open.into(), block_close.into())),
            padding: true,
        }
    }

    /// Disable padding.
    pub fn no_padding(mut self) -> Self {
        self.padding = false;
        self
    }

    /// Get the line comment with optional padding.
    pub fn line_prefix(&self) -> Option<String> {
        self.line.as_ref().map(|l| {
            if self.padding {
                format!("{} ", l)
            } else {
                l.clone()
            }
        })
    }
}

/// Comment registry for languages.
#[derive(Debug, Default)]
pub struct CommentRegistry {
    /// Styles by language/filetype.
    styles: HashMap<String, CommentStyle>,
}

impl CommentRegistry {
    /// Create with default language mappings.
    pub fn new() -> Self {
        let mut reg = Self::default();
        reg.register_defaults();
        reg
    }

    fn register_defaults(&mut self) {
        // C-style languages
        let c_style = CommentStyle::both("//", "/*", "*/");
        for lang in &["c", "cpp", "java", "javascript", "typescript", "rust", "go", "swift", "kotlin"] {
            self.styles.insert(lang.to_string(), c_style.clone());
        }

        // Hash-style languages
        let hash_style = CommentStyle::line("#");
        for lang in &["python", "ruby", "perl", "shell", "bash", "zsh", "yaml", "toml"] {
            self.styles.insert(lang.to_string(), hash_style.clone());
        }

        // Lisp-style
        self.styles.insert("lisp".to_string(), CommentStyle::line(";"));
        self.styles.insert("clojure".to_string(), CommentStyle::line(";"));
        self.styles.insert("scheme".to_string(), CommentStyle::line(";"));

        // Lua
        self.styles.insert("lua".to_string(), CommentStyle::both("--", "--[[", "]]"));

        // HTML/XML
        self.styles.insert("html".to_string(), CommentStyle::block("<!--", "-->"));
        self.styles.insert("xml".to_string(), CommentStyle::block("<!--", "-->"));

        // CSS
        self.styles.insert("css".to_string(), CommentStyle::block("/*", "*/"));
        self.styles.insert("scss".to_string(), CommentStyle::both("//", "/*", "*/"));

        // SQL
        self.styles.insert("sql".to_string(), CommentStyle::both("--", "/*", "*/"));

        // Haskell
        self.styles.insert("haskell".to_string(), CommentStyle::both("--", "{-", "-}"));

        // Vim
        self.styles.insert("vim".to_string(), CommentStyle::line("\""));

        // LaTeX
        self.styles.insert("latex".to_string(), CommentStyle::line("%"));
        self.styles.insert("tex".to_string(), CommentStyle::line("%"));
    }

    /// Get comment style for a language.
    pub fn get(&self, language: &str) -> Option<&CommentStyle> {
        self.styles.get(language)
    }

    /// Register custom style.
    pub fn register(&mut self, language: impl Into<String>, style: CommentStyle) {
        self.styles.insert(language.into(), style);
    }
}

/// Comment detection result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentState {
    /// Line is not commented.
    Uncommented,
    /// Line is line-commented.
    LineCommented,
    /// Line is block-commented (partial).
    BlockCommented,
}

/// Comment operation.
#[derive(Debug)]
pub struct CommentOp {
    /// Comment registry.
    registry: CommentRegistry,
}

impl Default for CommentOp {
    fn default() -> Self {
        Self::new()
    }
}

impl CommentOp {
    /// Create with default registry.
    pub fn new() -> Self {
        Self {
            registry: CommentRegistry::new(),
        }
    }

    /// Create with custom registry.
    pub fn with_registry(registry: CommentRegistry) -> Self {
        Self { registry }
    }

    /// Get the registry.
    pub fn registry(&self) -> &CommentRegistry {
        &self.registry
    }

    /// Get mutable registry.
    pub fn registry_mut(&mut self) -> &mut CommentRegistry {
        &mut self.registry
    }

    /// Check if a line is commented.
    pub fn check_line(&self, line: &str, language: &str) -> CommentState {
        let style = match self.registry.get(language) {
            Some(s) => s,
            None => return CommentState::Uncommented,
        };

        let trimmed = line.trim_start();

        // Check line comment
        if let Some(ref prefix) = style.line {
            if trimmed.starts_with(prefix) {
                return CommentState::LineCommented;
            }
        }

        // Check block comment
        if let Some((ref open, ref close)) = style.block {
            if trimmed.starts_with(open) || trimmed.ends_with(close) {
                return CommentState::BlockCommented;
            }
        }

        CommentState::Uncommented
    }

    /// Toggle line comment on a single line.
    pub fn toggle_line(&self, line: &str, language: &str) -> String {
        let style = match self.registry.get(language) {
            Some(s) => s,
            None => return line.to_string(),
        };

        let prefix = match style.line_prefix() {
            Some(p) => p,
            None => return line.to_string(),
        };

        let line_comment = style.line.as_ref().unwrap();

        // Find leading whitespace
        let leading_ws: String = line.chars().take_while(|c| c.is_whitespace()).collect();
        let content = &line[leading_ws.len()..];

        if content.starts_with(&prefix) {
            // Uncomment - remove prefix with padding
            format!("{}{}", leading_ws, &content[prefix.len()..])
        } else if content.starts_with(line_comment) {
            // Uncomment - remove prefix without padding
            format!("{}{}", leading_ws, &content[line_comment.len()..].trim_start())
        } else {
            // Comment
            format!("{}{}{}", leading_ws, prefix, content)
        }
    }

    /// Toggle line comment on multiple lines.
    pub fn toggle_lines(&self, lines: &[&str], language: &str) -> Vec<String> {
        if lines.is_empty() {
            return Vec::new();
        }

        let style = match self.registry.get(language) {
            Some(s) => s,
            None => return lines.iter().map(|l| l.to_string()).collect(),
        };

        let prefix = match style.line_prefix() {
            Some(p) => p,
            None => return lines.iter().map(|l| l.to_string()).collect(),
        };

        let line_comment = style.line.as_ref().unwrap();

        // Determine action: if all non-empty lines are commented, uncomment; else comment
        let non_empty: Vec<_> = lines.iter().filter(|l| !l.trim().is_empty()).collect();
        let all_commented = non_empty.iter().all(|l| {
            let trimmed = l.trim_start();
            trimmed.starts_with(line_comment)
        });

        // Find minimum indentation for consistent commenting
        let min_indent = lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);

        lines
            .iter()
            .map(|line| {
                if line.trim().is_empty() {
                    return line.to_string();
                }

                let leading_ws: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                let content = &line[leading_ws.len()..];

                if all_commented {
                    // Uncomment
                    if content.starts_with(&prefix) {
                        format!("{}{}", leading_ws, &content[prefix.len()..])
                    } else if content.starts_with(line_comment) {
                        format!("{}{}", leading_ws, &content[line_comment.len()..].trim_start())
                    } else {
                        line.to_string()
                    }
                } else {
                    // Comment - use min indent for alignment
                    let (ws, rest) = line.split_at(min_indent.min(line.len()));
                    format!("{}{}{}", ws, prefix, rest)
                }
            })
            .collect()
    }

    /// Add block comment around text.
    pub fn add_block_comment(&self, text: &str, language: &str) -> Option<String> {
        let style = self.registry.get(language)?;
        let (open, close) = style.block.as_ref()?;

        if style.padding {
            Some(format!("{} {} {}", open, text, close))
        } else {
            Some(format!("{}{}{}", open, text, close))
        }
    }

    /// Remove block comment from text.
    pub fn remove_block_comment(&self, text: &str, language: &str) -> Option<String> {
        let style = self.registry.get(language)?;
        let (open, close) = style.block.as_ref()?;

        let trimmed = text.trim();

        if trimmed.starts_with(open) && trimmed.ends_with(close) {
            let inner = &trimmed[open.len()..trimmed.len() - close.len()];
            Some(inner.trim().to_string())
        } else {
            None
        }
    }

    /// Toggle block comment.
    pub fn toggle_block(&self, text: &str, language: &str) -> Option<String> {
        // Try to remove first
        if let Some(uncommented) = self.remove_block_comment(text, language) {
            return Some(uncommented);
        }
        // Otherwise add
        self.add_block_comment(text, language)
    }
}

/// Comment TODO insertion.
#[derive(Debug, Clone)]
pub struct TodoComment {
    /// TODO keyword.
    pub keyword: String,
    /// Author (optional).
    pub author: Option<String>,
    /// Date (optional).
    pub date: Option<String>,
    /// Description.
    pub description: String,
}

impl TodoComment {
    /// Create a new TODO.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            keyword: "TODO".to_string(),
            author: None,
            date: None,
            description: description.into(),
        }
    }

    /// Set keyword (TODO, FIXME, HACK, etc.).
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = keyword.into();
        self
    }

    /// Set author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set date.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Format as comment.
    pub fn format(&self, language: &str, registry: &CommentRegistry) -> Option<String> {
        let style = registry.get(language)?;
        let prefix = style.line_prefix()?;

        let mut parts = vec![self.keyword.clone()];

        if let Some(ref author) = self.author {
            parts.push(format!("({})", author));
        }

        if let Some(ref date) = self.date {
            parts.push(format!("[{}]", date));
        }

        parts.push(format!(": {}", self.description));

        Some(format!("{}{}", prefix, parts.join(" ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_style_line() {
        let style = CommentStyle::line("//");
        assert_eq!(style.line, Some("//".to_string()));
        assert!(style.block.is_none());
    }

    #[test]
    fn test_comment_style_block() {
        let style = CommentStyle::block("/*", "*/");
        assert!(style.line.is_none());
        assert_eq!(style.block, Some(("/*".to_string(), "*/".to_string())));
    }

    #[test]
    fn test_comment_style_prefix() {
        let style = CommentStyle::line("//");
        assert_eq!(style.line_prefix(), Some("// ".to_string()));

        let no_pad = style.no_padding();
        assert_eq!(no_pad.line_prefix(), Some("//".to_string()));
    }

    #[test]
    fn test_registry_defaults() {
        let reg = CommentRegistry::new();
        assert!(reg.get("rust").is_some());
        assert!(reg.get("python").is_some());
        assert!(reg.get("html").is_some());
    }

    #[test]
    fn test_check_line_uncommented() {
        let op = CommentOp::new();
        assert_eq!(op.check_line("let x = 1;", "rust"), CommentState::Uncommented);
    }

    #[test]
    fn test_check_line_commented() {
        let op = CommentOp::new();
        assert_eq!(op.check_line("// let x = 1;", "rust"), CommentState::LineCommented);
        assert_eq!(op.check_line("  # comment", "python"), CommentState::LineCommented);
    }

    #[test]
    fn test_toggle_line_comment() {
        let op = CommentOp::new();

        let result = op.toggle_line("let x = 1;", "rust");
        assert_eq!(result, "// let x = 1;");

        let result = op.toggle_line("// let x = 1;", "rust");
        assert_eq!(result, "let x = 1;");
    }

    #[test]
    fn test_toggle_line_with_indent() {
        let op = CommentOp::new();

        let result = op.toggle_line("    let x = 1;", "rust");
        assert_eq!(result, "    // let x = 1;");

        let result = op.toggle_line("    // let x = 1;", "rust");
        assert_eq!(result, "    let x = 1;");
    }

    #[test]
    fn test_toggle_lines_all_uncommented() {
        let op = CommentOp::new();
        let lines = vec!["let x = 1;", "let y = 2;"];
        let result = op.toggle_lines(&lines, "rust");
        assert_eq!(result, vec!["// let x = 1;", "// let y = 2;"]);
    }

    #[test]
    fn test_toggle_lines_all_commented() {
        let op = CommentOp::new();
        let lines = vec!["// let x = 1;", "// let y = 2;"];
        let result = op.toggle_lines(&lines, "rust");
        assert_eq!(result, vec!["let x = 1;", "let y = 2;"]);
    }

    #[test]
    fn test_toggle_lines_mixed() {
        let op = CommentOp::new();
        let lines = vec!["let x = 1;", "// let y = 2;"];
        let result = op.toggle_lines(&lines, "rust");
        // Should comment all
        assert!(result[0].starts_with("//"));
        assert!(result[1].starts_with("//"));
    }

    #[test]
    fn test_block_comment_add() {
        let op = CommentOp::new();
        let result = op.add_block_comment("hello", "rust").unwrap();
        assert_eq!(result, "/* hello */");
    }

    #[test]
    fn test_block_comment_remove() {
        let op = CommentOp::new();
        let result = op.remove_block_comment("/* hello */", "rust").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_block_comment_toggle() {
        let op = CommentOp::new();

        let result = op.toggle_block("hello", "rust").unwrap();
        assert_eq!(result, "/* hello */");

        let result = op.toggle_block("/* hello */", "rust").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_todo_comment() {
        let reg = CommentRegistry::new();
        let todo = TodoComment::new("Fix this bug")
            .author("user")
            .keyword("FIXME");

        let result = todo.format("rust", &reg).unwrap();
        assert!(result.contains("FIXME"));
        assert!(result.contains("(user)"));
        assert!(result.contains("Fix this bug"));
    }
}
