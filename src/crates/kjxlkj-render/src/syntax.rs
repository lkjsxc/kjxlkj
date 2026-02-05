//! Syntax highlighting module.
//!
//! Provides syntax highlighting using tree-sitter, LSP semantic tokens,
//! or regex patterns as fallback.

use std::collections::HashMap;
use std::ops::Range;

/// A highlight group name.
pub type HighlightGroup = String;

/// A color value (can be hex or name).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Color {
    /// Named color.
    Named(String),
    /// RGB hex color.
    Rgb(u8, u8, u8),
    /// Default/none.
    #[default]
    None,
}

impl Color {
    /// Create from hex string (e.g., "#ff0000").
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self::Rgb(r, g, b))
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> Option<String> {
        match self {
            Self::Rgb(r, g, b) => Some(format!("#{:02x}{:02x}{:02x}", r, g, b)),
            _ => None,
        }
    }
}

/// Text style attributes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Style {
    /// Foreground color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
    /// Strikethrough.
    pub strikethrough: bool,
}

impl Style {
    /// Create a new empty style.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set foreground color.
    pub fn with_fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set background color.
    pub fn with_bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set bold.
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set italic.
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Set underline.
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// Merge another style on top of this one.
    pub fn merge(&self, other: &Style) -> Style {
        Style {
            fg: if other.fg != Color::None {
                other.fg.clone()
            } else {
                self.fg.clone()
            },
            bg: if other.bg != Color::None {
                other.bg.clone()
            } else {
                self.bg.clone()
            },
            bold: self.bold || other.bold,
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
            strikethrough: self.strikethrough || other.strikethrough,
        }
    }
}

/// A highlight span in a line.
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Column range (0-indexed, exclusive end).
    pub range: Range<usize>,
    /// Highlight group.
    pub group: HighlightGroup,
}

impl HighlightSpan {
    /// Create a new highlight span.
    pub fn new(range: Range<usize>, group: impl Into<HighlightGroup>) -> Self {
        Self {
            range,
            group: group.into(),
        }
    }
}

/// Highlights for a single line.
#[derive(Debug, Clone, Default)]
pub struct LineHighlights {
    /// Spans in this line.
    pub spans: Vec<HighlightSpan>,
}

impl LineHighlights {
    /// Create empty highlights.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a span.
    pub fn add(&mut self, span: HighlightSpan) {
        self.spans.push(span);
    }

    /// Sort spans by start position.
    pub fn sort(&mut self) {
        self.spans.sort_by_key(|s| s.range.start);
    }
}

/// A colorscheme mapping highlight groups to styles.
#[derive(Debug, Clone, Default)]
pub struct Colorscheme {
    /// Name of the colorscheme.
    pub name: String,
    /// Group to style mappings.
    groups: HashMap<HighlightGroup, Style>,
    /// Link from one group to another.
    links: HashMap<HighlightGroup, HighlightGroup>,
}

impl Colorscheme {
    /// Create a new colorscheme.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            groups: HashMap::new(),
            links: HashMap::new(),
        }
    }

    /// Create default colorscheme with common groups.
    pub fn default_scheme() -> Self {
        let mut cs = Self::new("default");

        // Basic groups
        cs.set(
            "Normal",
            Style::new().with_fg(Color::Named("white".into())),
        );
        cs.set(
            "Comment",
            Style::new()
                .with_fg(Color::Named("gray".into()))
                .with_italic(true),
        );
        cs.set(
            "Keyword",
            Style::new()
                .with_fg(Color::Named("blue".into()))
                .with_bold(true),
        );
        cs.set("String", Style::new().with_fg(Color::Named("green".into())));
        cs.set("Number", Style::new().with_fg(Color::Named("cyan".into())));
        cs.set(
            "Function",
            Style::new().with_fg(Color::Named("yellow".into())),
        );
        cs.set("Type", Style::new().with_fg(Color::Named("magenta".into())));
        cs.set(
            "Error",
            Style::new()
                .with_fg(Color::Named("red".into()))
                .with_underline(true),
        );
        cs.set(
            "Warning",
            Style::new()
                .with_fg(Color::Named("yellow".into()))
                .with_underline(true),
        );

        // Links for tree-sitter groups
        cs.link("@comment", "Comment");
        cs.link("@keyword", "Keyword");
        cs.link("@string", "String");
        cs.link("@number", "Number");
        cs.link("@function", "Function");
        cs.link("@type", "Type");

        cs
    }

    /// Set a style for a group.
    pub fn set(&mut self, group: impl Into<HighlightGroup>, style: Style) {
        self.groups.insert(group.into(), style);
    }

    /// Link one group to another.
    pub fn link(&mut self, from: impl Into<HighlightGroup>, to: impl Into<HighlightGroup>) {
        self.links.insert(from.into(), to.into());
    }

    /// Get style for a group (follows links).
    pub fn get(&self, group: &str) -> Option<&Style> {
        // Check direct definition
        if let Some(style) = self.groups.get(group) {
            return Some(style);
        }

        // Follow links
        let mut current = group;
        let mut visited = Vec::new();
        while let Some(target) = self.links.get(current) {
            if visited.contains(&target.as_str()) {
                // Circular link, break
                break;
            }
            visited.push(current);
            if let Some(style) = self.groups.get(target) {
                return Some(style);
            }
            current = target;
        }

        None
    }
}

