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
}
