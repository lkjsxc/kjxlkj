//! Wrap tests: BD-01, BD-02, BD-10, WR-07 and helpers.

#[cfg(test)]
mod tests {
    use crate::wrap::{grapheme_to_display_pos, wrap_line};

    #[test]
    fn bd01_ascii_wrap() {
        let line = "a".repeat(25);
        let rows = wrap_line(&line, 10);
        assert_eq!(rows.len(), 3); // 10+10+5
        assert_eq!(rows[0].used_cols, 10);
        assert_eq!(rows[1].used_cols, 10);
        assert_eq!(rows[2].used_cols, 5);
    }

    #[test]
    fn bd02_cjk_wrap_no_split() {
        // 5 CJK chars (width 2 each = 10 display cols), wrap at 7 cols
        let line = "日本語表示";
        let rows = wrap_line(line, 7);
        // First row: 3 chars (6 cols) + pad (7th col)
        assert_eq!(rows[0].used_cols, 6);
        assert!(rows[0].has_pad);
        // Second row: 2 chars (4 cols)
        assert_eq!(rows[1].used_cols, 4);
    }

    #[test]
    fn bd10_wrap_boundary_width2_padding() {
        // Width-2 char at position where only 1 col remains
        let line = "abcde漢"; // 5 ascii + 1 CJK (width 2) = 7 total
        let rows = wrap_line(line, 6);
        // First row: "abcde" (5 cols) + pad (1 col remains, can't fit width-2)
        assert_eq!(rows[0].used_cols, 5);
        assert!(rows[0].has_pad);
        // Second row: "漢" (2 cols)
        assert_eq!(rows[1].segments.len(), 1);
        assert_eq!(rows[1].segments[0].grapheme, "漢");
    }

    #[test]
    fn empty_line_produces_one_row() {
        let rows = wrap_line("", 80);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].used_cols, 0);
    }

    #[test]
    fn fits_exactly() {
        let rows = wrap_line("hello", 5);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].used_cols, 5);
    }

    #[test]
    fn grapheme_display_pos_wrap() {
        // "abcd漢字" on 6-col display
        let line = "abcd漢字";
        let (row, col) = grapheme_to_display_pos(line, 4, 6); // '漢'
        assert_eq!(row, 0);
        assert_eq!(col, 4);
        let (row2, col2) = grapheme_to_display_pos(line, 5, 6); // '字'
        assert_eq!(row2, 1);
        assert_eq!(col2, 0);
    }

    #[test]
    fn wr07_no_overflow() {
        // Verify no display row exceeds text_cols
        let line = "a".repeat(1000) + &"漢".repeat(500);
        let rows = wrap_line(&line, 80);
        for row in &rows {
            let total: usize = row.segments.iter().map(|s| s.width).sum::<usize>()
                + if row.has_pad { 1 } else { 0 };
            assert!(total <= 80, "row exceeds text_cols: {total}");
        }
    }
}
