//! Frame diff computation.

use kjxlkj_core_types::Cell;

use crate::CellGrid;

/// A single cell change in the diff.
#[derive(Debug, Clone)]
pub struct CellChange {
    pub col: u16,
    pub row: u16,
    pub cell: Cell,
}

/// Diff between two frames.
#[derive(Debug, Default)]
pub struct FrameDiff {
    /// List of changed cells.
    pub changes: Vec<CellChange>,
    /// Whether a full redraw is needed.
    pub full_redraw: bool,
}

impl FrameDiff {
    /// Compute the diff between a previous and current cell grid.
    pub fn compute(
        prev: &CellGrid,
        curr: &CellGrid,
    ) -> Self {
        // If dimensions changed, full redraw.
        if prev.cols != curr.cols || prev.rows != curr.rows {
            return Self::full(curr);
        }

        let mut changes = Vec::new();

        for row in 0..curr.rows {
            for col in 0..curr.cols {
                let prev_cell = prev.get(col, row);
                let curr_cell = curr.get(col, row);
                if prev_cell != curr_cell {
                    changes.push(CellChange {
                        col,
                        row,
                        cell: curr_cell.clone(),
                    });
                }
            }
        }

        Self {
            changes,
            full_redraw: false,
        }
    }

    /// Create a full-redraw diff from a grid.
    pub fn full(grid: &CellGrid) -> Self {
        let mut changes = Vec::with_capacity(grid.cell_count());
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                changes.push(CellChange {
                    col,
                    row,
                    cell: grid.get(col, row).clone(),
                });
            }
        }
        Self {
            changes,
            full_redraw: true,
        }
    }

    /// Number of changed cells.
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Whether there are any changes.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Color;

    #[test]
    fn identical_grids_no_diff() {
        let a = CellGrid::new(10, 5);
        let b = CellGrid::new(10, 5);
        let diff = FrameDiff::compute(&a, &b);
        assert!(diff.is_empty());
    }

    #[test]
    fn different_size_full_redraw() {
        let a = CellGrid::new(10, 5);
        let b = CellGrid::new(20, 10);
        let diff = FrameDiff::compute(&a, &b);
        assert!(diff.full_redraw);
    }

    #[test]
    fn single_cell_change() {
        let a = CellGrid::new(10, 5);
        let mut b = CellGrid::new(10, 5);
        let mut cell = Cell::default();
        cell.fg = Color::Indexed(1);
        b.set(3, 2, cell);
        let diff = FrameDiff::compute(&a, &b);
        assert_eq!(diff.change_count(), 1);
        assert_eq!(diff.changes[0].col, 3);
        assert_eq!(diff.changes[0].row, 2);
    }
}
