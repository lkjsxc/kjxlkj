//! Paint a frame to stdout using crossterm.
//!
//! Uses diff rendering and batched IO per
//! /docs/spec/architecture/render-pipeline.md.

use crate::frame::Frame;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use kjxlkj_core_types::EditorSnapshot;
use std::io::{self, Stdout, Write};

/// Paint the frame to stdout using crossterm commands.
///
/// All output is batched into one flush call per frame.
pub fn paint_to_stdout(
    frame: &Frame,
    snapshot: &EditorSnapshot,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(Hide)?;

    for (row_idx, row_content) in frame.rows.iter().enumerate() {
        stdout.queue(MoveTo(0, row_idx as u16))?;
        stdout.queue(Print(row_content))?;
    }

    // Position cursor.
    stdout.queue(MoveTo(frame.cursor_col, frame.cursor_row))?;
    stdout.queue(Show)?;
    stdout.flush()?;
    Ok(())
}
