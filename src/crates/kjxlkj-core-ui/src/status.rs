//! Status line utilities.

use kjxlkj_core_types::{BufferMeta, Cursor, Mode};

use crate::StatusSnapshot;

/// Build the status line left content.
pub fn build_status_left(mode: Mode, buffer: &BufferMeta) -> String {
    let modified = if buffer.modified { "[+]" } else { "" };
    format!("{} {}{}", mode.name(), buffer.name, modified)
}

/// Build the status line right content.
pub fn build_status_right(cursor: &Cursor, line_count: usize) -> String {
    format!(
        "{}:{} ({}/{})",
        cursor.line() + 1,
        cursor.col() + 1,
        cursor.line() + 1,
        line_count
    )
}

/// Build a complete status snapshot.
pub fn build_status(mode: Mode, buffer: &BufferMeta, cursor: &Cursor, line_count: usize) -> StatusSnapshot {
    StatusSnapshot {
        left: build_status_left(mode, buffer),
        right: build_status_right(cursor, line_count),
    }
}
