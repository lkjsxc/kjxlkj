/// Long line handling — safe slicing, virtualization, width caching.

/// Maximum columns to render before truncation.
pub const MAX_RENDER_COLS: usize = 10_000;

/// A segment of a long line for virtualized rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct LineSegment {
    pub start_col: usize,
    pub text: String,
    pub display_width: usize,
}

/// Split a long line into segments of at most `max_width` display columns.
pub fn segment_line(line: &str, max_width: usize) -> Vec<LineSegment> {
    if max_width == 0 { return vec![]; }
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut current_width = 0usize;
    let mut start_col = 0usize;
    let mut col = 0usize;
    for ch in line.chars() {
        let w = char_width(ch);
        if current_width + w > max_width && !current.is_empty() {
            segments.push(LineSegment { start_col, text: std::mem::take(&mut current), display_width: current_width });
            start_col = col;
            current_width = 0;
        }
        current.push(ch);
        current_width += w;
        col += 1;
    }
    if !current.is_empty() || segments.is_empty() {
        segments.push(LineSegment { start_col, text: current, display_width: current_width });
    }
    segments
}

/// Compute display width of a character (tab=8, wide=2, others=1).
fn char_width(ch: char) -> usize {
    match ch {
        '\t' => 8,
        c if is_wide(c) => 2,
        _ => 1,
    }
}

/// Check if a character is East Asian wide (simplified).
fn is_wide(ch: char) -> bool {
    let c = ch as u32;
    (0x1100..=0x115F).contains(&c) || (0x2E80..=0x9FFF).contains(&c)
        || (0xAC00..=0xD7AF).contains(&c) || (0xF900..=0xFAFF).contains(&c)
        || (0xFE10..=0xFE6F).contains(&c) || (0xFF01..=0xFF60).contains(&c)
        || (0x20000..=0x2FA1F).contains(&c)
}

/// Compute display width of a string.
pub fn display_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

/// Safely slice a string by display column range. Returns the substring
/// that falls within [start_col, start_col + width).
pub fn safe_slice(line: &str, start_col: usize, width: usize) -> String {
    let mut result = String::new();
    let mut col = 0usize;
    for ch in line.chars() {
        let w = char_width(ch);
        if col + w > start_col + width { break; }
        if col >= start_col { result.push(ch); }
        else if col + w > start_col { result.push(' '); } // Partial wide char → space
        col += w;
    }
    result
}

/// Check if a line exceeds the render threshold.
pub fn is_long_line(line: &str) -> bool {
    display_width(line) > MAX_RENDER_COLS
}

/// Truncate indicator for lines exceeding max render columns.
pub fn truncation_indicator() -> &'static str { ">>>" }

/// Compute display column for a given char index in a line.
pub fn char_to_col(line: &str, char_idx: usize) -> usize {
    line.chars().take(char_idx).map(char_width).sum()
}

/// Compute char index for a given display column.
pub fn col_to_char(line: &str, target_col: usize) -> usize {
    let mut col = 0;
    for (i, ch) in line.chars().enumerate() {
        if col >= target_col { return i; }
        col += char_width(ch);
    }
    line.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_short_line() {
        let segs = segment_line("hello", 80);
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].text, "hello");
    }

    #[test]
    fn segment_long_line() {
        let line = "a".repeat(200);
        let segs = segment_line(&line, 80);
        assert!(segs.len() >= 3);
        assert_eq!(segs[0].display_width, 80);
    }

    #[test]
    fn display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn display_width_tab() {
        assert_eq!(display_width("\t"), 8);
    }

    #[test]
    fn safe_slice_basic() {
        assert_eq!(safe_slice("hello world", 6, 5), "world");
    }

    #[test]
    fn safe_slice_beyond() {
        assert_eq!(safe_slice("hi", 0, 10), "hi");
    }

    #[test]
    fn is_long_line_false() {
        assert!(!is_long_line("short"));
    }

    #[test]
    fn char_to_col_and_back() {
        let line = "abcdef";
        assert_eq!(char_to_col(line, 3), 3);
        assert_eq!(col_to_char(line, 3), 3);
    }

    #[test]
    fn col_to_char_past_end() {
        assert_eq!(col_to_char("ab", 10), 2);
    }
}
