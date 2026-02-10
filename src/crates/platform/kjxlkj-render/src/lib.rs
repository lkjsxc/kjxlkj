//! Rendering to terminal.
//!
//! This crate provides the render task and cell grid output.

mod color;
mod grid;
mod painter;
mod task;

pub use painter::*;
pub use task::*;

#[cfg(test)]
mod tests {
    use crate::grid::render_line_to_grid;
    use kjxlkj_core_ui::CellGrid;

    #[test]
    fn test_ascii_wrap_no_overflow() {
        let mut grid = CellGrid::new(10, 1);
        let line = "Hello World!"; // 12 chars
        render_line_to_grid(&mut grid, line, 0, 10);

        // Grid should have exactly 10 columns filled.
        for x in 0..10 {
            assert!(grid.get(x, 0).is_some());
        }

        // First 10 characters rendered: "Hello Worl".
        assert_eq!(grid.get(0, 0).unwrap().grapheme, "H");
        assert_eq!(grid.get(9, 0).unwrap().grapheme, "l"); // 10th char is 'l'.
    }

    #[test]
    fn test_wide_grapheme_padding() {
        let mut grid = CellGrid::new(5, 1);
        // "ab你" where 你 is width 2.
        let line = "ab你c";
        render_line_to_grid(&mut grid, line, 0, 5);

        // "a" at 0, "b" at 1, "你" at 2-3, "c" at 4.
        assert_eq!(grid.get(0, 0).unwrap().grapheme, "a");
        assert_eq!(grid.get(1, 0).unwrap().grapheme, "b");
        assert_eq!(grid.get(2, 0).unwrap().grapheme, "你");
        assert!(grid.get(3, 0).unwrap().is_wide_continuation);
        assert_eq!(grid.get(4, 0).unwrap().grapheme, "c");
    }

    #[test]
    fn test_wide_at_boundary_gets_padding() {
        let mut grid = CellGrid::new(3, 1);
        // "ab你" - "你" can't fit, should get padding.
        let line = "ab你";
        render_line_to_grid(&mut grid, line, 0, 3);

        assert_eq!(grid.get(0, 0).unwrap().grapheme, "a");
        assert_eq!(grid.get(1, 0).unwrap().grapheme, "b");
        // Position 2 should be padding (space) because 你 doesn't fit.
        assert_eq!(grid.get(2, 0).unwrap().grapheme, " ");
    }

    #[test]
    fn test_empty_line() {
        let mut grid = CellGrid::new(10, 1);
        render_line_to_grid(&mut grid, "", 0, 10);

        // All cells should be default (space).
        for x in 0..10 {
            assert_eq!(grid.get(x, 0).unwrap().grapheme, " ");
        }
    }

    #[test]
    fn test_single_column_wrap() {
        let mut grid = CellGrid::new(1, 1);
        let line = "abc";
        render_line_to_grid(&mut grid, line, 0, 1);

        // Only first char fits.
        assert_eq!(grid.get(0, 0).unwrap().grapheme, "a");
    }
}
