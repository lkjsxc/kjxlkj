//! Syntax highlighting module.
//!
//! Provides syntax highlighting using tree-sitter, LSP semantic tokens,
//! or regex patterns as fallback. Also includes folding and inlay hints.

use std::collections::HashMap;
use std::ops::Range;

/// A highlight group name.
pub type HighlightGroup = String;

/// Standard highlight groups for editor UI and syntax.
/// These match Vim/Neovim conventions.
pub mod groups {
    // Editor UI groups
    pub const NORMAL: &str = "Normal";
    pub const NORMAL_NC: &str = "NormalNC";
    pub const VISUAL: &str = "Visual";
    pub const CURSOR: &str = "Cursor";
    pub const CURSOR_LINE: &str = "CursorLine";
    pub const CURSOR_COLUMN: &str = "CursorColumn";
    pub const COLOR_COLUMN: &str = "ColorColumn";
    pub const LINE_NR: &str = "LineNr";
    pub const CURSOR_LINE_NR: &str = "CursorLineNr";
    pub const SIGN_COLUMN: &str = "SignColumn";
    pub const FOLD_COLUMN: &str = "FoldColumn";
    pub const FOLDED: &str = "Folded";
    pub const VERT_SPLIT: &str = "VertSplit";
    pub const WIN_SEPARATOR: &str = "WinSeparator";

    // Status/Tab groups
    pub const STATUS_LINE: &str = "StatusLine";
    pub const STATUS_LINE_NC: &str = "StatusLineNC";
    pub const TAB_LINE: &str = "TabLine";
    pub const TAB_LINE_FILL: &str = "TabLineFill";
    pub const TAB_LINE_SEL: &str = "TabLineSel";
    pub const WILD_MENU: &str = "WildMenu";

    // Message groups
    pub const MODE_MSG: &str = "ModeMsg";
    pub const MORE_MSG: &str = "MoreMsg";
    pub const QUESTION: &str = "Question";
    pub const WARNING_MSG: &str = "WarningMsg";
    pub const ERROR_MSG: &str = "ErrorMsg";

    // Search groups
    pub const SEARCH: &str = "Search";
    pub const INC_SEARCH: &str = "IncSearch";
    pub const CUR_SEARCH: &str = "CurSearch";
    pub const SUBSTITUTE: &str = "Substitute";

    // Popup/Float groups
    pub const PMENU: &str = "Pmenu";
    pub const PMENU_SEL: &str = "PmenuSel";
    pub const PMENU_SBAR: &str = "PmenuSbar";
    pub const PMENU_THUMB: &str = "PmenuThumb";
    pub const FLOAT_BORDER: &str = "FloatBorder";
    pub const FLOAT_TITLE: &str = "FloatTitle";

    // Syntax groups - Standard
    pub const COMMENT: &str = "Comment";
    pub const CONSTANT: &str = "Constant";
    pub const STRING: &str = "String";
    pub const CHARACTER: &str = "Character";
    pub const NUMBER: &str = "Number";
    pub const BOOLEAN: &str = "Boolean";
    pub const FLOAT: &str = "Float";

    // Syntax groups - Identifiers
    pub const IDENTIFIER: &str = "Identifier";
    pub const FUNCTION: &str = "Function";
    pub const STATEMENT: &str = "Statement";
    pub const CONDITIONAL: &str = "Conditional";
    pub const REPEAT: &str = "Repeat";
    pub const LABEL: &str = "Label";
    pub const OPERATOR: &str = "Operator";
    pub const KEYWORD: &str = "Keyword";

    // Syntax groups - Types
    pub const TYPE: &str = "Type";
    pub const STORAGE_CLASS: &str = "StorageClass";
    pub const STRUCTURE: &str = "Structure";
    pub const TYPEDEF: &str = "Typedef";

    // Syntax groups - Special
    pub const SPECIAL: &str = "Special";
    pub const SPECIAL_CHAR: &str = "SpecialChar";
    pub const TAG: &str = "Tag";
    pub const DELIMITER: &str = "Delimiter";
    pub const SPECIAL_COMMENT: &str = "SpecialComment";

    // Diagnostic groups
    pub const DIAGNOSTIC_ERROR: &str = "DiagnosticError";
    pub const DIAGNOSTIC_WARN: &str = "DiagnosticWarn";
    pub const DIAGNOSTIC_INFO: &str = "DiagnosticInfo";
    pub const DIAGNOSTIC_HINT: &str = "DiagnosticHint";
    pub const DIAGNOSTIC_UNDERLINE_ERROR: &str = "DiagnosticUnderlineError";
    pub const DIAGNOSTIC_UNDERLINE_WARN: &str = "DiagnosticUnderlineWarn";

