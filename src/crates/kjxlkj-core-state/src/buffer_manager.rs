//! Buffer manager for buffer-related features.
//!
//! Implements buffer navigation, alternate file, argument list, and buffer groups.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

/// Buffer ID type.
pub type BufferId = u64;

/// Buffer state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferState {
    /// Active in a window.
    Active,
    /// Hidden (loaded but not visible).
    Hidden,
    /// Unloaded.
    Unloaded,
}

/// Buffer flags.
#[derive(Debug, Clone, Default)]
pub struct BufferFlags {
    /// Has unsaved changes.
    pub modified: bool,
    /// Read-only buffer.
    pub readonly: bool,
    /// Special buffer (scratch, quickfix, etc.).
    pub special: bool,
    /// Listed in buffer list.
    pub listed: bool,
}

/// Buffer info.
#[derive(Debug, Clone)]
pub struct BufferInfo {
    /// Buffer ID.
    pub id: BufferId,
    /// Buffer name.
    pub name: String,
    /// File path (if file-backed).
    pub path: Option<PathBuf>,
    /// Buffer state.
    pub state: BufferState,
    /// Buffer flags.
    pub flags: BufferFlags,
    /// Line count.
    pub line_count: usize,
}

impl BufferInfo {
    /// Create a new buffer info.
    pub fn new(id: BufferId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            path: None,
            state: BufferState::Active,
            flags: BufferFlags { listed: true, ..Default::default() },
            line_count: 1,
        }
    }

    /// Set path.
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }
}

/// Alternate file tracking.
#[derive(Debug, Default)]
pub struct AlternateFile {
    /// Previous buffer ID.
    current: Option<BufferId>,
    /// Alternate buffer ID.
    alternate: Option<BufferId>,
}

impl AlternateFile {
    /// Create new alternate file tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current buffer (updates alternate).
    pub fn set_current(&mut self, id: BufferId) {
        if self.current != Some(id) {
            self.alternate = self.current;
            self.current = Some(id);
        }
    }

    /// Get current buffer.
    pub fn current(&self) -> Option<BufferId> {
        self.current
    }

    /// Get alternate buffer.
    pub fn alternate(&self) -> Option<BufferId> {
        self.alternate
    }

    /// Toggle between current and alternate.
    pub fn toggle(&mut self) -> Option<BufferId> {
        if let Some(alt) = self.alternate {
            std::mem::swap(&mut self.current, &mut self.alternate);
            Some(alt)
        } else {
            None
        }
    }
}

/// Argument list.
#[derive(Debug, Default)]
pub struct ArgList {
    /// Arguments (file paths).
    args: Vec<PathBuf>,
    /// Current index.
    index: usize,
}

impl ArgList {
    /// Create a new argument list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from paths.
    pub fn from_paths(paths: Vec<PathBuf>) -> Self {
        Self { args: paths, index: 0 }
    }

    /// Get number of arguments.
    pub fn len(&self) -> usize {
        self.args.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    /// Get current argument.
    pub fn current(&self) -> Option<&PathBuf> {
        self.args.get(self.index)
    }

    /// Get current index.
    pub fn current_index(&self) -> usize {
        self.index
    }

    /// Go to next argument.
    pub fn next_arg(&mut self) -> Option<&PathBuf> {
        if self.index + 1 < self.args.len() {
            self.index += 1;
            self.current()
        } else {
            None
        }
    }

    /// Go to previous argument.
    pub fn prev_arg(&mut self) -> Option<&PathBuf> {
        if self.index > 0 {
            self.index -= 1;
            self.current()
        } else {
            None
        }
    }

    /// Go to first argument.
    pub fn first(&mut self) -> Option<&PathBuf> {
        self.index = 0;
        self.current()
    }

    /// Go to last argument.
    pub fn last(&mut self) -> Option<&PathBuf> {
        if !self.args.is_empty() {
            self.index = self.args.len() - 1;
            self.current()
        } else {
            None
        }
    }

    /// Go to specific index.
    pub fn goto(&mut self, index: usize) -> Option<&PathBuf> {
        if index < self.args.len() {
            self.index = index;
            self.current()
        } else {
            None
        }
    }

    /// Add argument.
    pub fn add(&mut self, path: PathBuf) {
        self.args.push(path);
    }

    /// Clear all arguments.
    pub fn clear(&mut self) {
        self.args.clear();
        self.index = 0;
    }

    /// Get all arguments.
    pub fn args(&self) -> &[PathBuf] {
        &self.args
    }
}

/// Buffer group.
#[derive(Debug, Clone)]
pub struct BufferGroup {
    /// Group name.
    pub name: String,
    /// Buffer IDs in this group.
    pub buffers: HashSet<BufferId>,
}

impl BufferGroup {
    /// Create a new buffer group.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            buffers: HashSet::new(),
        }
    }

    /// Add buffer to group.
    pub fn add(&mut self, id: BufferId) {
        self.buffers.insert(id);
    }

    /// Remove buffer from group.
    pub fn remove(&mut self, id: BufferId) {
        self.buffers.remove(&id);
    }

    /// Check if buffer is in group.
    pub fn contains(&self, id: BufferId) -> bool {
        self.buffers.contains(&id)
    }
}

