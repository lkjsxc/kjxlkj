//! Statusline types and configuration.
//!
//! Implements statusline as specified in `/docs/spec/features/ui/statusline/`.

/// Statusline section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusSection {
    /// Mode indicator.
    Mode,
    /// File name.
    FileName,
    /// File modified indicator.
    Modified,
    /// File readonly indicator.
    ReadOnly,
    /// File type.
    FileType,
    /// Cursor position (line:col).
    Position,
    /// Line percentage.
    Percentage,
    /// Git branch.
    GitBranch,
    /// Git status.
    GitStatus,
    /// Diagnostics count.
    Diagnostics,
    /// LSP status.
    LspStatus,
    /// Encoding.
    Encoding,
    /// Line ending (LF/CRLF).
    LineEnding,
    /// Selection count.
    Selection,
    /// Macro recording.
    Recording,
    /// Custom text.
    Text(String),
    /// Separator.
    Separator,
    /// Spacer (flexible space).
    Spacer,
}

/// Statusline segment with styling.
#[derive(Debug, Clone)]
pub struct StatusSegment {
    /// Section type.
    pub section: StatusSection,
    /// Foreground color (optional).
    pub fg: Option<String>,
    /// Background color (optional).
    pub bg: Option<String>,
    /// Bold text.
    pub bold: bool,
    /// Minimum width.
    pub min_width: Option<usize>,
}

impl StatusSegment {
    /// Create a new segment.
    pub fn new(section: StatusSection) -> Self {
        Self {
            section,
            fg: None,
            bg: None,
            bold: false,
            min_width: None,
        }
    }

    /// Set foreground color.
    pub fn with_fg(mut self, color: &str) -> Self {
        self.fg = Some(color.to_string());
        self
    }

    /// Set background color.
    pub fn with_bg(mut self, color: &str) -> Self {
        self.bg = Some(color.to_string());
        self
    }

    /// Make bold.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set minimum width.
    pub fn with_min_width(mut self, width: usize) -> Self {
        self.min_width = Some(width);
        self
    }
}

/// Statusline configuration.
#[derive(Debug, Clone)]
pub struct StatuslineConfig {
    /// Left sections.
    pub left: Vec<StatusSegment>,
    /// Right sections.
    pub right: Vec<StatusSegment>,
    /// Show statusline (global).
    pub enabled: bool,
    /// Global style.
    pub global_style: bool,
    /// Show mode in statusline (vs separate area).
    pub show_mode: bool,
}

impl Default for StatuslineConfig {
    fn default() -> Self {
        Self {
            left: vec![
                StatusSegment::new(StatusSection::Mode).bold(),
                StatusSegment::new(StatusSection::Separator),
                StatusSegment::new(StatusSection::FileName),
                StatusSegment::new(StatusSection::Modified),
                StatusSegment::new(StatusSection::ReadOnly),
            ],
            right: vec![
                StatusSegment::new(StatusSection::GitBranch),
                StatusSegment::new(StatusSection::Diagnostics),
                StatusSegment::new(StatusSection::Separator),
                StatusSegment::new(StatusSection::FileType),
                StatusSegment::new(StatusSection::Separator),
                StatusSegment::new(StatusSection::Position),
                StatusSegment::new(StatusSection::Percentage),
            ],
            enabled: true,
            global_style: true,
            show_mode: true,
        }
    }
}

impl StatuslineConfig {
    /// Create a minimal config.
    pub fn minimal() -> Self {
        Self {
            left: vec![
                StatusSegment::new(StatusSection::FileName),
                StatusSegment::new(StatusSection::Modified),
            ],
            right: vec![
                StatusSegment::new(StatusSection::Position),
            ],
            enabled: true,
            global_style: true,
            show_mode: true,
        }
    }

    /// Disable statusline.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Check if statusline should be shown.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Rendered statusline data.
#[derive(Debug, Clone, Default)]
pub struct RenderedStatusline {
    /// Left portion text.
    pub left: String,
    /// Right portion text.
    pub right: String,
    /// Total width consumed.
    pub width: usize,
}

impl RenderedStatusline {
    /// Create a new rendered statusline.
    pub fn new(left: String, right: String) -> Self {
        let width = left.len() + right.len();
        Self { left, right, width }
    }