    // Git/Diff groups
    pub const DIFF_ADD: &str = "DiffAdd";
    pub const DIFF_CHANGE: &str = "DiffChange";
    pub const DIFF_DELETE: &str = "DiffDelete";
    pub const DIFF_TEXT: &str = "DiffText";
    pub const GIT_SIGNS_ADD: &str = "GitSignsAdd";
    pub const GIT_SIGNS_CHANGE: &str = "GitSignsChange";
    pub const GIT_SIGNS_DELETE: &str = "GitSignsDelete";

    // Tree-sitter capture groups
    pub const TS_COMMENT: &str = "@comment";
    pub const TS_KEYWORD: &str = "@keyword";
    pub const TS_STRING: &str = "@string";
    pub const TS_NUMBER: &str = "@number";
    pub const TS_FUNCTION: &str = "@function";
    pub const TS_TYPE: &str = "@type";
    pub const TS_VARIABLE: &str = "@variable";
    pub const TS_CONSTANT: &str = "@constant";
    pub const TS_OPERATOR: &str = "@operator";
    pub const TS_PUNCTUATION: &str = "@punctuation";
}

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

// ============================================================================
// Folding
// ============================================================================

/// Fold method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FoldMethod {
    /// Tree-sitter based folding.
    #[default]
    TreeSitter,
    /// Indent-based folding.
    Indent,
    /// Manual folding (zf + motion).
    Manual,
    /// Marker-based folding ({{{ / }}}).
    Marker,
    /// Syntax-based folding.
    Syntax,
    /// Expression-based folding.
    Expr,
}

/// A single fold region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fold {
    /// Start line (0-indexed).
    pub start: usize,
    /// End line (0-indexed, inclusive).
    pub end: usize,
    /// Fold level (1 = top-level).
    pub level: u8,
    /// Whether the fold is closed.
    pub closed: bool,
    /// Optional fold text/summary.
    pub text: Option<String>,
}

impl Fold {
    /// Create a new fold.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            level: 1,
            closed: false,
            text: None,
        }
    }

    /// Set fold level.
    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    /// Set fold text.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Number of lines in fold.
    pub fn line_count(&self) -> usize {
        self.end - self.start + 1
    }

    /// Toggle fold state.
    pub fn toggle(&mut self) {
        self.closed = !self.closed;
    }

    /// Check if a line is within this fold.
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
}

/// Fold state for a buffer.
#[derive(Debug, Default)]
pub struct FoldState {
    /// All folds in the buffer.
    folds: Vec<Fold>,
    /// Fold method.
    method: FoldMethod,
    /// Minimum fold size (lines).
    min_lines: usize,
    /// Maximum fold level to auto-close.
    fold_level: u8,
    /// Show fold column.
    fold_column: u8,
}

impl FoldState {
    /// Create new fold state.
    pub fn new() -> Self {
        Self {
            folds: Vec::new(),
            method: FoldMethod::TreeSitter,
            min_lines: 1,
            fold_level: 99,
            fold_column: 0,
        }
    }

    /// Set fold method.
    pub fn set_method(&mut self, method: FoldMethod) {
        self.method = method;
    }

    /// Get fold method.
    pub fn method(&self) -> FoldMethod {
        self.method
    }

    /// Add a fold.
    pub fn add(&mut self, fold: Fold) {
        if fold.line_count() > self.min_lines {
            self.folds.push(fold);
        }
    }

    /// Get all folds.
    pub fn folds(&self) -> &[Fold] {
        &self.folds
    }

    /// Get fold at line.
    pub fn fold_at(&self, line: usize) -> Option<&Fold> {
        self.folds.iter().find(|f| f.start == line)
    }

    /// Get mutable fold at line.
    pub fn fold_at_mut(&mut self, line: usize) -> Option<&mut Fold> {
        self.folds.iter_mut().find(|f| f.start == line)
    }

    /// Check if line is folded (hidden).
    pub fn is_folded(&self, line: usize) -> bool {
        self.folds.iter().any(|f| f.closed && f.contains(line) && f.start != line)
    }

    /// Toggle fold at line.
    pub fn toggle(&mut self, line: usize) {
        if let Some(fold) = self.fold_at_mut(line) {
            fold.toggle();
        }
    }

