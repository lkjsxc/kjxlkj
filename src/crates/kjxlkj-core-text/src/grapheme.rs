//! Grapheme cluster utilities.
//!
//! Provides grapheme-aware cursor positioning and display-width
//! computation per the cursor specification.

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Count grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Iterate grapheme clusters in a line, stripping trailing newline.
pub fn line_graphemes(line: &str) -> Vec<&str> {
    let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
    trimmed.graphemes(true).collect()
}

/// Compute the display column for a grapheme offset.
pub fn display_col(line: &str, grapheme_offset: usize) -> usize {
    let graphemes = line_graphemes(line);
    graphemes
        .iter()
        .take(grapheme_offset)
        .map(|g| UnicodeWidthStr::width(*g))
        .sum()
}

/// Find the grapheme offset at a given display column.
/// For continuation cells of width-2 graphemes, returns the
/// owning grapheme offset.
pub fn grapheme_at_display_col(line: &str, target_col: usize) -> usize {
    let graphemes = line_graphemes(line);
    let mut col = 0;
    for (i, g) in graphemes.iter().enumerate() {
        let w = UnicodeWidthStr::width(*g);
        if col + w > target_col {
            return i;
        }
        col += w;
        if col > target_col {
            return i;
        }
    }
    graphemes.len().saturating_sub(1)
}

/// Cache for display width computations per line version.
#[derive(Debug, Default)]
pub struct DisplayWidthCache {
    cache: HashMap<(u64, usize), Vec<usize>>,
}

impl DisplayWidthCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or compute width of each grapheme in a line.
    pub fn get_widths(&mut self, version: u64, line_idx: usize, line: &str) -> &[usize] {
        let key = (version, line_idx);
        self.cache.entry(key).or_insert_with(|| {
            line_graphemes(line)
                .iter()
                .map(|g| UnicodeWidthStr::width(*g))
                .collect()
        })
    }

    /// Invalidate cache for a specific version.
    pub fn invalidate(&mut self, older_than: u64) {
        self.cache.retain(|(v, _), _| *v >= older_than);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
    }

    #[test]
    fn test_grapheme_count_cjk() {
        // あいう = 3 grapheme clusters
        assert_eq!(grapheme_count("あいう"), 3);
    }

    #[test]
    fn test_display_col() {
        // ASCII: each char is 1 column wide
        assert_eq!(display_col("hello", 0), 0);
        assert_eq!(display_col("hello", 3), 3);
        // CJK: each char is 2 columns wide
        assert_eq!(display_col("あいう", 0), 0);
        assert_eq!(display_col("あいう", 1), 2);
        assert_eq!(display_col("あいう", 2), 4);
    }

    #[test]
    fn test_grapheme_at_display_col_ascii() {
        assert_eq!(grapheme_at_display_col("hello", 0), 0);
        assert_eq!(grapheme_at_display_col("hello", 3), 3);
    }

    #[test]
    fn test_grapheme_at_display_col_cjk() {
        // あいう: display cols 0-1=あ, 2-3=い, 4-5=う
        assert_eq!(grapheme_at_display_col("あいう", 0), 0);
        assert_eq!(grapheme_at_display_col("あいう", 1), 0);
        assert_eq!(grapheme_at_display_col("あいう", 2), 1);
        assert_eq!(grapheme_at_display_col("あいう", 3), 1);
        assert_eq!(grapheme_at_display_col("あいう", 4), 2);
    }

    #[test]
    fn test_line_graphemes_strips_newline() {
        let gs = line_graphemes("hello\n");
        assert_eq!(gs.len(), 5);
        assert_eq!(gs[0], "h");
    }
}