/// Source of highlight information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightSource {
    /// Tree-sitter parser.
    TreeSitter,
    /// LSP semantic tokens.
    SemanticTokens,
    /// Regex-based fallback.
    Regex,
    /// Manual/vim-style syntax.
    VimSyntax,
}

/// Syntax state for a buffer.
#[derive(Debug, Default)]
pub struct SyntaxState {
    /// Line highlights.
    lines: HashMap<usize, LineHighlights>,
    /// Source of current highlights.
    source: Option<HighlightSource>,
    /// Language ID.
    language: Option<String>,
}

impl SyntaxState {
    /// Create new syntax state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the language.
    pub fn set_language(&mut self, lang: impl Into<String>) {
        self.language = Some(lang.into());
    }

    /// Get the language.
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Set highlights for a line.
    pub fn set_line(&mut self, line: usize, highlights: LineHighlights) {
        self.lines.insert(line, highlights);
    }

    /// Get highlights for a line.
    pub fn get_line(&self, line: usize) -> Option<&LineHighlights> {
        self.lines.get(&line)
    }

    /// Set highlight source.
    pub fn set_source(&mut self, source: HighlightSource) {
        self.source = Some(source);
    }

    /// Get highlight source.
    pub fn source(&self) -> Option<HighlightSource> {
        self.source
    }

    /// Clear all highlights.
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Invalidate lines after a change.
    pub fn invalidate_from(&mut self, line: usize) {
        self.lines.retain(|&l, _| l < line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert!(matches!(color, Color::Rgb(255, 0, 0)));
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::Rgb(255, 128, 0);
        assert_eq!(color.to_hex(), Some("#ff8000".to_string()));
    }

    #[test]
    fn test_style_merge() {
        let base = Style::new()
            .with_fg(Color::Named("white".into()))
            .with_bold(true);
        let overlay = Style::new()
            .with_fg(Color::Named("red".into()))
            .with_italic(true);

        let merged = base.merge(&overlay);
        assert_eq!(merged.fg, Color::Named("red".into()));
        assert!(merged.bold);
        assert!(merged.italic);
    }

    #[test]
    fn test_highlight_span() {
        let span = HighlightSpan::new(0..5, "Comment");
        assert_eq!(span.range, 0..5);
        assert_eq!(span.group, "Comment");
    }

    #[test]
    fn test_line_highlights() {
        let mut lh = LineHighlights::new();
        lh.add(HighlightSpan::new(10..15, "String"));
        lh.add(HighlightSpan::new(0..5, "Comment"));
        lh.sort();

        assert_eq!(lh.spans[0].range.start, 0);
        assert_eq!(lh.spans[1].range.start, 10);
    }

    #[test]
    fn test_colorscheme_set_get() {
        let mut cs = Colorscheme::new("test");
        cs.set("Comment", Style::new().with_italic(true));

        let style = cs.get("Comment").unwrap();
        assert!(style.italic);
    }

    #[test]
    fn test_colorscheme_link() {
        let mut cs = Colorscheme::new("test");
        cs.set("Comment", Style::new().with_italic(true));
        cs.link("@comment", "Comment");

        let style = cs.get("@comment").unwrap();
        assert!(style.italic);
    }

    #[test]
    fn test_colorscheme_default() {
        let cs = Colorscheme::default_scheme();
        assert!(cs.get("Comment").is_some());
        assert!(cs.get("@comment").is_some());
    }

    #[test]
    fn test_syntax_state_lines() {
        let mut state = SyntaxState::new();
        let mut lh = LineHighlights::new();
        lh.add(HighlightSpan::new(0..5, "Comment"));
        state.set_line(0, lh);

        assert!(state.get_line(0).is_some());
        assert!(state.get_line(1).is_none());
    }

    #[test]
    fn test_syntax_state_invalidate() {
        let mut state = SyntaxState::new();
        state.set_line(0, LineHighlights::new());
        state.set_line(5, LineHighlights::new());
        state.set_line(10, LineHighlights::new());

        state.invalidate_from(5);

        assert!(state.get_line(0).is_some());
        assert!(state.get_line(5).is_none());
        assert!(state.get_line(10).is_none());
    }

    #[test]
    fn test_syntax_state_language() {
        let mut state = SyntaxState::new();
        assert!(state.language().is_none());

        state.set_language("rust");
        assert_eq!(state.language(), Some("rust"));
    }
}