    /// Open fold at line.
    pub fn open(&mut self, line: usize) {
        if let Some(fold) = self.fold_at_mut(line) {
            fold.closed = false;
        }
    }

    /// Close fold at line.
    pub fn close(&mut self, line: usize) {
        if let Some(fold) = self.fold_at_mut(line) {
            fold.closed = true;
        }
    }

    /// Open all folds.
    pub fn open_all(&mut self) {
        for fold in &mut self.folds {
            fold.closed = false;
        }
    }

    /// Close all folds.
    pub fn close_all(&mut self) {
        for fold in &mut self.folds {
            fold.closed = true;
        }
    }

    /// Clear all folds.
    pub fn clear(&mut self) {
        self.folds.clear();
    }

    /// Set fold column width.
    pub fn set_fold_column(&mut self, width: u8) {
        self.fold_column = width;
    }

    /// Get fold column width.
    pub fn fold_column(&self) -> u8 {
        self.fold_column
    }

    /// Get minimum fold size.
    pub fn min_lines(&self) -> usize {
        self.min_lines
    }

    /// Set minimum fold size.
    pub fn set_min_lines(&mut self, min: usize) {
        self.min_lines = min;
    }

    /// Get fold level (for auto-close).
    pub fn fold_level(&self) -> u8 {
        self.fold_level
    }

    /// Set fold level.
    pub fn set_fold_level(&mut self, level: u8) {
        self.fold_level = level;
    }
}

// ============================================================================
// Inlay Hints
// ============================================================================

/// Type of inlay hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintKind {
    /// Type annotation hint.
    Type,
    /// Parameter name hint.
    Parameter,
    /// Chained method hint.
    Chaining,
}

/// A single inlay hint.
#[derive(Debug, Clone)]
pub struct InlayHint {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column position (0-indexed).
    pub column: usize,
    /// Hint text.
    pub text: String,
    /// Kind of hint.
    pub kind: InlayHintKind,
    /// Whether hint is interactive (can be inserted).
    pub interactive: bool,
}

impl InlayHint {
    /// Create a new inlay hint.
    pub fn new(line: usize, column: usize, text: impl Into<String>, kind: InlayHintKind) -> Self {
        Self {
            line,
            column,
            text: text.into(),
            kind,
            interactive: false,
        }
    }

    /// Mark as interactive.
    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
}

/// Inlay hint state for a buffer.
#[derive(Debug, Default)]
pub struct InlayHintState {
    /// All hints in the buffer.
    hints: Vec<InlayHint>,
    /// Whether hints are enabled.
    enabled: bool,
    /// Show type hints.
    show_types: bool,
    /// Show parameter hints.
    show_parameters: bool,
    /// Show chaining hints.
    show_chaining: bool,
    /// Delay before showing hints (ms).
    delay_ms: u32,
}

impl InlayHintState {
    /// Create new inlay hint state.
    pub fn new() -> Self {
        Self {
            hints: Vec::new(),
            enabled: true,
            show_types: true,
            show_parameters: true,
            show_chaining: false,
            delay_ms: 500,
        }
    }

    /// Enable/disable hints.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if hints are enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Toggle hints.
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Add a hint.
    pub fn add(&mut self, hint: InlayHint) {
        self.hints.push(hint);
    }

    /// Get hints for a line.
    pub fn hints_for_line(&self, line: usize) -> Vec<&InlayHint> {
        if !self.enabled {
            return Vec::new();
        }
        self.hints
            .iter()
            .filter(|h| {
                h.line == line
                    && match h.kind {
                        InlayHintKind::Type => self.show_types,
                        InlayHintKind::Parameter => self.show_parameters,
                        InlayHintKind::Chaining => self.show_chaining,
                    }
            })
            .collect()
    }

    /// Get all hints.
    pub fn hints(&self) -> &[InlayHint] {
        &self.hints
    }

    /// Clear all hints.
    pub fn clear(&mut self) {
        self.hints.clear();
    }

    /// Set hint visibility options.
    pub fn set_visibility(&mut self, types: bool, parameters: bool, chaining: bool) {
        self.show_types = types;
        self.show_parameters = parameters;
        self.show_chaining = chaining;
    }

    /// Get delay in milliseconds.
    pub fn delay_ms(&self) -> u32 {
        self.delay_ms
    }

