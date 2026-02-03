//! Buffer implementation using ropey.

use ropey::Rope;
use std::path::PathBuf;

use kjxlkj_core_types::{
    BufferId, BufferMeta, BufferName, BufferVersion, EditorError, EditorResult, LineEnding,
    Position, Range,
};

/// A text buffer backed by a rope.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    /// Buffer identifier.
    id: BufferId,
    /// Display name.
    name: BufferName,
    /// Optional file path.
    path: Option<PathBuf>,
    /// Text content.
    rope: Rope,
    /// Whether the buffer has been modified.
    modified: bool,
    /// Version counter.
    version: BufferVersion,
    /// Line ending style.
    line_ending: LineEnding,
}

impl TextBuffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        Self {
            id: BufferId::new(),
            name: BufferName::new("[No Name]"),
            path: None,
            rope: Rope::new(),
            modified: false,
            version: BufferVersion::new(0),
            line_ending: LineEnding::Lf,
        }
    }

    /// Create a buffer from text content.
    pub fn from_text(text: &str) -> Self {
        let line_ending = detect_line_ending(text);
        Self {
            id: BufferId::new(),
            name: BufferName::new("[No Name]"),
            path: None,
            rope: Rope::from_str(text),
            modified: false,
            version: BufferVersion::new(0),
            line_ending,
        }
    }

    /// Create a buffer from a file path and content.
    pub fn from_file(path: PathBuf, content: &str) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "[No Name]".to_string());
        let line_ending = detect_line_ending(content);

        Self {
            id: BufferId::new(),
            name: BufferName::new(name),
            path: Some(path),
            rope: Rope::from_str(content),
            modified: false,
            version: BufferVersion::new(0),
            line_ending,
        }
    }

    /// Get the buffer ID.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Get buffer metadata.
    pub fn meta(&self) -> BufferMeta {
        BufferMeta {
            id: self.id,
            name: self.name.clone(),
            path: self.path.clone(),
            modified: self.modified,
            version: self.version,
            line_ending: self.line_ending,
        }
    }

    /// Get the file path.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Set the file path.
    pub fn set_path(&mut self, path: PathBuf) {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "[No Name]".to_string());
        self.name = BufferName::new(name);
        self.path = Some(path);
    }

    /// Get the buffer version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Check if the buffer is modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark the buffer as unmodified.
    pub fn mark_clean(&mut self) {
        self.modified = false;
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get a line's text (without line ending).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.line_count() {
            return None;
        }
        let line = self.rope.line(line_idx);
        let text = line.to_string();
        Some(text.trim_end_matches(['\n', '\r']).to_string())
    }

    /// Get the length of a line in graphemes.
    pub fn line_len(&self, line_idx: usize) -> usize {
        self.line(line_idx)
            .map(|s| super::grapheme_count(&s))
            .unwrap_or(0)
    }

    /// Get the full text content.
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Get the underlying rope (for advanced operations).
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Convert a position to a char offset (for rope operations).
    pub fn pos_to_offset(&self, pos: Position) -> EditorResult<usize> {
        let line_idx = pos.line as usize;
        if line_idx >= self.line_count() {
            return Err(EditorError::InvalidPosition {
                line: pos.line,
                col: pos.col,
            });
        }

        let line_start = self.rope.line_to_char(line_idx);
        let line_text = self.line(line_idx).unwrap_or_default();
        let char_offset = super::grapheme_to_char_offset(&line_text, pos.col as usize);

        Ok(line_start + char_offset)
    }

    /// Insert text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) -> EditorResult<()> {
        let offset = self.pos_to_offset(pos)?;
        self.rope.insert(offset, text);
        self.modified = true;
        self.version.increment();
        Ok(())
    }

    /// Delete a range of text.
    pub fn delete(&mut self, range: Range) -> EditorResult<String> {
        let norm = range.normalized();
        let start = self.pos_to_offset(norm.start)?;
        let end = self.pos_to_offset(norm.end)?;

        if end > start {
            let deleted: String = self.rope.slice(start..end).to_string();
            self.rope.remove(start..end);
            self.modified = true;
            self.version.increment();
            Ok(deleted)
        } else {
            Ok(String::new())
        }
    }

    /// Delete a single line.
    pub fn delete_line(&mut self, line_idx: usize) -> EditorResult<String> {
        if line_idx >= self.line_count() {
            return Err(EditorError::InvalidPosition {
                line: line_idx as u32,
                col: 0,
            });
        }

        let start = self.rope.line_to_char(line_idx);
        let end = if line_idx + 1 < self.line_count() {
            self.rope.line_to_char(line_idx + 1)
        } else {
            self.rope.len_chars()
        };

        let deleted: String = self.rope.slice(start..end).to_string();
        self.rope.remove(start..end);
        self.modified = true;
        self.version.increment();
        Ok(deleted)
    }

    /// Insert a new line at the given line index.
    pub fn insert_line(&mut self, line_idx: usize, text: &str) -> EditorResult<()> {
        let offset = if line_idx >= self.line_count() {
            self.rope.len_chars()
        } else {
            self.rope.line_to_char(line_idx)
        };

        let line_with_ending = if text.ends_with('\n') {
            text.to_string()
        } else {
            format!("{}{}", text, self.line_ending.as_str())
        };

        self.rope.insert(offset, &line_with_ending);
        self.modified = true;
        self.version.increment();
        Ok(())
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect line ending style from text.
fn detect_line_ending(text: &str) -> LineEnding {
    if text.contains("\r\n") {
        LineEnding::CrLf
    } else {
        LineEnding::Lf
    }
}