/// Buffer manager.
#[derive(Debug, Default)]
pub struct BufferManager {
    /// All buffers.
    buffers: HashMap<BufferId, BufferInfo>,
    /// Buffer order (for :bn/:bp).
    order: VecDeque<BufferId>,
    /// Alternate file tracking.
    alternate: AlternateFile,
    /// Argument list.
    arglist: ArgList,
    /// Buffer groups.
    groups: HashMap<String, BufferGroup>,
    /// Next buffer ID.
    next_id: BufferId,
}

impl BufferManager {
    /// Create a new buffer manager.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Add a buffer.
    pub fn add(&mut self, info: BufferInfo) {
        let id = info.id;
        self.buffers.insert(id, info);
        self.order.push_back(id);
    }

    /// Create a new buffer and return its ID.
    pub fn create(&mut self, name: impl Into<String>) -> BufferId {
        let id = self.next_id;
        self.next_id += 1;
        let info = BufferInfo::new(id, name);
        self.add(info);
        id
    }

    /// Get buffer info.
    pub fn get(&self, id: BufferId) -> Option<&BufferInfo> {
        self.buffers.get(&id)
    }

    /// Get mutable buffer info.
    pub fn get_mut(&mut self, id: BufferId) -> Option<&mut BufferInfo> {
        self.buffers.get_mut(&id)
    }

    /// Remove a buffer.
    pub fn remove(&mut self, id: BufferId) -> Option<BufferInfo> {
        self.order.retain(|&b| b != id);
        self.buffers.remove(&id)
    }

    /// Get number of buffers.
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// List all buffers.
    pub fn list(&self) -> Vec<&BufferInfo> {
        self.order.iter().filter_map(|id| self.buffers.get(id)).collect()
    }

    /// List only listed buffers.
    pub fn listed(&self) -> Vec<&BufferInfo> {
        self.list().into_iter().filter(|b| b.flags.listed).collect()
    }

    /// Get next buffer.
    pub fn next(&self, current: BufferId) -> Option<BufferId> {
        let listed: Vec<_> = self.order.iter().copied().filter(|&id| {
            self.buffers.get(&id).map(|b| b.flags.listed).unwrap_or(false)
        }).collect();

        if let Some(pos) = listed.iter().position(|&id| id == current) {
            let next_pos = (pos + 1) % listed.len();
            listed.get(next_pos).copied()
        } else {
            listed.first().copied()
        }
    }

    /// Get previous buffer.
    pub fn prev(&self, current: BufferId) -> Option<BufferId> {
        let listed: Vec<_> = self.order.iter().copied().filter(|&id| {
            self.buffers.get(&id).map(|b| b.flags.listed).unwrap_or(false)
        }).collect();

        if let Some(pos) = listed.iter().position(|&id| id == current) {
            let prev_pos = if pos == 0 { listed.len() - 1 } else { pos - 1 };
            listed.get(prev_pos).copied()
        } else {
            listed.last().copied()
        }
    }

    /// Set current buffer (for alternate tracking).
    pub fn set_current(&mut self, id: BufferId) {
        self.alternate.set_current(id);
    }