    /// Set delay in milliseconds.
    pub fn set_delay_ms(&mut self, delay: u32) {
        self.delay_ms = delay;
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

    // Fold tests
    #[test]
    fn test_fold_new() {
        let fold = Fold::new(5, 10);
        assert_eq!(fold.start, 5);
        assert_eq!(fold.end, 10);
        assert_eq!(fold.level, 1);
        assert!(!fold.closed);
        assert_eq!(fold.line_count(), 6);
    }

    #[test]
    fn test_fold_contains() {
        let fold = Fold::new(5, 10);
        assert!(!fold.contains(4));
        assert!(fold.contains(5));
        assert!(fold.contains(7));
        assert!(fold.contains(10));
        assert!(!fold.contains(11));
    }

    #[test]
    fn test_fold_toggle() {
        let mut fold = Fold::new(0, 5);
        assert!(!fold.closed);
        fold.toggle();
        assert!(fold.closed);
        fold.toggle();
        assert!(!fold.closed);
    }

    #[test]
    fn test_fold_state() {
        let mut fs = FoldState::new();
        fs.add(Fold::new(0, 5));
        fs.add(Fold::new(10, 15));

        assert_eq!(fs.folds().len(), 2);
        assert!(fs.fold_at(0).is_some());
        assert!(fs.fold_at(10).is_some());
        assert!(fs.fold_at(5).is_none());
    }

    #[test]
    fn test_fold_state_toggle() {
        let mut fs = FoldState::new();
        fs.add(Fold::new(0, 5));

        assert!(!fs.fold_at(0).unwrap().closed);
        fs.toggle(0);
        assert!(fs.fold_at(0).unwrap().closed);
    }

    #[test]
    fn test_fold_state_is_folded() {
        let mut fs = FoldState::new();
        fs.add(Fold::new(5, 10));

        assert!(!fs.is_folded(6)); // Not folded yet

        fs.close(5);
        assert!(!fs.is_folded(5)); // Start line is never hidden
        assert!(fs.is_folded(6));
        assert!(fs.is_folded(10));
        assert!(!fs.is_folded(11));
    }

    #[test]
    fn test_fold_state_open_close_all() {
        let mut fs = FoldState::new();
        fs.add(Fold::new(0, 5));
        fs.add(Fold::new(10, 15));

        fs.close_all();
        assert!(fs.folds().iter().all(|f| f.closed));

        fs.open_all();
        assert!(fs.folds().iter().all(|f| !f.closed));
    }

    // Inlay hint tests
    #[test]
    fn test_inlay_hint_new() {
        let hint = InlayHint::new(5, 10, ": i32", InlayHintKind::Type);
        assert_eq!(hint.line, 5);
        assert_eq!(hint.column, 10);
        assert_eq!(hint.text, ": i32");
        assert_eq!(hint.kind, InlayHintKind::Type);
        assert!(!hint.interactive);
    }

    #[test]
    fn test_inlay_hint_state() {
        let mut state = InlayHintState::new();
        state.add(InlayHint::new(0, 5, ": String", InlayHintKind::Type));
        state.add(InlayHint::new(0, 10, "name", InlayHintKind::Parameter));
        state.add(InlayHint::new(1, 0, ": i32", InlayHintKind::Type));

        assert_eq!(state.hints().len(), 3);
        assert_eq!(state.hints_for_line(0).len(), 2);
        assert_eq!(state.hints_for_line(1).len(), 1);
    }

    #[test]
    fn test_inlay_hint_toggle() {
        let mut state = InlayHintState::new();
        state.add(InlayHint::new(0, 5, ": i32", InlayHintKind::Type));

        assert!(state.enabled());
        assert_eq!(state.hints_for_line(0).len(), 1);

        state.toggle();
        assert!(!state.enabled());
        assert_eq!(state.hints_for_line(0).len(), 0);
    }

    #[test]
    fn test_inlay_hint_visibility() {
        let mut state = InlayHintState::new();
        state.add(InlayHint::new(0, 5, ": i32", InlayHintKind::Type));
        state.add(InlayHint::new(0, 10, "x", InlayHintKind::Parameter));

        // Both visible by default
        assert_eq!(state.hints_for_line(0).len(), 2);

        // Disable type hints
        state.set_visibility(false, true, false);
        let hints = state.hints_for_line(0);
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].kind, InlayHintKind::Parameter);
    }

    #[test]
    fn test_highlight_groups_constants() {
        assert_eq!(groups::NORMAL, "Normal");
        assert_eq!(groups::COMMENT, "Comment");
        assert_eq!(groups::TS_FUNCTION, "@function");
    }
}
