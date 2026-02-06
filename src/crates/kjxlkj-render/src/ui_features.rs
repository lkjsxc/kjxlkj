/// Full UI feature documents — statusline, winbar, message area integration.

/// Statusline segment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusSegment { Mode, FileName, FileType, Encoding, LineEnding, Position, Percent, Branch, Diagnostics, Modified, ReadOnly, Space }

/// A built statusline from segments.
#[derive(Debug, Clone)]
pub struct StatusLine { pub left: Vec<StatusSegment>, pub right: Vec<StatusSegment> }

impl StatusLine {
    pub fn default_layout() -> Self {
        Self {
            left: vec![StatusSegment::Mode, StatusSegment::Space, StatusSegment::FileName, StatusSegment::Modified, StatusSegment::ReadOnly],
            right: vec![StatusSegment::Branch, StatusSegment::Space, StatusSegment::FileType, StatusSegment::Space, StatusSegment::Encoding, StatusSegment::Space, StatusSegment::Position, StatusSegment::Percent],
        }
    }
    pub fn total_segments(&self) -> usize { self.left.len() + self.right.len() }
}

/// Render a status segment to text.
pub fn render_segment(seg: StatusSegment, ctx: &StatusContext) -> String {
    match seg {
        StatusSegment::Mode => format!(" {} ", ctx.mode),
        StatusSegment::FileName => ctx.filename.clone(),
        StatusSegment::FileType => format!("[{}]", ctx.filetype),
        StatusSegment::Encoding => ctx.encoding.clone(),
        StatusSegment::Position => format!("{}:{}", ctx.line + 1, ctx.col + 1),
        StatusSegment::Percent => { if ctx.total_lines == 0 { "Top".into() }
            else { format!("{}%", (ctx.line * 100) / ctx.total_lines.max(1)) }
        }
        StatusSegment::Modified => if ctx.modified { "[+]" } else { "" }.into(),
        StatusSegment::ReadOnly => if ctx.readonly { "[RO]" } else { "" }.into(),
        StatusSegment::Branch => ctx.branch.clone().unwrap_or_default(),
        StatusSegment::Diagnostics => format!("E:{} W:{}", ctx.errors, ctx.warnings),
        StatusSegment::LineEnding => ctx.line_ending.clone(),
        StatusSegment::Space => " ".into(),
    }
}

/// Context for rendering status segments.
#[derive(Debug, Clone)]
pub struct StatusContext {
    pub mode: String, pub filename: String, pub filetype: String,
    pub encoding: String, pub line_ending: String,
    pub line: usize, pub col: usize, pub total_lines: usize,
    pub modified: bool, pub readonly: bool,
    pub branch: Option<String>, pub errors: usize, pub warnings: usize,
}

impl Default for StatusContext {
    fn default() -> Self {
        Self { mode: "NORMAL".into(), filename: "[No Name]".into(), filetype: "".into(),
            encoding: "utf-8".into(), line_ending: "LF".into(),
            line: 0, col: 0, total_lines: 0, modified: false, readonly: false,
            branch: None, errors: 0, warnings: 0 }
    }
}

/// Message area display.
#[derive(Debug, Clone)]
pub struct MessageArea { pub text: String, pub is_error: bool, pub persistent: bool }

impl MessageArea {
    pub fn info(text: impl Into<String>) -> Self { Self { text: text.into(), is_error: false, persistent: false } }
    pub fn error(text: impl Into<String>) -> Self { Self { text: text.into(), is_error: true, persistent: false } }
    pub fn clear() -> Self { Self { text: String::new(), is_error: false, persistent: false } }
    pub fn is_empty(&self) -> bool { self.text.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_statusline() { let s = StatusLine::default_layout(); assert!(s.total_segments() > 8); }

    #[test]
    fn render_mode() {
        let ctx = StatusContext::default();
        assert_eq!(render_segment(StatusSegment::Mode, &ctx), " NORMAL ");
    }

    #[test]
    fn render_position() {
        let mut ctx = StatusContext::default(); ctx.line = 9; ctx.col = 4;
        assert_eq!(render_segment(StatusSegment::Position, &ctx), "10:5");
    }

    #[test]
    fn render_modified() {
        let mut ctx = StatusContext::default(); ctx.modified = true;
        assert_eq!(render_segment(StatusSegment::Modified, &ctx), "[+]");
    }

    #[test]
    fn render_percent() {
        let mut ctx = StatusContext::default(); ctx.line = 50; ctx.total_lines = 100;
        assert_eq!(render_segment(StatusSegment::Percent, &ctx), "50%");
    }

    #[test]
    fn message_area_info() { let m = MessageArea::info("saved"); assert!(!m.is_error); }
    #[test]
    fn message_area_error() { let m = MessageArea::error("E45: readonly"); assert!(m.is_error); }
    #[test]
    fn message_area_empty() { assert!(MessageArea::clear().is_empty()); }
}
