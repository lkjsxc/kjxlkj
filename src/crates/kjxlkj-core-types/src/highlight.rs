//! Syntax highlight group model.

use crate::Style;

/// Standard highlight groups for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighlightGroup {
    Normal,
    Comment,
    Keyword,
    String,
    Number,
    Identifier,
    Type,
    Function,
    Operator,
    Delimiter,
    PreProc,
    Constant,
    Special,
    Error,
    Warning,
    Todo,
    StatusLine,
    LineNr,
    CursorLine,
    Visual,
    Search,
    MatchParen,
    Pmenu,
    PmenuSel,
}

impl HighlightGroup {
    /// Default style for a highlight group.
    pub fn default_style(&self) -> Style {
        use crate::Color;
        match self {
            Self::Comment => Style::default().fg(Color::DarkGrey).italic(),
            Self::Keyword => Style::default().fg(Color::Blue).bold(),
            Self::String => Style::default().fg(Color::Green),
            Self::Number => Style::default().fg(Color::Magenta),
            Self::Type => Style::default().fg(Color::Cyan),
            Self::Function => Style::default().fg(Color::Yellow),
            Self::Operator => Style::default().fg(Color::White),
            Self::Error => Style::default().fg(Color::Red).bold(),
            Self::Warning => Style::default().fg(Color::Yellow),
            Self::Todo => Style::default().fg(Color::Yellow).bold(),
            Self::StatusLine => Style::default().fg(Color::Black).bg(Color::White),
            Self::LineNr => Style::default().fg(Color::DarkGrey),
            Self::Visual => Style::default().bg(Color::Blue),
            Self::Search => Style::default().fg(Color::Black).bg(Color::Yellow),
            Self::MatchParen => Style::default().bg(Color::DarkGrey).bold(),
            Self::Pmenu => Style::default().bg(Color::DarkGrey),
            Self::PmenuSel => Style::default().bg(Color::Blue),
            _ => Style::default(),
        }
    }

    /// Parse a highlight group name.
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "Normal" => Some(Self::Normal),
            "Comment" => Some(Self::Comment),
            "Keyword" => Some(Self::Keyword),
            "String" => Some(Self::String),
            "Number" => Some(Self::Number),
            "Identifier" => Some(Self::Identifier),
            "Type" => Some(Self::Type),
            "Function" => Some(Self::Function),
            "Operator" => Some(Self::Operator),
            "Delimiter" => Some(Self::Delimiter),
            "PreProc" => Some(Self::PreProc),
            "Constant" => Some(Self::Constant),
            "Special" => Some(Self::Special),
            "Error" => Some(Self::Error),
            "Warning" => Some(Self::Warning),
            "Todo" => Some(Self::Todo),
            "StatusLine" | "StatusLineNC" => Some(Self::StatusLine),
            "LineNr" => Some(Self::LineNr),
            "CursorLine" => Some(Self::CursorLine),
            "Visual" => Some(Self::Visual),
            "Search" => Some(Self::Search),
            "MatchParen" => Some(Self::MatchParen),
            "Pmenu" => Some(Self::Pmenu),
            "PmenuSel" => Some(Self::PmenuSel),
            _ => None,
        }
    }
}
