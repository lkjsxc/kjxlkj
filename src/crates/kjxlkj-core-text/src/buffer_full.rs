//! Extended buffer types: BufferType, BufferFlags, BufferInfo, AlternateTracker.

use kjxlkj_core_types::BufferId;
use serde::{Deserialize, Serialize};

/// The kind of buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BufferType {
    #[default]
    Normal,
    Scratch,
    Help,
    QuickFix,
    Terminal,
    Prompt,
    Popup,
}

/// Boolean flags associated with a buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferFlags {
    pub modified: bool,
    pub readonly: bool,
    pub listed: bool,
    pub loaded: bool,
    pub modifiable: bool,
}

impl Default for BufferFlags {
    fn default() -> Self {
        Self {
            modified: false,
            readonly: false,
            listed: true,
            loaded: true,
            modifiable: true,
        }
    }
}

/// Summary information about a buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferInfo {
    pub id: BufferId,
    pub name: String,
    pub buf_type: BufferType,
    pub flags: BufferFlags,
    pub filetype: String,
    pub encoding: String,
    pub line_count: usize,
}

impl BufferInfo {
    /// Create a minimal BufferInfo with sensible defaults.
    pub fn new(id: BufferId, name: String) -> Self {
        Self {
            id,
            name,
            buf_type: BufferType::default(),
            flags: BufferFlags::default(),
            filetype: String::new(),
            encoding: "utf-8".into(),
            line_count: 0,
        }
    }
}

/// Tracks the current and alternate buffer ids (like `#`).
#[derive(Debug, Clone)]
pub struct AlternateTracker {
    pub current: Option<BufferId>,
    pub alternate: Option<BufferId>,
}

impl AlternateTracker {
    pub fn new() -> Self {
        Self {
            current: None,
            alternate: None,
        }
    }

    /// Set a new current buffer; the old current becomes alternate.
    pub fn set_current(&mut self, id: BufferId) {
        if self.current != Some(id) {
            self.alternate = self.current;
            self.current = Some(id);
        }
    }

    /// Swap current and alternate.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.alternate);
    }
}

impl Default for AlternateTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Return only listed buffers.
pub fn filter_listed(buffers: &[BufferInfo]) -> Vec<&BufferInfo> {
    buffers.iter().filter(|b| b.flags.listed).collect()
}

/// Find a buffer by name.
pub fn find_by_name<'a>(buffers: &'a [BufferInfo], name: &str) -> Option<&'a BufferInfo> {
    buffers.iter().find(|b| b.name == name)
}

/// Count how many buffers are modified.
pub fn modified_count(buffers: &[BufferInfo]) -> usize {
    buffers.iter().filter(|b| b.flags.modified).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternate_tracker_swap() {
        let mut t = AlternateTracker::new();
        t.set_current(BufferId(1));
        t.set_current(BufferId(2));
        assert_eq!(t.current, Some(BufferId(2)));
        assert_eq!(t.alternate, Some(BufferId(1)));
        t.swap();
        assert_eq!(t.current, Some(BufferId(1)));
        assert_eq!(t.alternate, Some(BufferId(2)));
    }

    #[test]
    fn filter_listed_works() {
        let mut b1 = BufferInfo::new(BufferId(1), "a".into());
        let b2 = BufferInfo::new(BufferId(2), "b".into());
        b1.flags.listed = false;
        let bufs = vec![b1, b2];
        assert_eq!(filter_listed(&bufs).len(), 1);
    }

    #[test]
    fn modified_count_works() {
        let mut b = BufferInfo::new(BufferId(1), "a".into());
        b.flags.modified = true;
        let bufs = vec![b, BufferInfo::new(BufferId(2), "b".into())];
        assert_eq!(modified_count(&bufs), 1);
    }
}
