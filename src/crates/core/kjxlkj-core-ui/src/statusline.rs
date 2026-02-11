//! Statusline data model.
//!
//! Represents the rendered statusline state derived from snapshot data.
//! See /docs/spec/features/ui/statusline/statusline.md

/// A segment in the statusline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    /// Current mode name: "NORMAL", "INSERT", "VISUAL", etc.
    Mode(String),
    /// Buffer file path (relative to project root).
    File(String),
    /// Modified flag: "[+]" when dirty.
    Modified,
    /// Read-only flag: "[-]" or "[RO]".
    ReadOnly,
    /// File type name (e.g. "rust", "markdown").
    FileType(String),
    /// Cursor position as "line:col".
    Position { line: usize, col: usize },
    /// Position as percentage through the file.
    Percent(u8),
    /// File encoding (e.g. "utf-8").
    Encoding(String),
    /// Line ending format ("LF" or "CRLF").
    FileFormat(String),
    /// Diagnostic counts "E:n W:n".
    Diagnostics { errors: usize, warnings: usize },
    /// Git branch and summary "+n ~n -n".
    Git { branch: String, summary: String },
    /// Literal text.
    Text(String),
}

impl Segment {
    /// Render this segment to a display string.
    pub fn render(&self) -> String {
        match self {
            Self::Mode(m) => m.clone(),
            Self::File(f) => f.clone(),
            Self::Modified => "[+]".into(),
            Self::ReadOnly => "[RO]".into(),
            Self::FileType(ft) => ft.clone(),
            Self::Position { line, col } => format!("{}:{}", line, col),
            Self::Percent(p) => {
                if *p == 0 { "Top".into() }
                else if *p >= 100 { "Bot".into() }
                else { format!("{}%", p) }
            }
            Self::Encoding(e) => e.clone(),
            Self::FileFormat(f) => f.clone(),
            Self::Diagnostics { errors, warnings } => {
                format!("E:{} W:{}", errors, warnings)
            }
            Self::Git { branch, summary } => format!("{} {}", branch, summary),
            Self::Text(t) => t.clone(),
        }
    }
}

/// Statusline layout with left/center/right sections.
#[derive(Debug, Clone)]
pub struct StatuslineData {
    /// Segments aligned to the left.
    pub left: Vec<Segment>,
    /// Segments centered.
    pub center: Vec<Segment>,
    /// Segments aligned to the right.
    pub right: Vec<Segment>,
    /// Separator between segments.
    pub separator: String,
    /// Whether this is active (focused) window.
    pub active: bool,
}

impl StatuslineData {
    /// Create a default statusline for the active window.
    pub fn new_active() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            separator: " ".into(),
            active: true,
        }
    }

    /// Create a default statusline for an inactive window.
    pub fn new_inactive() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            separator: " ".into(),
            active: false,
        }
    }

    /// Render the left section as a single string.
    pub fn render_left(&self) -> String {
        self.left.iter().map(|s| s.render()).collect::<Vec<_>>().join(&self.separator)
    }

    /// Render the center section as a single string.
    pub fn render_center(&self) -> String {
        self.center.iter().map(|s| s.render()).collect::<Vec<_>>().join(&self.separator)
    }

    /// Render the right section as a single string.
    pub fn render_right(&self) -> String {
        self.right.iter().map(|s| s.render()).collect::<Vec<_>>().join(&self.separator)
    }

    /// Build a default active statusline from common state info.
    pub fn from_state(mode: &str, file: &str, modified: bool, filetype: &str,
                      line: usize, col: usize, percent: u8) -> Self {
        let mut data = Self::new_active();
        data.left.push(Segment::Mode(mode.to_uppercase()));
        data.left.push(Segment::File(file.into()));
        if modified { data.left.push(Segment::Modified); }
        if !filetype.is_empty() { data.right.push(Segment::FileType(filetype.into())); }
        data.right.push(Segment::Position { line, col });
        data.right.push(Segment::Percent(percent));
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_mode_renders() {
        let s = Segment::Mode("NORMAL".into());
        assert_eq!(s.render(), "NORMAL");
    }

    #[test]
    fn segment_position_renders() {
        let s = Segment::Position { line: 42, col: 15 };
        assert_eq!(s.render(), "42:15");
    }

    #[test]
    fn segment_percent_top_bot() {
        assert_eq!(Segment::Percent(0).render(), "Top");
        assert_eq!(Segment::Percent(100).render(), "Bot");
        assert_eq!(Segment::Percent(50).render(), "50%");
    }

    #[test]
    fn segment_diagnostics_renders() {
        let s = Segment::Diagnostics { errors: 2, warnings: 5 };
        assert_eq!(s.render(), "E:2 W:5");
    }

    #[test]
    fn segment_git_renders() {
        let s = Segment::Git { branch: "main".into(), summary: "+2 ~1 -0".into() };
        assert_eq!(s.render(), "main +2 ~1 -0");
    }

    #[test]
    fn from_state_builds_default() {
        let data = StatuslineData::from_state("normal", "src/main.rs", true, "rust", 42, 15, 50);
        assert_eq!(data.render_left(), "NORMAL src/main.rs [+]");
        assert_eq!(data.render_right(), "rust 42:15 50%");
        assert_eq!(data.render_center(), "");
        assert!(data.active);
    }

    #[test]
    fn inactive_statusline() {
        let data = StatuslineData::new_inactive();
        assert!(!data.active);
        assert_eq!(data.render_left(), "");
    }
}
