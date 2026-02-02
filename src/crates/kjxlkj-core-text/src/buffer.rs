//! Text buffer combining rope with metadata.

use crate::rope::TextRope;
use kjxlkj_core_types::{
    buffer::{BufferFlags, BufferInfo, BufferName},
    ids::{BufferId, BufferVersion},
    position::Position,
};

/// A text buffer with content and metadata.
#[derive(Debug)]
pub struct TextBuffer {
    /// Buffer identifier.
    id: BufferId,
    /// Buffer name/path.
    name: BufferName,
    /// The text content.
    rope: TextRope,
    /// Buffer flags.
    flags: BufferFlags,
    /// Version for change tracking.
    version: BufferVersion,
}

impl TextBuffer {
    /// Creates a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::Unnamed,
            rope: TextRope::new(),
            flags: BufferFlags::LISTED,
            version: BufferVersion::new(0),
        }
    }

    /// Creates a buffer from text content.
    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            name: BufferName::Unnamed,
            rope: TextRope::from_text(text),
            flags: BufferFlags::LISTED,
            version: BufferVersion::new(0),
        }
    }

    /// Returns the buffer ID.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Returns the buffer name.
    pub fn name(&self) -> &BufferName {
        &self.name
    }

    /// Sets the buffer name.
    pub fn set_name(&mut self, name: BufferName) {
        self.name = name;
    }

    /// Returns the buffer info.
    pub fn info(&self) -> BufferInfo {
        BufferInfo {
            id: self.id,
            name: self.name.clone(),
            flags: self.flags,
            line_count: self.rope.len_lines(),
        }
    }

    /// Returns the text rope.
    pub fn rope(&self) -> &TextRope {
        &self.rope
    }

    /// Returns a mutable reference to the text rope.
    pub fn rope_mut(&mut self) -> &mut TextRope {
        self.flags.insert(BufferFlags::MODIFIED);
        self.version = BufferVersion::new(self.version.get() + 1);
        &mut self.rope
    }

    /// Returns the current version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Returns true if the buffer is modified.
    pub fn is_modified(&self) -> bool {
        self.flags.contains(BufferFlags::MODIFIED)
    }

    /// Marks the buffer as saved.
    pub fn mark_saved(&mut self) {
        self.flags.remove(BufferFlags::MODIFIED);
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns a specific line.
    pub fn line(&self, idx: usize) -> Option<String> {
        self.rope.line(idx)
    }

    /// Converts position to character index.
    pub fn pos_to_idx(&self, pos: Position) -> Option<usize> {
        self.rope.pos_to_char_idx(pos)
    }

    /// Converts character index to position.
    pub fn idx_to_pos(&self, idx: usize) -> Option<Position> {
        self.rope.char_idx_to_pos(idx)
    }
}
