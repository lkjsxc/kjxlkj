//! Tag types and definitions.

use kjxlkj_core_types::{BufferId, Position};
use std::path::PathBuf;

/// A tag entry on the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagEntry {
    /// Tag name that was jumped to.
    pub tag: String,
    /// Buffer we jumped from.
    pub from_buffer: BufferId,
    /// Position we jumped from.
    pub from_position: Position,
    /// Buffer we jumped to.
    pub to_buffer: BufferId,
    /// Position we jumped to.
    pub to_position: Position,
}

impl TagEntry {
    /// Creates a new tag entry.
    pub fn new(
        tag: String,
        from_buffer: BufferId,
        from_position: Position,
        to_buffer: BufferId,
        to_position: Position,
    ) -> Self {
        Self {
            tag,
            from_buffer,
            from_position,
            to_buffer,
            to_position,
        }
    }
}

/// A tag definition from a tags file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagDef {
    /// Tag name.
    pub name: String,
    /// File containing the tag.
    pub file: PathBuf,
    /// Line number or pattern.
    pub location: TagLocation,
    /// Tag kind (function, class, etc).
    pub kind: Option<String>,
}

/// Location of a tag in a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagLocation {
    /// Line number.
    Line(usize),
    /// Search pattern.
    Pattern(String),
}

impl TagDef {
    /// Creates a new tag definition.
    pub fn new(name: String, file: PathBuf, location: TagLocation) -> Self {
        Self {
            name,
            file,
            location,
            kind: None,
        }
    }

    /// Sets the tag kind.
    pub fn with_kind(mut self, kind: String) -> Self {
        self.kind = Some(kind);
        self
    }
}

/// Manages multiple tags with the same name.
#[derive(Debug, Default)]
pub struct TagMatch {
    /// All matching tags.
    matches: Vec<TagDef>,
    /// Current match index.
    index: usize,
}

impl TagMatch {
    /// Creates empty tag match.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates from tag definitions.
    pub fn from_tags(tags: Vec<TagDef>) -> Self {
        Self { matches: tags, index: 0 }
    }

    /// Gets current match.
    pub fn current(&self) -> Option<&TagDef> {
        self.matches.get(self.index)
    }

    /// Moves to next match.
    pub fn advance_next(&mut self) -> Option<&TagDef> {
        if self.index + 1 < self.matches.len() {
            self.index += 1;
        }
        self.current()
    }

    /// Moves to previous match.
    pub fn prev(&mut self) -> Option<&TagDef> {
        if self.index > 0 {
            self.index -= 1;
        }
        self.current()
    }

    /// Gets all matches.
    pub fn matches(&self) -> &[TagDef] {
        &self.matches
    }

    /// Gets match count.
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    /// Checks if empty.
    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}
