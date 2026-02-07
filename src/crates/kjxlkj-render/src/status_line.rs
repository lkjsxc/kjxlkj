//! Status line rendering with configurable segments and layout.

use serde::{Deserialize, Serialize};

/// Individual segment of the status line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusSegment {
    Mode,
    FileName,
    FileType,
    Encoding,
    Position,
    Percent,
    Modified,
    ReadOnly,
    LineCount,
    BufNr,
    Custom(String),
    Separator,
}

/// Horizontal alignment.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// A group of segments with alignment and priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSection {
    pub segments: Vec<StatusSegment>,
    pub alignment: Alignment,
    pub priority: u8,
}

/// Full status line layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLineLayout {
    pub sections: Vec<StatusSection>,
    pub width: usize,
}

/// Context data fed to segment rendering.
#[derive(Debug, Clone)]
pub struct StatusContext {
    pub mode: String,
    pub filename: String,
    pub filetype: String,
    pub encoding: String,
    pub line: usize,
    pub col: usize,
    pub total_lines: usize,
    pub percent: u8,
    pub modified: bool,
    pub buf_nr: u64,
}

/// Render a single segment to string.
pub fn render_segment(seg: &StatusSegment, ctx: &StatusContext) -> String {
    match seg {
        StatusSegment::Mode => format!(" {} ", ctx.mode),
        StatusSegment::FileName => ctx.filename.clone(),
        StatusSegment::FileType => ctx.filetype.clone(),
        StatusSegment::Encoding => ctx.encoding.clone(),
        StatusSegment::Position => format!("{}:{}", ctx.line, ctx.col),
        StatusSegment::Percent => format!("{}%", ctx.percent),
        StatusSegment::Modified => {
            if ctx.modified {
                "[+]".into()
            } else {
                String::new()
            }
        }
        StatusSegment::ReadOnly => "[-]".into(),
        StatusSegment::LineCount => format!("{}", ctx.total_lines),
        StatusSegment::BufNr => format!("#{}", ctx.buf_nr),
        StatusSegment::Custom(s) => s.clone(),
        StatusSegment::Separator => " | ".into(),
    }
}

// Re-export rendering functions from dedicated module.
pub use crate::status_line_render::{render_status_line, vim_default};
