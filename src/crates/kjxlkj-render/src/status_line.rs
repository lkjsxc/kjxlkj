//! Status line rendering with configurable segments and layout.

use kjxlkj_core_ui::snapshot::EditorSnapshot;
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

/// Render a full status line from a snapshot.
pub fn render_status_line(snapshot: &EditorSnapshot, width: usize) -> String {
    let ctx = StatusContext {
        mode: format!("{}", snapshot.mode),
        filename: snapshot
            .buffers
            .iter()
            .find(|b| b.id == snapshot.active_buffer)
            .map(|b| b.name.clone())
            .unwrap_or_default(),
        filetype: snapshot
            .buffers
            .iter()
            .find(|b| b.id == snapshot.active_buffer)
            .map(|b| b.filetype.clone())
            .unwrap_or_default(),
        encoding: "utf-8".into(),
        line: snapshot.cursor.position.line + 1,
        col: snapshot.cursor.position.col + 1,
        total_lines: snapshot
            .buffers
            .iter()
            .find(|b| b.id == snapshot.active_buffer)
            .map(|b| b.line_count)
            .unwrap_or(0),
        percent: compute_percent(
            snapshot.cursor.position.line,
            snapshot
                .buffers
                .iter()
                .find(|b| b.id == snapshot.active_buffer)
                .map(|b| b.line_count)
                .unwrap_or(1),
        ),
        modified: snapshot
            .buffers
            .iter()
            .find(|b| b.id == snapshot.active_buffer)
            .map(|b| b.modified)
            .unwrap_or(false),
        buf_nr: snapshot.active_buffer.0,
    };
    let layout = vim_default();
    let mut parts: Vec<String> = Vec::new();
    for sec in &layout.sections {
        let text: String = sec
            .segments
            .iter()
            .map(|s| render_segment(s, &ctx))
            .collect();
        parts.push(text);
    }
    let left = parts.first().cloned().unwrap_or_default();
    let right = parts.last().cloned().unwrap_or_default();
    let gap = width.saturating_sub(left.len() + right.len());
    let mut out = left;
    out.extend(std::iter::repeat_n(' ', gap));
    out.push_str(&right);
    if out.len() > width {
        out.truncate(width);
    }
    out
}

fn compute_percent(line: usize, total: usize) -> u8 {
    if total <= 1 {
        return 100;
    }
    ((line * 100) / total.saturating_sub(1)).min(100) as u8
}

/// Standard Vim-like status line layout.
pub fn vim_default() -> StatusLineLayout {
    StatusLineLayout {
        sections: vec![
            StatusSection {
                segments: vec![
                    StatusSegment::Mode,
                    StatusSegment::Separator,
                    StatusSegment::FileName,
                    StatusSegment::Modified,
                ],
                alignment: Alignment::Left,
                priority: 0,
            },
            StatusSection {
                segments: vec![
                    StatusSegment::Position,
                    StatusSegment::Separator,
                    StatusSegment::Percent,
                ],
                alignment: Alignment::Right,
                priority: 0,
            },
        ],
        width: 80,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_mode_segment() {
        let ctx = StatusContext {
            mode: "NORMAL".into(),
            filename: "f.rs".into(),
            filetype: "rust".into(),
            encoding: "utf-8".into(),
            line: 1,
            col: 1,
            total_lines: 10,
            percent: 0,
            modified: false,
            buf_nr: 1,
        };
        assert_eq!(render_segment(&StatusSegment::Mode, &ctx), " NORMAL ");
    }

    #[test]
    fn render_modified_flag() {
        let mut ctx = StatusContext {
            mode: "N".into(),
            filename: "x".into(),
            filetype: "".into(),
            encoding: "utf-8".into(),
            line: 1,
            col: 1,
            total_lines: 1,
            percent: 100,
            modified: true,
            buf_nr: 1,
        };
        assert_eq!(render_segment(&StatusSegment::Modified, &ctx), "[+]");
        ctx.modified = false;
        assert_eq!(render_segment(&StatusSegment::Modified, &ctx), "");
    }

    #[test]
    fn vim_default_layout() {
        let layout = vim_default();
        assert_eq!(layout.sections.len(), 2);
    }

    #[test]
    fn percent_computation() {
        assert_eq!(compute_percent(0, 100), 0);
        assert_eq!(compute_percent(99, 100), 100);
        assert_eq!(compute_percent(0, 1), 100);
    }
}
