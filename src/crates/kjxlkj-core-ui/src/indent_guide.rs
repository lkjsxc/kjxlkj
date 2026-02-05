//! Indent guides and scope visualization.
//!
//! Implements indent guides as specified in `/docs/spec/features/ui/indent-guides.md`.

/// Indent guide character style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndentGuideStyle {
    /// Thin vertical line (│).
    #[default]
    Line,
    /// Dashed line (┆).
    Dashed,
    /// Dotted line (┊).
    Dotted,
    /// Bold line (┃).
    Bold,
    /// ASCII pipe (|).
    Ascii,
}

impl IndentGuideStyle {
    /// Get the character for this style.
    pub fn char(&self) -> char {
        match self {
            Self::Line => '│',
            Self::Dashed => '┆',
            Self::Dotted => '┊',
            Self::Bold => '┃',
            Self::Ascii => '|',
        }
    }
}

/// Indent guide configuration.
#[derive(Debug, Clone)]
pub struct IndentGuideConfig {
    /// Enable indent guides.
    pub enabled: bool,
    /// Character style.
    pub style: IndentGuideStyle,
    /// Highlight current scope.
    pub highlight_scope: bool,
    /// Show guides through blank lines.
    pub through_blank_lines: bool,
    /// Excluded file types.
    pub exclude_filetypes: Vec<String>,
    /// Use tree-sitter for scope detection.
    pub use_treesitter: bool,
}

impl Default for IndentGuideConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            style: IndentGuideStyle::Line,
            highlight_scope: true,
            through_blank_lines: true,
            exclude_filetypes: vec![
                "help".to_string(),
                "dashboard".to_string(),
            ],
            use_treesitter: true,
        }
    }
}

impl IndentGuideConfig {
    /// Create a disabled config.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Check if guides should be shown for a filetype.
    pub fn should_show(&self, filetype: &str) -> bool {
        self.enabled && !self.exclude_filetypes.contains(&filetype.to_string())
    }

    /// Set the guide style.
    pub fn with_style(mut self, style: IndentGuideStyle) -> Self {
        self.style = style;
        self
    }

    /// Add excluded filetype.
    pub fn exclude(mut self, filetype: &str) -> Self {
        self.exclude_filetypes.push(filetype.to_string());
        self
    }
}

/// A single indent guide for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndentGuide {
    /// Column position (0-indexed).
    pub column: usize,
    /// Whether this is the active scope.
    pub active: bool,
    /// Indent level (for coloring).
    pub level: usize,
}

impl IndentGuide {
    /// Create a new indent guide.
    pub fn new(column: usize, level: usize) -> Self {
        Self {
            column,
            level,
            active: false,
        }
    }

    /// Mark as active scope.
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }
}

/// Indent guides for a line.
#[derive(Debug, Clone, Default)]
pub struct LineIndentGuides {
    /// Guides on this line.
    pub guides: Vec<IndentGuide>,
}

impl LineIndentGuides {
    /// Create empty guides.
    pub fn new() -> Self {
        Self { guides: Vec::new() }
    }

    /// Add a guide.
    pub fn add(&mut self, guide: IndentGuide) {
        self.guides.push(guide);
    }

    /// Calculate guides from line content and indent width.
    pub fn from_line(content: &str, indent_width: usize, active_level: Option<usize>) -> Self {
        let mut guides = Self::new();
        let leading_spaces = content.len() - content.trim_start().len();
        
        if indent_width == 0 {
            return guides;
        }

        let levels = leading_spaces / indent_width;
        for level in 0..levels {
            let column = level * indent_width;
            let mut guide = IndentGuide::new(column, level);
            if active_level == Some(level) {
                guide = guide.active();
            }
            guides.add(guide);
        }
        guides
    }

    /// Get guide at column.
    pub fn at_column(&self, column: usize) -> Option<&IndentGuide> {
        self.guides.iter().find(|g| g.column == column)
    }

    /// Check if any guides are active.
    pub fn has_active(&self) -> bool {
        self.guides.iter().any(|g| g.active)
    }

    /// Get the maximum level.
    pub fn max_level(&self) -> Option<usize> {
        self.guides.iter().map(|g| g.level).max()
    }
}

/// Context line display for showing scope headers.
#[derive(Debug, Clone)]
pub struct ContextLine {
    /// Line number in buffer.
    pub line: usize,
    /// Line content.
    pub content: String,
    /// Indent level.
    pub level: usize,
}

impl ContextLine {
    /// Create a new context line.
    pub fn new(line: usize, content: String, level: usize) -> Self {
        Self { line, content, level }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_guide_style_char() {
        assert_eq!(IndentGuideStyle::Line.char(), '│');
        assert_eq!(IndentGuideStyle::Dashed.char(), '┆');
        assert_eq!(IndentGuideStyle::Ascii.char(), '|');
    }

    #[test]
    fn test_indent_guide_config_default() {
        let config = IndentGuideConfig::default();
        assert!(config.enabled);
        assert!(config.highlight_scope);
    }

    #[test]
    fn test_indent_guide_config_disabled() {
        let config = IndentGuideConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_indent_guide_config_should_show() {
        let config = IndentGuideConfig::default();
        assert!(config.should_show("rust"));
        assert!(!config.should_show("help"));
    }

    #[test]
    fn test_indent_guide_config_with_style() {
        let config = IndentGuideConfig::default().with_style(IndentGuideStyle::Dashed);
        assert_eq!(config.style, IndentGuideStyle::Dashed);
    }

    #[test]
    fn test_indent_guide_config_exclude() {
        let config = IndentGuideConfig::default().exclude("markdown");
        assert!(!config.should_show("markdown"));
    }

    #[test]
    fn test_indent_guide_new() {
        let guide = IndentGuide::new(4, 1);
        assert_eq!(guide.column, 4);
        assert_eq!(guide.level, 1);
        assert!(!guide.active);
    }

    #[test]
    fn test_indent_guide_active() {
        let guide = IndentGuide::new(4, 1).active();
        assert!(guide.active);
    }

    #[test]
    fn test_line_indent_guides_from_line() {
        let guides = LineIndentGuides::from_line("        code", 4, None);
        assert_eq!(guides.guides.len(), 2);
        assert_eq!(guides.guides[0].column, 0);
        assert_eq!(guides.guides[1].column, 4);
    }

    #[test]
    fn test_line_indent_guides_with_active() {
        let guides = LineIndentGuides::from_line("        code", 4, Some(1));
        assert!(guides.has_active());
        assert!(guides.guides[1].active);
    }

    #[test]
    fn test_line_indent_guides_at_column() {
        let guides = LineIndentGuides::from_line("    code", 4, None);
        assert!(guides.at_column(0).is_some());
        assert!(guides.at_column(4).is_none());
    }

    #[test]
    fn test_line_indent_guides_max_level() {
        let guides = LineIndentGuides::from_line("            code", 4, None);
        assert_eq!(guides.max_level(), Some(2));
    }

    #[test]
    fn test_line_indent_guides_empty() {
        let guides = LineIndentGuides::from_line("code", 4, None);
        assert!(guides.guides.is_empty());
    }

    #[test]
    fn test_line_indent_guides_zero_width() {
        let guides = LineIndentGuides::from_line("    code", 0, None);
        assert!(guides.guides.is_empty());
    }

    #[test]
    fn test_context_line_new() {
        let cl = ContextLine::new(10, "fn main() {".to_string(), 0);
        assert_eq!(cl.line, 10);
        assert_eq!(cl.level, 0);
    }
}
