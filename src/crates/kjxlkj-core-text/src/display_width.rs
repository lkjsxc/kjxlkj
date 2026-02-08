//! Display width computation for grapheme clusters.
//!
//! Uses `unicode-width` (UAX #11) to determine how many terminal
//! columns a grapheme cluster occupies.

use unicode_width::UnicodeWidthStr;

/// Compute the display width of a grapheme cluster in terminal columns.
///
/// Returns 1 for most Latin/ASCII, 2 for CJK/fullwidth, 0 for
/// zero-width joiners and combining marks when they form a cluster.
pub fn grapheme_display_width(grapheme: &str) -> u8 {
    let w = UnicodeWidthStr::width(grapheme);
    // Clamp to u8; max 2 for normal graphemes
    if w == 0 && !grapheme.is_empty() {
        // Non-empty cluster with zero width (ZWJ sequences, etc.)
        // Treat as width 1 for rendering purposes.
        1
    } else {
        w.min(2) as u8
    }
}

/// Compute the total display width of a string.
pub fn str_display_width(s: &str) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    s.graphemes(true)
        .map(|g| grapheme_display_width(g) as usize)
        .sum()
}

/// Compute the display column for a given grapheme index within a line.
///
/// Returns the starting display column of the grapheme at position `idx`.
/// If `idx` exceeds the number of graphemes, returns the total width.
pub fn display_col_at_grapheme(line: &str, grapheme_idx: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    let mut col = 0;
    for (i, g) in line.graphemes(true).enumerate() {
        if i == grapheme_idx {
            return col;
        }
        col += grapheme_display_width(g) as usize;
    }
    col
}

/// Find the grapheme index whose display range contains `target_col`.
///
/// For the second column of a wide character, returns the same
/// grapheme index as for the first column.
pub fn grapheme_at_display_col(line: &str, target_col: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    let mut col = 0;
    for (i, g) in line.graphemes(true).enumerate() {
        let w = grapheme_display_width(g) as usize;
        if target_col < col + w {
            return i;
        }
        col += w;
    }
    // Past the end — return the count (for Insert mode end-inclusive)
    line.graphemes(true).count()
}

/// Count the number of grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    s.graphemes(true).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_width() {
        assert_eq!(grapheme_display_width("a"), 1);
        assert_eq!(grapheme_display_width(" "), 1);
    }

    #[test]
    fn cjk_width() {
        assert_eq!(grapheme_display_width("あ"), 2);
        assert_eq!(grapheme_display_width("漢"), 2);
    }

    #[test]
    fn str_width() {
        assert_eq!(str_display_width("hello"), 5);
        assert_eq!(str_display_width("あいう"), 6);
        assert_eq!(str_display_width("aあb"), 4);
    }

    #[test]
    fn column_mapping() {
        let line = "aあbいc";
        assert_eq!(display_col_at_grapheme(line, 0), 0);
        assert_eq!(display_col_at_grapheme(line, 1), 1);
        assert_eq!(display_col_at_grapheme(line, 2), 3);
        assert_eq!(display_col_at_grapheme(line, 3), 4);
        assert_eq!(display_col_at_grapheme(line, 4), 6);
    }

    #[test]
    fn display_col_to_grapheme() {
        let line = "aあbいc";
        assert_eq!(grapheme_at_display_col(line, 0), 0); // a
        assert_eq!(grapheme_at_display_col(line, 1), 1); // あ starts
        assert_eq!(grapheme_at_display_col(line, 2), 1); // あ second col
        assert_eq!(grapheme_at_display_col(line, 3), 2); // b
        assert_eq!(grapheme_at_display_col(line, 4), 3); // い starts
        assert_eq!(grapheme_at_display_col(line, 5), 3); // い second col
        assert_eq!(grapheme_at_display_col(line, 6), 4); // c
    }
}
