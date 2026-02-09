//! Terminal snapshot for rendering terminal windows.

use serde::{Deserialize, Serialize};

use kjxlkj_core_types::{Cell, TerminalId};

/// Immutable terminal screen snapshot for the render task.
///
/// Terminal windows copy their internal screen buffer into this
/// structure so the renderer can display PTY output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSnapshot {
    /// Terminal identity.
    pub id: TerminalId,
    /// Screen dimensions (cols, rows).
    pub cols: u16,
    pub rows: u16,
    /// Flattened cell grid (row-major order).
    pub cells: Vec<Cell>,
    /// Cursor position (col, row), None if hidden.
    pub cursor: Option<(u16, u16)>,
    /// Title set by escape sequences.
    pub title: String,
    /// Whether the terminal process has exited.
    pub exited: bool,
    /// Exit code if exited.
    pub exit_code: Option<i32>,
}

impl TerminalSnapshot {
    /// Create an empty terminal snapshot.
    pub fn empty(id: TerminalId, cols: u16, rows: u16) -> Self {
        let cell_count = cols as usize * rows as usize;
        Self {
            id,
            cols,
            rows,
            cells: vec![Cell::default(); cell_count],
            cursor: Some((0, 0)),
            title: String::new(),
            exited: false,
            exit_code: None,
        }
    }

    /// Get a cell at (col, row).
    pub fn cell_at(&self, col: u16, row: u16) -> Option<&Cell> {
        if col < self.cols && row < self.rows {
            let idx = row as usize * self.cols as usize + col as usize;
            self.cells.get(idx)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_terminal() {
        let snap = TerminalSnapshot::empty(TerminalId(1), 80, 24);
        assert_eq!(snap.cells.len(), 80 * 24);
        assert_eq!(snap.cursor, Some((0, 0)));
    }

    #[test]
    fn cell_access() {
        let snap = TerminalSnapshot::empty(TerminalId(1), 10, 5);
        assert!(snap.cell_at(0, 0).is_some());
        assert!(snap.cell_at(10, 0).is_none());
    }
}
