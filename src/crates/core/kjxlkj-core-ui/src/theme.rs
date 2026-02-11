//! Highlight groups and theme model for syntax and UI styling.
//! See /docs/spec/features/ui/cursor-customization.md, /docs/spec/features/syntax/syntax.md.

/// Named highlight groups used by syntax highlighting and UI elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HlGroup {
    // Syntax
    Keyword, Type, Function, Variable, String, Number, Comment,
    Operator, Punctuation, Constant, Namespace, Macro, Attribute,
    // UI
    Normal, StatusLine, StatusLineNC, LineNr, CursorLineNr, CursorLine,
    Visual, Search, IncSearch, MatchParen, Pmenu, PmenuSel, PmenuThumb,
    Error, Warning, Info, Hint,
    DiffAdd, DiffChange, DiffDelete,
    SignColumn, FoldColumn, Folded, NonText,
    TabLine, TabLineSel, TabLineFill,
    Title, Directory,
}

/// RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color(pub u8, pub u8, pub u8);

/// Style attributes for a highlight group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub reverse: bool,
}

impl Style {
    pub const fn plain() -> Self {
        Self { fg: None, bg: None, bold: false, italic: false,
            underline: false, strikethrough: false, reverse: false }
    }
    pub const fn fg(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg = Some(Color(r, g, b)); self
    }
    pub const fn bg(mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg = Some(Color(r, g, b)); self
    }
    pub const fn bold(mut self) -> Self { self.bold = true; self }
    pub const fn italic(mut self) -> Self { self.italic = true; self }
    pub const fn underline(mut self) -> Self { self.underline = true; self }
}

/// Theme mapping highlight groups to styles.
#[derive(Debug)]
pub struct Theme {
    pub name: String,
    styles: std::collections::HashMap<HlGroup, Style>,
}

impl Theme {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), styles: std::collections::HashMap::new() }
    }
    pub fn set(&mut self, group: HlGroup, style: Style) { self.styles.insert(group, style); }
    pub fn get(&self, group: HlGroup) -> &Style {
        self.styles.get(&group).unwrap_or(&DEFAULT_STYLE)
    }
    pub fn groups(&self) -> impl Iterator<Item = (&HlGroup, &Style)> { self.styles.iter() }
}

static DEFAULT_STYLE: Style = Style::plain();

/// Built-in dark theme (sensible defaults).
pub fn default_dark() -> Theme {
    let mut t = Theme::new("default-dark");
    t.set(HlGroup::Normal, Style::plain().fg(204, 204, 204).bg(30, 30, 30));
    t.set(HlGroup::Keyword, Style::plain().fg(198, 120, 221).bold());
    t.set(HlGroup::Type, Style::plain().fg(229, 192, 123));
    t.set(HlGroup::Function, Style::plain().fg(97, 175, 239));
    t.set(HlGroup::Variable, Style::plain().fg(224, 108, 117));
    t.set(HlGroup::String, Style::plain().fg(152, 195, 121));
    t.set(HlGroup::Number, Style::plain().fg(209, 154, 102));
    t.set(HlGroup::Comment, Style::plain().fg(92, 99, 112).italic());
    t.set(HlGroup::Operator, Style::plain().fg(86, 182, 194));
    t.set(HlGroup::Constant, Style::plain().fg(209, 154, 102).bold());
    t.set(HlGroup::StatusLine, Style::plain().fg(204, 204, 204).bg(55, 55, 55));
    t.set(HlGroup::StatusLineNC, Style::plain().fg(128, 128, 128).bg(40, 40, 40));
    t.set(HlGroup::LineNr, Style::plain().fg(80, 80, 80));
    t.set(HlGroup::CursorLineNr, Style::plain().fg(204, 204, 204).bold());
    t.set(HlGroup::CursorLine, Style::plain().bg(40, 44, 52));
    t.set(HlGroup::Visual, Style::plain().bg(61, 69, 82));
    t.set(HlGroup::Search, Style::plain().fg(30, 30, 30).bg(229, 192, 123));
    t.set(HlGroup::IncSearch, Style::plain().fg(30, 30, 30).bg(86, 182, 194));
    t.set(HlGroup::MatchParen, Style::plain().fg(86, 182, 194).bold().underline());
    t.set(HlGroup::Error, Style::plain().fg(224, 108, 117));
    t.set(HlGroup::Warning, Style::plain().fg(229, 192, 123));
    t.set(HlGroup::Info, Style::plain().fg(97, 175, 239));
    t.set(HlGroup::Hint, Style::plain().fg(86, 182, 194));
    t.set(HlGroup::DiffAdd, Style::plain().fg(152, 195, 121));
    t.set(HlGroup::DiffChange, Style::plain().fg(229, 192, 123));
    t.set(HlGroup::DiffDelete, Style::plain().fg(224, 108, 117));
    t.set(HlGroup::Folded, Style::plain().fg(92, 99, 112).bg(40, 44, 52));
    t.set(HlGroup::NonText, Style::plain().fg(60, 60, 60));
    t.set(HlGroup::Pmenu, Style::plain().fg(204, 204, 204).bg(45, 45, 45));
    t.set(HlGroup::PmenuSel, Style::plain().fg(30, 30, 30).bg(97, 175, 239));
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_dark_has_normal() {
        let t = default_dark();
        let s = t.get(HlGroup::Normal);
        assert!(s.fg.is_some());
        assert!(s.bg.is_some());
    }
    #[test]
    fn get_missing_returns_plain() {
        let t = Theme::new("empty");
        let s = t.get(HlGroup::Title);
        assert!(s.fg.is_none());
        assert!(!s.bold);
    }
    #[test]
    fn set_overrides() {
        let mut t = Theme::new("test");
        t.set(HlGroup::Keyword, Style::plain().fg(255, 0, 0));
        let s = t.get(HlGroup::Keyword);
        assert_eq!(s.fg, Some(Color(255, 0, 0)));
    }
    #[test]
    fn keyword_is_bold_in_default() {
        let t = default_dark();
        assert!(t.get(HlGroup::Keyword).bold);
    }
    #[test]
    fn comment_is_italic_in_default() {
        let t = default_dark();
        assert!(t.get(HlGroup::Comment).italic);
    }
    #[test]
    fn style_builder_chain() {
        let s = Style::plain().fg(10, 20, 30).bg(40, 50, 60).bold().italic().underline();
        assert_eq!(s.fg, Some(Color(10, 20, 30)));
        assert_eq!(s.bg, Some(Color(40, 50, 60)));
        assert!(s.bold && s.italic && s.underline);
        assert!(!s.strikethrough && !s.reverse);
    }
    #[test]
    fn groups_iterator() {
        let t = default_dark();
        let count = t.groups().count();
        assert!(count >= 25); // at least 25 groups in default dark
    }
}
