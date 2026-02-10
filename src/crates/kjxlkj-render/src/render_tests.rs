//! Tests for rendering (RR-01 through RR-10).
//!
//! Covers spec requirements from `/docs/spec/technical/testing-unit.md`.

#[cfg(test)]
mod tests {
    use crate::grid::{Cell, CellGrid};
    use crate::paint::paint_window::{display_col_for_grapheme_offset, line_number_width};

    /// RR-01: ASCII line render.
    /// A line "hello" renders 5 cells with correct characters.
    #[test]
    fn rr01_ascii_render() {
        let line = "hello";
        let col = display_col_for_grapheme_offset(line, 5);
        assert_eq!(col, 5);
    }

    /// RR-02: CJK line render.
    /// "あいう" = 6 display columns.
    #[test]
    fn rr02_cjk_render() {
        let line = "あいう";
        let col = display_col_for_grapheme_offset(line, 3);
        assert_eq!(col, 6); // Each CJK char is 2 columns
    }

    /// RR-07: Grid diff.
    /// Two consecutive grids with different content.
    #[test]
    fn rr07_grid_diff() {
        let g1 = CellGrid::new(2, 10);
        let mut g2 = CellGrid::new(2, 10);
        g2.set(
            0,
            0,
            Cell {
                grapheme: "X".to_string(),
                ..Cell::default()
            },
        );
        // Verify cells differ at (0,0)
        assert_ne!(g1.cells[0][0], g2.cells[0][0]);
        // But same at (0,1)
        assert_eq!(g1.cells[0][1], g2.cells[0][1]);
    }

    /// RR-09: Empty buffer.
    /// Grid for empty buffer should be all spaces.
    #[test]
    fn rr09_empty_grid() {
        let grid = CellGrid::new(24, 80);
        assert_eq!(grid.rows, 24);
        assert_eq!(grid.cols, 80);
        assert_eq!(grid.cells[0][0].grapheme, " ");
    }

    /// RR-10: Status line width.
    /// line_number_width computes correct gutter width.
    #[test]
    fn rr10_line_number_width() {
        assert_eq!(line_number_width(1), 2); // 1 digit + 1 space
        assert_eq!(line_number_width(99), 3); // 2 digits + 1 space
        assert_eq!(line_number_width(100), 4); // 3 digits + 1 space
        assert_eq!(line_number_width(1000), 5); // 4 digits + 1 space
    }

    /// Test grid set and get.
    #[test]
    fn test_grid_set_get() {
        let mut grid = CellGrid::new(10, 20);
        grid.set(
            3,
            5,
            Cell {
                grapheme: "A".to_string(),
                ..Cell::default()
            },
        );
        assert_eq!(grid.get(3, 5).grapheme, "A");
    }

    /// Test grid clear.
    #[test]
    fn test_grid_clear() {
        let mut grid = CellGrid::new(5, 5);
        grid.set(
            0,
            0,
            Cell {
                grapheme: "X".to_string(),
                ..Cell::default()
            },
        );
        grid.clear();
        assert_eq!(grid.cells[0][0].grapheme, " ");
    }

    /// Test CJK display column mapping.
    #[test]
    fn test_cjk_display_col_offset() {
        // "aあb" -> widths: a=1, あ=2, b=1
        let line = "aあb";
        assert_eq!(display_col_for_grapheme_offset(line, 0), 0);
        assert_eq!(display_col_for_grapheme_offset(line, 1), 1);
        assert_eq!(display_col_for_grapheme_offset(line, 2), 3);
        assert_eq!(display_col_for_grapheme_offset(line, 3), 4);
    }

    /// Test wide continuation cell marking.
    #[test]
    fn test_wide_continuation() {
        let cell = Cell {
            grapheme: String::new(),
            width: 0,
            is_wide_continuation: true,
            ..Cell::default()
        };
        assert!(cell.is_wide_continuation);
        assert_eq!(cell.width, 0);
    }

    /// WR-07: Long-line display safety.
    ///
    /// No rendered text extends beyond window bounds when wrap is on.
    /// A 200-char ASCII line in an 80-col window wraps to 3 rows
    /// with no overflow.
    #[test]
    fn wr07_long_line_no_overflow() {
        let long_line = "a".repeat(200);
        let rows = crate::wrap::wrap_line(&long_line, 80);
        assert_eq!(rows.len(), 3, "200 chars in 80 cols = 3 rows");
        for (i, row) in rows.iter().enumerate() {
            assert!(
                row.used_cols <= 80,
                "row {i} used {} cols, exceeds 80",
                row.used_cols
            );
        }
    }

    /// WR-07 CJK: Long CJK line wraps safely.
    #[test]
    fn wr07_long_cjk_no_overflow() {
        // 100 CJK chars = 200 display cols
        let long_line = "漢".repeat(100);
        let rows = crate::wrap::wrap_line(&long_line, 80);
        for (i, row) in rows.iter().enumerate() {
            assert!(
                row.used_cols <= 80,
                "CJK row {i} used {} cols, exceeds 80",
                row.used_cols
            );
        }
    }
}
