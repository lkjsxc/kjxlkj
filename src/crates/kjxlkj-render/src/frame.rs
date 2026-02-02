//! Frame output.

use crate::ScreenBuffer;
use crossterm::{
    cursor::MoveTo,
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

/// A frame ready for output.
pub struct Frame {
    /// Screen buffer.
    buffer: ScreenBuffer,
}

impl Frame {
    /// Creates a new frame.
    pub fn new(buffer: ScreenBuffer) -> Self {
        Self { buffer }
    }

    /// Renders the frame to stdout.
    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.queue(Clear(ClearType::All))?;

        let dims = self.buffer.dimensions();
        for y in 0..dims.height {
            stdout.queue(MoveTo(0, y))?;
            for x in 0..dims.width {
                if let Some(cell) = self.buffer.get(x, y) {
                    stdout.queue(SetForegroundColor(cell.style.fg.into()))?;
                    stdout.queue(SetBackgroundColor(cell.style.bg.into()))?;
                    stdout.queue(Print(&cell.content))?;
                }
            }
        }

        stdout.flush()?;
        Ok(())
    }

    /// Dumps the frame to a string for debugging.
    pub fn dump(&self) -> String {
        let dims = self.buffer.dimensions();
        let mut output = String::new();
        output.push_str(&format!("=== Frame Dump ({}x{}) ===\n", dims.width, dims.height));
        
        for y in 0..dims.height {
            for x in 0..dims.width {
                if let Some(cell) = self.buffer.get(x, y) {
                    output.push_str(&cell.content);
                }
            }
            output.push('\n');
        }
        output
    }

    /// Returns the buffer.
    pub fn buffer(&self) -> &ScreenBuffer {
        &self.buffer
    }
}
