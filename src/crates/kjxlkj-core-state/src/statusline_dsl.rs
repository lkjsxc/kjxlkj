//! Statusline DSL per /docs/spec/features/ui/statusline/.
//!
//! A configurable statusline with format strings.

/// Statusline segment kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusSegment {
    /// Mode indicator.
    Mode,
    /// File name.
    FileName,
    /// File path.
    FilePath,
    /// Modified flag ([+]).
    Modified,
    /// Read-only flag.
    ReadOnly,
    /// File type.
    FileType,
    /// Line number.
    Line,
    /// Column number.
    Column,
    /// Percentage through file.
    Percentage,
    /// Encoding (utf-8, etc).
    Encoding,
    /// Line ending (unix/dos/mac).
    LineEnding,
    /// Git branch.
    GitBranch,
    /// Git diff stats (+N ~N -N).
    GitDiff,
    /// LSP diagnostics count.
    Diagnostics,
    /// Separator (pushes right).
    Separator,
    /// Literal text.
    Text(String),
}

/// Statusline configuration.
#[derive(Debug, Clone)]
pub struct StatuslineConfig {
    /// Left segments.
    pub left: Vec<StatusSegment>,
    /// Right segments (after separator).
    pub right: Vec<StatusSegment>,
    /// Whether global statusline (one for all).
    pub global: bool,
}

impl Default for StatuslineConfig {
    fn default() -> Self {
        Self {
            left: vec![
                StatusSegment::Mode,
                StatusSegment::FileName,
                StatusSegment::Modified,
                StatusSegment::ReadOnly,
            ],
            right: vec![
                StatusSegment::FileType,
                StatusSegment::Encoding,
                StatusSegment::Line,
                StatusSegment::Column,
                StatusSegment::Percentage,
            ],
            global: false,
        }
    }
}

impl StatuslineConfig {
    /// Create default config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse format string into segments.
    pub fn parse_format(
        fmt: &str,
    ) -> Vec<StatusSegment> {
        let mut segs = Vec::new();
        for part in fmt.split('%') {
            if part.is_empty() {
                continue;
            }
            match part.chars().next() {
                Some('m') => {
                    segs.push(StatusSegment::Mode);
                }
                Some('f') => {
                    segs.push(StatusSegment::FileName);
                }
                Some('F') => {
                    segs.push(StatusSegment::FilePath);
                }
                Some('M') => {
                    segs.push(StatusSegment::Modified);
                }
                Some('l') => {
                    segs.push(StatusSegment::Line);
                }
                Some('c') => {
                    segs.push(StatusSegment::Column);
                }
                Some('p') => {
                    segs.push(StatusSegment::Percentage);
                }
                Some('=') => {
                    segs.push(StatusSegment::Separator);
                }
                _ => {
                    segs.push(StatusSegment::Text(
                        part.to_string(),
                    ));
                }
            }
        }
        segs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let cfg = StatuslineConfig::new();
        assert!(!cfg.global);
        assert!(!cfg.left.is_empty());
        assert!(!cfg.right.is_empty());
    }

    #[test]
    fn parse_format_string() {
        let segs = StatuslineConfig::parse_format(
            "%m %f %=%l:%c",
        );
        assert!(segs.contains(&StatusSegment::Mode));
        assert!(segs.contains(&StatusSegment::FileName));
        assert!(segs.contains(&StatusSegment::Separator));
    }
}
