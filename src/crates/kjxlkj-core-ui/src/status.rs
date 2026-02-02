//! Status line types.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

/// Status line data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusLine {
    /// Current mode.
    pub mode: Mode,
    /// File name.
    pub file_name: String,
    /// Modified indicator.
    pub modified: bool,
    /// Line number.
    pub line: usize,
    /// Column number.
    pub col: usize,
    /// Total lines.
    pub total_lines: usize,
    /// File type.
    pub file_type: String,
    /// Encoding.
    pub encoding: String,
    /// Line ending type.
    pub line_ending: String,
}

impl StatusLine {
    /// Creates a new status line.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            file_name: String::from("[No Name]"),
            modified: false,
            line: 1,
            col: 1,
            total_lines: 0,
            file_type: String::new(),
            encoding: String::from("utf-8"),
            line_ending: String::from("LF"),
        }
    }

    /// Returns the mode string.
    pub fn mode_str(&self) -> &'static str {
        self.mode.name()
    }

    /// Returns the position string.
    pub fn position_str(&self) -> String {
        format!("{}:{}", self.line, self.col)
    }

    /// Returns the file info string.
    pub fn file_info_str(&self) -> String {
        let modified = if self.modified { "[+]" } else { "" };
        format!("{}{}", self.file_name, modified)
    }

    /// Returns percentage through file.
    pub fn percentage(&self) -> String {
        if self.total_lines == 0 {
            return "Top".to_string();
        }
        let pct = (self.line * 100) / self.total_lines.max(1);
        if self.line <= 1 {
            "Top".to_string()
        } else if self.line >= self.total_lines {
            "Bot".to_string()
        } else {
            format!("{}%", pct)
        }
    }

    /// Formats the status line with a given width.
    pub fn format(&self, width: usize) -> String {
        let left = self.format_left();
        let right = self.format_right();

        let left_len = left.chars().count();
        let right_len = right.chars().count();

        if left_len + right_len >= width {
            // Truncate left side.
            let available = width.saturating_sub(right_len + 3);
            let truncated: String = left.chars().take(available).collect();
            format!("{}...{}", truncated, right)
        } else {
            let padding = width - left_len - right_len;
            format!("{}{:width$}{}", left, "", right, width = padding)
        }
    }

    /// Formats the left side of status line.
    fn format_left(&self) -> String {
        let mode = self.mode_str();
        let file = self.file_info_str();
        format!(" {} | {}", mode, file)
    }

    /// Formats the right side of status line.
    fn format_right(&self) -> String {
        let pos = self.position_str();
        let pct = self.percentage();
        let ft = if self.file_type.is_empty() {
            String::new()
        } else {
            format!("{} | ", self.file_type)
        };
        format!("{}{} | {} ", ft, pos, pct)
    }
}

/// Status line segment for styling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusSegment {
    /// Segment text.
    pub text: String,
    /// Segment style.
    pub style: SegmentStyle,
}

/// Style for a status segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SegmentStyle {
    /// Normal text.
    #[default]
    Normal,
    /// Mode indicator.
    Mode,
    /// File name.
    FileName,
    /// Modified indicator.
    Modified,
    /// Position.
    Position,
    /// File type.
    FileType,
}

impl StatusSegment {
    /// Creates a new segment.
    pub fn new(text: impl Into<String>, style: SegmentStyle) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statusline_new() {
        let sl = StatusLine::new();
        assert_eq!(sl.mode, Mode::Normal);
        assert_eq!(sl.file_name, "[No Name]");
    }

    #[test]
    fn test_mode_str() {
        let mut sl = StatusLine::new();
        sl.mode = Mode::Insert;
        assert_eq!(sl.mode_str(), "INSERT");
    }

    #[test]
    fn test_position_str() {
        let mut sl = StatusLine::new();
        sl.line = 42;
        sl.col = 10;
        assert_eq!(sl.position_str(), "42:10");
    }

    #[test]
    fn test_file_info_modified() {
        let mut sl = StatusLine::new();
        sl.file_name = "test.rs".to_string();
        sl.modified = true;
        assert_eq!(sl.file_info_str(), "test.rs[+]");
    }

    #[test]
    fn test_percentage_top() {
        let mut sl = StatusLine::new();
        sl.line = 1;
        sl.total_lines = 100;
        assert_eq!(sl.percentage(), "Top");
    }

    #[test]
    fn test_percentage_bottom() {
        let mut sl = StatusLine::new();
        sl.line = 100;
        sl.total_lines = 100;
        assert_eq!(sl.percentage(), "Bot");
    }

    #[test]
    fn test_format_basic() {
        let sl = StatusLine::new();
        let formatted = sl.format(80);
        assert!(formatted.contains("NORMAL"));
        assert_eq!(formatted.chars().count(), 80);
    }

    #[test]
    fn test_segment_new() {
        let seg = StatusSegment::new("test", SegmentStyle::Mode);
        assert_eq!(seg.text, "test");
        assert_eq!(seg.style, SegmentStyle::Mode);
    }
}
