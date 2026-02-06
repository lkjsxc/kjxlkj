//! Soft-wrap display-row model for viewport management.
//!
//! Computes display rows from buffer lines when `wrap = true`,
//! accounting for line width vs viewport width.

/// A display row referencing a buffer line and a column offset within it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayRow {
    pub buffer_line: usize,
    pub wrap_offset: usize,
    pub display_cols: usize,
}

/// Result of computing display rows for a range of buffer lines.
#[derive(Debug, Clone)]
pub struct DisplayMap {
    pub rows: Vec<DisplayRow>,
    pub viewport_width: usize,
}

impl DisplayMap {
    /// Total number of display rows.
    pub fn total_rows(&self) -> usize { self.rows.len() }

    /// Find the display row index for a given buffer line and column.
    pub fn display_row_for(&self, line: usize, col: usize) -> Option<usize> {
        self.rows.iter().position(|r| {
            r.buffer_line == line && col >= r.wrap_offset && col < r.wrap_offset + r.display_cols
        }).or_else(|| {
            // If col is past the last wrap segment, return last row for that line
            self.rows.iter().rposition(|r| r.buffer_line == line)
        })
    }

    /// Get all display rows for a given buffer line.
    pub fn rows_for_line(&self, line: usize) -> Vec<&DisplayRow> {
        self.rows.iter().filter(|r| r.buffer_line == line).collect()
    }
}

/// Compute display rows for buffer lines with the given widths.
/// `line_widths` is a slice of (line_index, display_width_in_columns).
pub fn compute_display_rows(line_widths: &[(usize, usize)], viewport_width: usize) -> DisplayMap {
    let vw = viewport_width.max(1);
    let mut rows = Vec::new();
    for &(line_idx, line_width) in line_widths {
        if line_width == 0 {
            rows.push(DisplayRow { buffer_line: line_idx, wrap_offset: 0, display_cols: 0 });
        } else {
            let mut offset = 0;
            while offset < line_width {
                let remaining = line_width - offset;
                let cols = remaining.min(vw);
                rows.push(DisplayRow { buffer_line: line_idx, wrap_offset: offset, display_cols: cols });
                offset += vw;
            }
        }
    }
    DisplayMap { rows, viewport_width: vw }
}

/// Compute effective vertical margin clamped to half the viewport.
pub fn effective_scrolloff(scrolloff: usize, text_rows: usize) -> usize {
    scrolloff.min((text_rows.saturating_sub(1)) / 2)
}

/// Compute effective horizontal margin clamped to half the viewport.
pub fn effective_sidescrolloff(sidescrolloff: usize, text_cols: usize) -> usize {
    sidescrolloff.min((text_cols.saturating_sub(1)) / 2)
}

/// Perform cursor-follow for the wrap model: adjust top_line so cursor is visible.
/// Returns the new top_line (as buffer line index).
pub fn follow_cursor_wrap(
    display_map: &DisplayMap,
    cursor_line: usize,
    cursor_col: usize,
    top_line: usize,
    text_rows: usize,
    scrolloff: usize,
) -> usize {
    let v_margin = effective_scrolloff(scrolloff, text_rows);
    let cursor_drow = match display_map.display_row_for(cursor_line, cursor_col) {
        Some(dr) => dr,
        None => return top_line,
    };
    // Find display row for current top_line
    let top_drow = display_map.display_row_for(top_line, 0).unwrap_or(0);
    let cursor_in_viewport = cursor_drow.saturating_sub(top_drow);
    let min_row = v_margin;
    let max_row = text_rows.saturating_sub(1).saturating_sub(v_margin);
    let new_top_drow = if cursor_in_viewport < min_row {
        cursor_drow.saturating_sub(min_row)
    } else if cursor_in_viewport > max_row {
        cursor_drow.saturating_sub(max_row)
    } else {
        return top_line;
    };
    // Convert display row back to buffer line
    display_map.rows.get(new_top_drow).map(|r| r.buffer_line).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_no_wrap() {
        let dm = compute_display_rows(&[(0, 40)], 80);
        assert_eq!(dm.total_rows(), 1);
        assert_eq!(dm.rows[0].wrap_offset, 0);
    }

    #[test]
    fn single_line_wraps() {
        let dm = compute_display_rows(&[(0, 200)], 80);
        assert_eq!(dm.total_rows(), 3); // 80 + 80 + 40 = 200
        assert_eq!(dm.rows[0].wrap_offset, 0);
        assert_eq!(dm.rows[1].wrap_offset, 80);
        assert_eq!(dm.rows[2].wrap_offset, 160);
        assert_eq!(dm.rows[2].display_cols, 40);
    }

    #[test]
    fn empty_line() {
        let dm = compute_display_rows(&[(0, 0)], 80);
        assert_eq!(dm.total_rows(), 1);
        assert_eq!(dm.rows[0].display_cols, 0);
    }

    #[test]
    fn multiple_lines_mixed() {
        let dm = compute_display_rows(&[(0, 40), (1, 160), (2, 10)], 80);
        // Line 0: 1 row; Line 1: 2 rows; Line 2: 1 row = 4 total
        assert_eq!(dm.total_rows(), 4);
        assert_eq!(dm.rows[0].buffer_line, 0);
        assert_eq!(dm.rows[1].buffer_line, 1);
        assert_eq!(dm.rows[2].buffer_line, 1);
        assert_eq!(dm.rows[3].buffer_line, 2);
    }

    #[test]
    fn display_row_for_position() {
        let dm = compute_display_rows(&[(0, 200)], 80);
        assert_eq!(dm.display_row_for(0, 0), Some(0));
        assert_eq!(dm.display_row_for(0, 79), Some(0));
        assert_eq!(dm.display_row_for(0, 80), Some(1));
        assert_eq!(dm.display_row_for(0, 199), Some(2));
    }

    #[test]
    fn effective_margins() {
        assert_eq!(effective_scrolloff(3, 20), 3);
        assert_eq!(effective_scrolloff(15, 20), 9); // (20-1)/2 = 9
        assert_eq!(effective_sidescrolloff(5, 10), 4); // (10-1)/2 = 4
    }

    #[test]
    fn follow_cursor_scrolls_down() {
        let dm = compute_display_rows(
            &(0..30).map(|i| (i, 40)).collect::<Vec<_>>(), 80
        );
        let new_top = follow_cursor_wrap(&dm, 25, 0, 0, 20, 3);
        assert!(new_top > 0, "should scroll down to show cursor");
    }

    #[test]
    fn follow_cursor_no_change() {
        let dm = compute_display_rows(
            &(0..30).map(|i| (i, 40)).collect::<Vec<_>>(), 80
        );
        let new_top = follow_cursor_wrap(&dm, 10, 0, 5, 20, 3);
        assert_eq!(new_top, 5, "cursor is visible, no scroll needed");
    }
}