    /// Format for given width (with padding).
    pub fn format(&self, total_width: usize) -> String {
        let padding = total_width.saturating_sub(self.left.len() + self.right.len());
        format!("{}{:padding$}{}", self.left, "", self.right, padding = padding)
    }
}

/// Tabline (buffer tabs) configuration.
#[derive(Debug, Clone)]
pub struct TablineConfig {
    /// Show tabline.
    pub enabled: bool,
    /// Show when only one buffer.
    pub show_single: bool,
    /// Show buffer numbers.
    pub show_numbers: bool,
    /// Show close button.
    pub show_close: bool,
    /// Show modified indicator.
    pub show_modified: bool,
    /// Maximum tabs to show.
    pub max_tabs: usize,
}

impl Default for TablineConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_single: false,
            show_numbers: true,
            show_close: false,
            show_modified: true,
            max_tabs: 10,
        }
    }
}

impl TablineConfig {
    /// Create a minimal config.
    pub fn minimal() -> Self {
        Self {
            enabled: true,
            show_single: false,
            show_numbers: false,
            show_close: false,
            show_modified: true,
            max_tabs: 5,
        }
    }

    /// Disable tabline.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
}

/// A tab entry for rendering.
#[derive(Debug, Clone)]
pub struct TabEntry {
    /// Buffer ID.
    pub id: u32,
    /// Display name.
    pub name: String,
    /// Is modified.
    pub modified: bool,
    /// Is active.
    pub active: bool,
}

impl TabEntry {
    /// Create a new tab entry.
    pub fn new(id: u32, name: impl Into<String>, modified: bool, active: bool) -> Self {
        Self {
            id,
            name: name.into(),
            modified,
            active,
        }
    }

    /// Get display text.
    pub fn display(&self, config: &TablineConfig) -> String {
        let mut s = String::new();
        if config.show_numbers {
            s.push_str(&format!("{}: ", self.id));
        }
        s.push_str(&self.name);
        if config.show_modified && self.modified {
            s.push_str(" [+]");
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_segment_new() {
        let seg = StatusSegment::new(StatusSection::Mode);
        assert!(!seg.bold);
        assert!(seg.fg.is_none());
    }

    #[test]
    fn test_status_segment_with_fg() {
        let seg = StatusSegment::new(StatusSection::Mode).with_fg("red");
        assert_eq!(seg.fg, Some("red".to_string()));
    }

    #[test]
    fn test_status_segment_bold() {
        let seg = StatusSegment::new(StatusSection::Mode).bold();
        assert!(seg.bold);
    }

    #[test]
    fn test_statusline_config_default() {
        let config = StatuslineConfig::default();
        assert!(config.enabled);
        assert!(!config.left.is_empty());
        assert!(!config.right.is_empty());
    }

    #[test]
    fn test_statusline_config_minimal() {
        let config = StatuslineConfig::minimal();
        assert!(config.enabled);
        assert!(config.left.len() < StatuslineConfig::default().left.len());
    }

    #[test]
    fn test_statusline_config_disabled() {
        let config = StatuslineConfig::disabled();
        assert!(!config.is_enabled());
    }

    #[test]
    fn test_rendered_statusline_new() {
        let sl = RenderedStatusline::new("left".to_string(), "right".to_string());
        assert_eq!(sl.width, 9);
    }

    #[test]
    fn test_rendered_statusline_format() {
        let sl = RenderedStatusline::new("L".to_string(), "R".to_string());
        let formatted = sl.format(10);
        assert_eq!(formatted.len(), 10);
        assert!(formatted.starts_with("L"));
        assert!(formatted.ends_with("R"));
    }

    #[test]
    fn test_tabline_config_default() {
        let config = TablineConfig::default();
        assert!(config.enabled);
        assert!(config.show_numbers);
    }

    #[test]
    fn test_tabline_config_minimal() {
        let config = TablineConfig::minimal();
        assert!(!config.show_numbers);
    }

    #[test]
    fn test_tabline_config_disabled() {
        let config = TablineConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_tab_entry_new() {
        let tab = TabEntry::new(1, "file.rs", false, true);
        assert_eq!(tab.id, 1);
        assert!(tab.active);
    }

    #[test]
    fn test_tab_entry_display() {
        let tab = TabEntry::new(1, "file.rs", true, false);
        let config = TablineConfig::default();
        let display = tab.display(&config);
        assert!(display.contains("file.rs"));
        assert!(display.contains("[+]"));
    }

    #[test]
    fn test_tab_entry_display_no_numbers() {
        let tab = TabEntry::new(1, "file.rs", false, false);
        let config = TablineConfig::minimal();
        let display = tab.display(&config);
        assert!(!display.contains("1:"));
    }
}
