//! Unique identifiers for editor entities.

use serde::{Deserialize, Serialize};

/// Unique identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(u64);

impl BufferId {
    /// Creates a new buffer ID from a raw value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// Buffer version for change tracking.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct BufferVersion(u64);

impl BufferVersion {
    /// Creates a new buffer version.
    pub fn new(version: u64) -> Self {
        Self(version)
    }

    /// Returns the raw version value.
    pub fn get(self) -> u64 {
        self.0
    }

    /// Increments the version.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// Unique identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(u64);

impl WindowId {
    /// Creates a new window ID from a raw value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// Identifier for a register (a-z, 0-9, special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisterId {
    /// Unnamed register (default)
    Unnamed,
    /// Named register (a-z)
    Named(char),
    /// Numbered register (0-9)
    Numbered(u8),
    /// System clipboard (+)
    Clipboard,
    /// Primary selection (*)
    Primary,
    /// Black hole register (_)
    BlackHole,
    /// Last search pattern (/)
    Search,
    /// Last command (:)
    Command,
    /// Current filename (%)
    Filename,
    /// Alternate filename (#)
    Alternate,
    /// Last inserted text (.)
    LastInsert,
    /// Expression register (=)
    Expression,
}

impl RegisterId {
    /// Creates a register ID from a character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '"' => Some(Self::Unnamed),
            'a'..='z' | 'A'..='Z' => Some(Self::Named(c.to_ascii_lowercase())),
            '0'..='9' => Some(Self::Numbered(c as u8 - b'0')),
            '+' => Some(Self::Clipboard),
            '*' => Some(Self::Primary),
            '_' => Some(Self::BlackHole),
            '/' => Some(Self::Search),
            ':' => Some(Self::Command),
            '%' => Some(Self::Filename),
            '#' => Some(Self::Alternate),
            '.' => Some(Self::LastInsert),
            '=' => Some(Self::Expression),
            _ => None,
        }
    }
}

/// Identifier for a mark (a-z, A-Z, special).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkId {
    /// Local mark (a-z)
    Local(char),
    /// Global mark (A-Z)
    Global(char),
    /// Last position before jump
    LastJump,
    /// Last change position
    LastChange,
    /// Last insert position
    LastInsert,
    /// Start of last visual selection
    VisualStart,
    /// End of last visual selection
    VisualEnd,
    /// Start of last change
    ChangeStart,
    /// End of last change
    ChangeEnd,
}

impl MarkId {
    /// Creates a mark ID from a character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a'..='z' => Some(Self::Local(c)),
            'A'..='Z' => Some(Self::Global(c)),
            '\'' | '`' => Some(Self::LastJump),
            '.' => Some(Self::LastChange),
            '^' => Some(Self::LastInsert),
            '<' => Some(Self::VisualStart),
            '>' => Some(Self::VisualEnd),
            '[' => Some(Self::ChangeStart),
            ']' => Some(Self::ChangeEnd),
            _ => None,
        }
    }
}
