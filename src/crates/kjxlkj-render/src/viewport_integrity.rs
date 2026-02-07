//! Viewport validation: display cells, line wrapping, and integrity checks.

use unicode_width::UnicodeWidthChar;

/// A single cell in the display grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayCell {
    /// Normal single-width character.
    Normal(char),
    /// First cell of a wide character.
    Wide(char),
    /// Second cell occupied by the preceding wide character.
    Continuation,
}

/// A row of display cells.
#[derive(Debug, Clone)]
pub struct DisplayRow {
    pub cells: Vec<DisplayCell>,
}

impl DisplayRow {
    /// Compute the display width of this row.
    pub fn width(&self) -> usize {
        self.cells.len()
    }
}

/// Wrap a line into display rows respecting character widths.
pub fn wrap_line(line: &str, max_width: usize) -> Vec<DisplayRow> {
    if max_width == 0 { return vec![DisplayRow { cells: vec![] }]; }
    let mut rows: Vec<DisplayRow> = Vec::new();
    let mut current: Vec<DisplayCell> = Vec::new();
    let mut col = 0usize;

    for ch in line.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(1);
        if col + w > max_width {
            rows.push(DisplayRow { cells: std::mem::take(&mut current) });
            col = 0;
        }
        if w == 2 {
            current.push(DisplayCell::Wide(ch));
            current.push(DisplayCell::Continuation);
        } else {
            current.push(DisplayCell::Normal(ch));
        }
        col += w;
    }
    rows.push(DisplayRow { cells: current });
    rows
}

/// Check whether a line exceeds the given threshold in characters.
pub fn is_long_line(line: &str, threshold: usize) -> bool {
    line.len() > threshold
}

/// Truncate a line to fit within `max_width` display columns.
pub fn truncate_line(line: &str, max_width: usize) -> String {
    let mut out = String::new();
    let mut col = 0usize;
    for ch in line.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(1);
        if col + w > max_width { break; }
        out.push(ch);
        col += w;
    }
    out
}

/// Validate that all rows fit the expected width, returning error messages.
pub fn validate_viewport(rows: &[DisplayRow], expected_width: usize) -> Vec<String> {
    let mut errors = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let w = row.width();
        if w > expected_width {
            errors.push(format!("row {i}: width {w} exceeds expected {expected_width}"));
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_ascii() {
        let rows = wrap_line("abcdef", 3);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].width(), 3);
        assert_eq!(rows[1].width(), 3);
    }

    #[test]
    fn wrap_empty() {
        let rows = wrap_line("", 10);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].width(), 0);
    }

    #[test]
    fn wrap_wide_chars() {
        // '全' is a wide character (width 2)
        let rows = wrap_line("全全全", 4);
        // Each '全' = 2 columns; 3 chars = 6 cols; at width 4, wraps after 2 chars
        assert!(rows.len() >= 2);
    }

    #[test]
    fn is_long_line_check() {
        assert!(!is_long_line("short", 1000));
        assert!(is_long_line(&"x".repeat(1001), 1000));
    }

    #[test]
    fn truncate_basic() {
        assert_eq!(truncate_line("hello world", 5), "hello");
    }

    #[test]
    fn truncate_wide() {
        let s = "全全全"; // each is 2 cols wide
        assert_eq!(truncate_line(s, 4), "全全");
    }

    #[test]
    fn validate_ok() {
        let rows = wrap_line("abcdef", 3);
        assert!(validate_viewport(&rows, 3).is_empty());
    }

    #[test]
    fn validate_fail() {
        let row = DisplayRow { cells: vec![DisplayCell::Normal('a'); 10] };
        let errs = validate_viewport(&[row], 5);
        assert_eq!(errs.len(), 1);
    }
}
