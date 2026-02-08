//! Line ending detection and conversion.

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Unix-style LF (`\n`).
    Lf,
    /// Windows-style CRLF (`\r\n`).
    CrLf,
}

impl LineEnding {
    /// The string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
        }
    }
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::Lf
    }
}

/// Detect the predominant line ending in a text.
///
/// Counts occurrences of `\r\n` vs standalone `\n`. Returns CRLF
/// if more than half of line endings are CRLF, else LF.
pub fn detect_line_ending(text: &str) -> LineEnding {
    let crlf_count = text.matches("\r\n").count();
    let lf_count = text.matches('\n').count();
    let standalone_lf = lf_count.saturating_sub(crlf_count);

    if crlf_count > standalone_lf {
        LineEnding::CrLf
    } else {
        LineEnding::Lf
    }
}

/// Normalize all line endings to the target style.
pub fn normalize_line_endings(text: &str, target: LineEnding) -> String {
    // First normalize to LF
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    match target {
        LineEnding::Lf => normalized,
        LineEnding::CrLf => normalized.replace('\n', "\r\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_lf() {
        let text = "hello\nworld\n";
        assert_eq!(detect_line_ending(text), LineEnding::Lf);
    }

    #[test]
    fn detect_crlf() {
        let text = "hello\r\nworld\r\n";
        assert_eq!(detect_line_ending(text), LineEnding::CrLf);
    }

    #[test]
    fn normalize_to_lf() {
        let text = "a\r\nb\r\nc";
        let norm = normalize_line_endings(text, LineEnding::Lf);
        assert_eq!(norm, "a\nb\nc");
    }

    #[test]
    fn normalize_to_crlf() {
        let text = "a\nb\nc";
        let norm = normalize_line_endings(text, LineEnding::CrLf);
        assert_eq!(norm, "a\r\nb\r\nc");
    }
}