    /// Get alternate buffer.
    pub fn alternate(&self) -> Option<BufferId> {
        self.alternate.alternate()
    }

    /// Toggle to alternate buffer.
    pub fn toggle_alternate(&mut self) -> Option<BufferId> {
        self.alternate.toggle()
    }

    /// Get argument list.
    pub fn arglist(&self) -> &ArgList {
        &self.arglist
    }

    /// Get mutable argument list.
    pub fn arglist_mut(&mut self) -> &mut ArgList {
        &mut self.arglist
    }

    /// Create a buffer group.
    pub fn create_group(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.groups.contains_key(&name) {
            self.groups.insert(name.clone(), BufferGroup::new(name));
        }
    }

    /// Get a buffer group.
    pub fn group(&self, name: &str) -> Option<&BufferGroup> {
        self.groups.get(name)
    }

    /// Add buffer to group.
    pub fn add_to_group(&mut self, group: &str, id: BufferId) {
        if let Some(g) = self.groups.get_mut(group) {
            g.add(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_info_new() {
        let info = BufferInfo::new(1, "test");
        assert_eq!(info.id, 1);
        assert_eq!(info.name, "test");
        assert!(info.flags.listed);
    }

    #[test]
    fn test_alternate_file() {
        let mut alt = AlternateFile::new();
        alt.set_current(1);
        alt.set_current(2);
        assert_eq!(alt.current(), Some(2));
        assert_eq!(alt.alternate(), Some(1));

        assert_eq!(alt.toggle(), Some(1));
        assert_eq!(alt.current(), Some(1));
        assert_eq!(alt.alternate(), Some(2));
    }

    #[test]
    fn test_arglist() {
        let mut args = ArgList::new();
        args.add(PathBuf::from("a.txt"));
        args.add(PathBuf::from("b.txt"));
        args.add(PathBuf::from("c.txt"));

        assert_eq!(args.len(), 3);
        assert_eq!(args.current(), Some(&PathBuf::from("a.txt")));

        assert_eq!(args.next_arg(), Some(&PathBuf::from("b.txt")));
        assert_eq!(args.next_arg(), Some(&PathBuf::from("c.txt")));
        assert_eq!(args.next_arg(), None);

        assert_eq!(args.prev_arg(), Some(&PathBuf::from("b.txt")));
        assert_eq!(args.first(), Some(&PathBuf::from("a.txt")));
        assert_eq!(args.last(), Some(&PathBuf::from("c.txt")));
    }

    #[test]
    fn test_buffer_group() {
        let mut group = BufferGroup::new("test");
        group.add(1);
        group.add(2);
        assert!(group.contains(1));
        assert!(group.contains(2));
        group.remove(1);
        assert!(!group.contains(1));
    }

    #[test]
    fn test_buffer_manager_create() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        let id2 = manager.create("buf2");
        assert_eq!(manager.len(), 2);
        assert_eq!(manager.get(id1).unwrap().name, "buf1");
        assert_eq!(manager.get(id2).unwrap().name, "buf2");
    }

    #[test]
    fn test_buffer_manager_next_prev() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        let id2 = manager.create("buf2");
        let id3 = manager.create("buf3");

        assert_eq!(manager.next(id1), Some(id2));
        assert_eq!(manager.next(id2), Some(id3));
        assert_eq!(manager.next(id3), Some(id1));

        assert_eq!(manager.prev(id1), Some(id3));
        assert_eq!(manager.prev(id3), Some(id2));
    }

    #[test]
    fn test_buffer_manager_alternate() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        let id2 = manager.create("buf2");

        manager.set_current(id1);
        manager.set_current(id2);

        assert_eq!(manager.alternate(), Some(id1));
        assert_eq!(manager.toggle_alternate(), Some(id1));
    }

    #[test]
    fn test_buffer_manager_groups() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        let id2 = manager.create("buf2");

        manager.create_group("test");
        manager.add_to_group("test", id1);
        manager.add_to_group("test", id2);

        let group = manager.group("test").unwrap();
        assert!(group.contains(id1));
        assert!(group.contains(id2));
    }
}
