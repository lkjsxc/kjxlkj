//! Buffer edit operations (insert / delete).
//!
//! Extracted from buffer.rs to keep each file ≤ 200 lines.

use crate::buffer::Buffer;
use crate::grapheme::grapheme_to_byte_offset;

impl Buffer {
    /// Insert text at the given line and grapheme offset.
    pub fn insert(
        &mut self,
        line: usize,
        grapheme_offset: usize,
        text: &str,
    ) -> Result<(), &'static str> {
        if self.readonly {
            return Err("buffer is readonly");
        }
        if line >= self.content.len_lines() {
            return Err("line out of bounds");
        }
        let line_slice = self.content.line(line);
        let byte_in_line =
            grapheme_to_byte_offset(line_slice, grapheme_offset)
                .ok_or("grapheme offset out of bounds")?;
        let line_byte_start = self.content.line_to_byte(line);
        let byte_offset = line_byte_start + byte_in_line;
        self.content.insert(byte_offset, text);
        self.version += 1;
        self.modified = true;
        Ok(())
    }

    /// Delete a range from (start_line, start_grapheme) to
    /// (end_line, end_grapheme).
    pub fn delete(
        &mut self,
        start_line: usize,
        start_grapheme: usize,
        end_line: usize,
        end_grapheme: usize,
    ) -> Result<(), &'static str> {
        if self.readonly {
            return Err("buffer is readonly");
        }
        let start_byte = self.grapheme_to_absolute_byte(
            start_line,
            start_grapheme,
        )?;
        let end_byte =
            self.grapheme_to_absolute_byte(end_line, end_grapheme)?;
        if start_byte >= end_byte {
            return Err("invalid range");
        }
        self.content.remove(start_byte..end_byte);
        self.version += 1;
        self.modified = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn insert_and_version() {
        let mut b = Buffer::from_text(BufferId(0), "test", "hello");
        assert_eq!(b.version, 0);
        b.insert(0, 5, " world").unwrap();
        assert_eq!(b.version, 1);
        assert!(b.modified);
        assert_eq!(b.line(0).unwrap(), "hello world");
    }

    #[test]
    fn delete_basic() {
        let mut b =
            Buffer::from_text(BufferId(0), "test", "hello world");
        b.delete(0, 5, 0, 11).unwrap();
        assert_eq!(b.line(0).unwrap(), "hello");
    }

    #[test]
    fn readonly_blocks_edit() {
        let mut b = Buffer::new_scratch(BufferId(0));
        b.readonly = true;
        assert!(b.insert(0, 0, "x").is_err());
    }
}
