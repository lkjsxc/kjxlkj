//! Highlight group definitions and color mapping.
//!
//! Maps syntax captures to editor highlight groups
//! with associated default colors.

/// Syntax highlight group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighlightGroup {
    Normal,
    Keyword,
    Function,
    Type,
    String,
    Comment,
    Identifier,
    Constant,
    Operator,
    Delimiter,
    Field,
    Number,
    Boolean,
    PreProc,
    Special,
    Error,
    LineNr,
    CursorLine,
    StatusLine,
    StatusLineNC,
    Visual,
    Search,
    IncSearch,
    DiffAdd,
    DiffChange,
    DiffDelete,
}

/// RGB color.
#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// Style for a highlight group (fg, bg, attributes).
#[derive(Debug, Clone, Copy)]
pub struct HighlightStyle {
    pub fg: Rgb,
    pub bg: Rgb,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl HighlightStyle {
    pub const fn new(fg: Rgb, bg: Rgb) -> Self {
        Self {
            fg,
            bg,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// Default dark colorscheme.
pub fn default_style(group: HighlightGroup) -> HighlightStyle {
    let bg = Rgb(30, 30, 30);
    match group {
        HighlightGroup::Normal => HighlightStyle::new(Rgb(212, 212, 212), bg),
        HighlightGroup::Keyword => HighlightStyle {
            fg: Rgb(197, 134, 192),
            bg,
            bold: true,
            italic: false,
            underline: false,
        },
        HighlightGroup::Function => HighlightStyle::new(Rgb(220, 220, 170), bg),
        HighlightGroup::Type => HighlightStyle::new(Rgb(78, 201, 176), bg),
        HighlightGroup::String => HighlightStyle::new(Rgb(206, 145, 120), bg),
        HighlightGroup::Comment => HighlightStyle {
            fg: Rgb(106, 153, 85),
            bg,
            bold: false,
            italic: true,
            underline: false,
        },
        HighlightGroup::Identifier => HighlightStyle::new(Rgb(156, 220, 254), bg),
        HighlightGroup::Constant => HighlightStyle::new(Rgb(100, 150, 224), bg),
        HighlightGroup::Operator => HighlightStyle::new(Rgb(212, 212, 212), bg),
        HighlightGroup::Delimiter => HighlightStyle::new(Rgb(212, 212, 212), bg),
        HighlightGroup::Field => HighlightStyle::new(Rgb(156, 220, 254), bg),
        HighlightGroup::Number => HighlightStyle::new(Rgb(181, 206, 168), bg),
        HighlightGroup::Boolean => HighlightStyle::new(Rgb(86, 156, 214), bg),
        HighlightGroup::PreProc => HighlightStyle::new(Rgb(86, 156, 214), bg),
        HighlightGroup::Special => HighlightStyle::new(Rgb(215, 186, 125), bg),
        HighlightGroup::Error => HighlightStyle::new(Rgb(244, 71, 71), bg),
        HighlightGroup::LineNr => HighlightStyle::new(Rgb(133, 133, 133), bg),
        HighlightGroup::CursorLine => HighlightStyle::new(Rgb(212, 212, 212), Rgb(45, 45, 45)),
        HighlightGroup::StatusLine => HighlightStyle::new(Rgb(255, 255, 255), Rgb(0, 122, 204)),
        HighlightGroup::StatusLineNC => HighlightStyle::new(Rgb(200, 200, 200), Rgb(60, 60, 60)),
        HighlightGroup::Visual => HighlightStyle::new(Rgb(212, 212, 212), Rgb(38, 79, 120)),
        HighlightGroup::Search => HighlightStyle::new(Rgb(0, 0, 0), Rgb(255, 200, 0)),
        HighlightGroup::IncSearch => HighlightStyle::new(Rgb(0, 0, 0), Rgb(255, 150, 0)),
        HighlightGroup::DiffAdd => HighlightStyle::new(Rgb(181, 206, 168), bg),
        HighlightGroup::DiffChange => HighlightStyle::new(Rgb(86, 156, 214), bg),
        HighlightGroup::DiffDelete => HighlightStyle::new(Rgb(244, 71, 71), bg),
    }
}
