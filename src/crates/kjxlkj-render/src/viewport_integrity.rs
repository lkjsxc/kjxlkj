/// Viewport integrity — long line handling, wrap consistency, display safety.

/// Display cell representing one rendered character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayCell { pub ch: char, pub width: u8, pub is_continuation: bool }

impl DisplayCell {
    pub fn normal(ch: char) -> Self { Self { ch, width: 1, is_continuation: false } }
    pub fn wide(ch: char) -> Self { Self { ch, width: 2, is_continuation: false } }
    pub fn continuation() -> Self { Self { ch: ' ', width: 0, is_continuation: true } }
}

/// A display row — one screen line of cells.
#[derive(Debug, Clone)]
pub struct DisplayRow { pub cells: Vec<DisplayCell>, pub buffer_line: usize, pub is_wrapped: bool }

impl DisplayRow {
    pub fn new(buffer_line: usize) -> Self { Self { cells: Vec::new(), buffer_line, is_wrapped: false } }
    pub fn cell_count(&self) -> usize { self.cells.len() }
}

/// Render a buffer line into display rows with wrapping.
pub fn wrap_line(line: &str, screen_width: u16) -> Vec<DisplayRow> {
    if screen_width == 0 { return vec![]; }
    let w = screen_width as usize;
    let mut rows = Vec::new();
    let mut current = DisplayRow::new(0);
    let mut col = 0usize;
    for ch in line.chars() {
        let cw = if ch == '\t' { 4usize.min(w.saturating_sub(col)) }
            else if ch.is_control() { 1 }
            else { unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) };
        if col + cw > w && !current.cells.is_empty() {
            rows.push(current); current = DisplayRow::new(0);
            current.is_wrapped = true; col = 0;
        }
        if ch == '\t' {
            for _ in 0..cw { current.cells.push(DisplayCell::normal(' ')); }
        } else if cw == 2 { current.cells.push(DisplayCell::wide(ch)); current.cells.push(DisplayCell::continuation()); }
        else { current.cells.push(DisplayCell::normal(ch)); }
        col += cw;
    }
    rows.push(current);
    rows
}

/// Check if a line is "long" (exceeds threshold).
pub fn is_long_line(line: &str, threshold: usize) -> bool {
    line.len() > threshold
}

/// Truncate a line for display without wrapping.
pub fn truncate_line(line: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut w = 0;
    for ch in line.chars() {
        let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
        if w + cw > max_width { result.push('>'); break; }
        result.push(ch);
        w += cw;
    }
    result
}

/// Validate viewport integrity: all rows have valid width.
pub fn validate_viewport(rows: &[DisplayRow], screen_width: u16) -> bool {
    rows.iter().all(|r| r.cell_count() <= screen_width as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_short_line() { let rows = wrap_line("hello", 80); assert_eq!(rows.len(), 1); }

    #[test]
    fn wrap_long_line() {
        let line = "a".repeat(160);
        let rows = wrap_line(&line, 80);
        assert_eq!(rows.len(), 2);
        assert!(rows[1].is_wrapped);
    }

    #[test]
    fn truncate_short() { assert_eq!(truncate_line("hi", 80), "hi"); }

    #[test]
    fn truncate_long() {
        let long = "a".repeat(100);
        let t = truncate_line(&long, 80);
        assert_eq!(t.len(), 81); // 80 chars + '>'
    }

    #[test]
    fn is_long() { assert!(is_long_line(&"x".repeat(1001), 1000)); assert!(!is_long_line("short", 1000)); }

    #[test]
    fn display_cell_wide() { let c = DisplayCell::wide('中'); assert_eq!(c.width, 2); }

    #[test]
    fn validate_ok() {
        let rows = wrap_line("hello world", 80);
        assert!(validate_viewport(&rows, 80));
    }

    #[test]
    fn zero_width() { let rows = wrap_line("abc", 0); assert!(rows.is_empty()); }
}
