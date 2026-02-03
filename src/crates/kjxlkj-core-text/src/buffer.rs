//! Buffer implementation using ropey.

use ropey::Rope;
use std::path::PathBuf;

use kjxlkj_core_types::{BufferId, BufferMeta, BufferName, BufferVersion, LineEnding};

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

    /// Get mutable access to the rope.
    pub(crate) fn rope_mut(&mut self) -> &mut Rope {
        &mut self.rope
    }

    /// Get line ending style.
    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    /// Mark buffer as modified and increment version.
    pub(crate) fn mark_modified(&mut self) {
        self.modified = true;
        self.version.increment();
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
