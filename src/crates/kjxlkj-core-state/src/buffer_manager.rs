//! Buffer manager for buffer-related features.
//!
//! Implements buffer navigation, alternate file, argument list, buffer groups,
//! MRU (most recently used) tracking, buffer-local options, and bufferline state.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

/// Buffer ID type.
pub type BufferId = u64;

/// Buffer type (matches Vim's buftype).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BufferType {
    /// Normal file buffer.
    #[default]
    Normal,
    /// Help buffer.
    Help,
    /// Quickfix list.
    Quickfix,
    /// Location list.
    Loclist,
    /// Terminal buffer.
    Terminal,
    /// Prompt buffer.
    Prompt,
    /// Scratch buffer (no file).
    Scratch,
    /// Nofile (read-only, no file).
    Nofile,
    /// Popup buffer.
    Popup,
}

/// What happens when buffer is hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BufferHidden {
    /// Use global 'hidden' option.
    #[default]
    UseGlobal,
    /// Hide even if modified.
    Hide,
    /// Unload when hidden.
    Unload,
    /// Delete when hidden.
    Delete,
    /// Wipe when hidden.
    Wipe,
}

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
    /// Can be modified.
    pub modifiable: bool,
    /// Use swap file.
    pub swapfile: bool,
}

/// Buffer-local options.
#[derive(Debug, Clone, Default)]
pub struct BufferLocalOptions {
    /// Tab stop width.
    pub tabstop: Option<u8>,
    /// Shift width.
    pub shiftwidth: Option<u8>,
    /// Expand tabs to spaces.
    pub expandtab: Option<bool>,
    /// Text width (0 = no wrap).
    pub textwidth: Option<usize>,
    /// Filetype.
    pub filetype: Option<String>,
    /// File encoding.
    pub fileencoding: Option<String>,
    /// File format (unix, dos, mac).
    pub fileformat: Option<String>,
    /// Spell checking.
    pub spell: Option<bool>,
    /// Spell language.
    pub spelllang: Option<String>,
    /// Fold method.
    pub foldmethod: Option<String>,
    /// Custom variables (b: namespace).
    pub variables: HashMap<String, String>,
}

impl BufferLocalOptions {
    /// Create new default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a variable.
    pub fn set_var(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(name.into(), value.into());
    }

    /// Get a variable.
    pub fn get_var(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    /// Check if variable exists.
    pub fn has_var(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Remove a variable.
    pub fn remove_var(&mut self, name: &str) -> Option<String> {
        self.variables.remove(name)
    }
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
    /// Buffer type.
    pub buftype: BufferType,
    /// Hidden behavior.
    pub bufhidden: BufferHidden,
    /// Line count.
    pub line_count: usize,
    /// Local options.
    pub local_options: BufferLocalOptions,
    /// Pinned in bufferline.
    pub pinned: bool,
}

impl BufferInfo {
    /// Create a new buffer info.
    pub fn new(id: BufferId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            path: None,
            state: BufferState::Active,
            flags: BufferFlags {
                listed: true,
                modifiable: true,
                swapfile: true,
                ..Default::default()
            },
            buftype: BufferType::Normal,
            bufhidden: BufferHidden::UseGlobal,
            line_count: 1,
            local_options: BufferLocalOptions::new(),
            pinned: false,
        }
    }

    /// Set path.
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    /// Set buffer type.
    pub fn with_type(mut self, buftype: BufferType) -> Self {
        self.buftype = buftype;
        self
    }

    /// Mark as scratch buffer.
    pub fn as_scratch(mut self) -> Self {
        self.buftype = BufferType::Scratch;
        self.flags.listed = false;
        self.flags.swapfile = false;
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
    /// Collapsed in bufferline.
    pub collapsed: bool,
}

impl BufferGroup {
    /// Create a new buffer group.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            buffers: HashSet::new(),
            collapsed: false,
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

    /// Toggle collapsed state.
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }
}

/// MRU (Most Recently Used) buffer tracking.
#[derive(Debug)]
pub struct MruList {
    /// Buffer IDs in MRU order (most recent first).
    list: VecDeque<BufferId>,
    /// Maximum size.
    max_size: usize,
}

impl Default for MruList {
    fn default() -> Self {
        Self::new()
    }
}

impl MruList {
    /// Create a new MRU list.
    pub fn new() -> Self {
        Self {
            list: VecDeque::new(),
            max_size: 100,
        }
    }

    /// Create with custom max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            list: VecDeque::new(),
            max_size,
        }
    }

    /// Touch a buffer (move to front).
    pub fn touch(&mut self, id: BufferId) {
        // Remove if already present.
        self.list.retain(|&i| i != id);
        // Add to front.
        self.list.push_front(id);
        // Trim if too long.
        while self.list.len() > self.max_size {
            self.list.pop_back();
        }
    }

    /// Remove a buffer.
    pub fn remove(&mut self, id: BufferId) {
        self.list.retain(|&i| i != id);
    }

    /// Get MRU list.
    pub fn list(&self) -> &VecDeque<BufferId> {
        &self.list
    }

    /// Get nth most recent buffer.
    pub fn get(&self, n: usize) -> Option<BufferId> {
        self.list.get(n).copied()
    }

    /// Get previous buffer (skip current).
    pub fn prev(&self, current: BufferId) -> Option<BufferId> {
        self.list.iter().find(|&&id| id != current).copied()
    }

    /// Cycle through MRU list.
    pub fn cycle(&self, current: BufferId, reverse: bool) -> Option<BufferId> {
        if self.list.is_empty() {
            return None;
        }
        let pos = self.list.iter().position(|&id| id == current);
        match pos {
            Some(i) => {
                let len = self.list.len();
                let new_pos = if reverse {
                    if i == 0 { len - 1 } else { i - 1 }
                } else {
                    (i + 1) % len
                };
                self.list.get(new_pos).copied()
            }
            None => self.list.front().copied(),
        }
    }
}

/// Buffer sorting mode for bufferline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BufferSortMode {
    /// Sort by buffer ID (creation order).
    #[default]
    Id,
    /// Sort by name.
    Name,
    /// Sort by directory then name.
    Directory,
    /// Sort by file extension.
    Extension,
    /// Sort by MRU.
    Mru,
}

/// Bufferline state.
#[derive(Debug, Default)]
pub struct Bufferline {
    /// Visible buffer IDs.
    visible: Vec<BufferId>,
    /// Selected buffer for operations.
    selected: Option<BufferId>,
    /// Scroll offset.
    offset: usize,
    /// Sort mode.
    sort_mode: BufferSortMode,
    /// Show buffer numbers.
    show_numbers: bool,
    /// Show icons.
    show_icons: bool,
    /// Maximum buffers before warning.
    max_buffers: usize,
}

impl Bufferline {
    /// Create new bufferline.
    pub fn new() -> Self {
        Self {
            visible: Vec::new(),
            selected: None,
            offset: 0,
            sort_mode: BufferSortMode::Id,
            show_numbers: true,
            show_icons: true,
            max_buffers: 20,
        }
    }

    /// Update visible buffers.
    pub fn update(&mut self, buffers: Vec<BufferId>) {
        self.visible = buffers;
    }

    /// Get visible buffers.
    pub fn visible(&self) -> &[BufferId] {
        &self.visible
    }

    /// Set sort mode.
    pub fn set_sort_mode(&mut self, mode: BufferSortMode) {
        self.sort_mode = mode;
    }

    /// Get sort mode.
    pub fn sort_mode(&self) -> BufferSortMode {
        self.sort_mode
    }

    /// Scroll left.
    pub fn scroll_left(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    /// Scroll right.
    pub fn scroll_right(&mut self) {
        if self.offset + 1 < self.visible.len() {
            self.offset += 1;
        }
    }

    /// Check if too many buffers.
    pub fn is_over_limit(&self) -> bool {
        self.visible.len() > self.max_buffers
    }

    /// Get selected buffer for pick mode.
    pub fn selected(&self) -> Option<BufferId> {
        self.selected
    }

    /// Set selected buffer for pick mode.
    pub fn set_selected(&mut self, id: Option<BufferId>) {
        self.selected = id;
    }

    /// Check if buffer numbers should be shown.
    pub fn show_numbers(&self) -> bool {
        self.show_numbers
    }

    /// Set whether to show buffer numbers.
    pub fn set_show_numbers(&mut self, show: bool) {
        self.show_numbers = show;
    }

    /// Check if icons should be shown.
    pub fn show_icons(&self) -> bool {
        self.show_icons
    }

    /// Set whether to show icons.
    pub fn set_show_icons(&mut self, show: bool) {
        self.show_icons = show;
    }

    /// Pick buffer by letter (returns index).
    pub fn pick_letter(letter: char) -> Option<usize> {
        const LETTERS: &str = "asdfjkl;ghqweruiopzxcvbnm";
        LETTERS.find(letter)
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
    /// MRU tracking.
    mru: MruList,
    /// Bufferline state.
    bufferline: Bufferline,
    /// Next buffer ID.
    next_id: BufferId,
    /// Pinned buffers (in order).
    pinned: Vec<BufferId>,
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
        self.mru.remove(id);
        self.pinned.retain(|&b| b != id);
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

    /// Set current buffer (for alternate and MRU tracking).
    pub fn set_current(&mut self, id: BufferId) {
        self.alternate.set_current(id);
        self.mru.touch(id);
    }

    /// Get alternate buffer.
    pub fn alternate(&self) -> Option<BufferId> {
        self.alternate.alternate()
    }

    /// Toggle to alternate buffer.
    pub fn toggle_alternate(&mut self) -> Option<BufferId> {
        self.alternate.toggle()
    }

    /// Get MRU list.
    pub fn mru(&self) -> &MruList {
        &self.mru
    }

    /// Get previous MRU buffer.
    pub fn mru_prev(&self, current: BufferId) -> Option<BufferId> {
        self.mru.prev(current)
    }

    /// Cycle through MRU buffers.
    pub fn mru_cycle(&self, current: BufferId, reverse: bool) -> Option<BufferId> {
        self.mru.cycle(current, reverse)
    }

    /// Get bufferline state.
    pub fn bufferline(&self) -> &Bufferline {
        &self.bufferline
    }

    /// Get mutable bufferline state.
    pub fn bufferline_mut(&mut self) -> &mut Bufferline {
        &mut self.bufferline
    }

    /// Update bufferline with current buffers.
    pub fn update_bufferline(&mut self) {
        let pinned: Vec<_> = self.pinned.to_vec();
        let unpinned: Vec<_> = self.order.iter()
            .copied()
            .filter(|id| {
                !self.pinned.contains(id)
                    && self.buffers.get(id).map(|b| b.flags.listed).unwrap_or(false)
            })
            .collect();

        let mut visible = pinned;
        visible.extend(unpinned);
        self.bufferline.update(visible);
    }

    /// Pin a buffer.
    pub fn pin(&mut self, id: BufferId) {
        if !self.pinned.contains(&id) {
            self.pinned.push(id);
            if let Some(info) = self.buffers.get_mut(&id) {
                info.pinned = true;
            }
        }
    }

    /// Unpin a buffer.
    pub fn unpin(&mut self, id: BufferId) {
        self.pinned.retain(|&b| b != id);
        if let Some(info) = self.buffers.get_mut(&id) {
            info.pinned = false;
        }
    }

    /// Toggle pin state.
    pub fn toggle_pin(&mut self, id: BufferId) {
        if self.pinned.contains(&id) {
            self.unpin(id);
        } else {
            self.pin(id);
        }
    }

    /// Get pinned buffers.
    pub fn pinned(&self) -> &[BufferId] {
        &self.pinned
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

    #[test]
    fn test_buffer_type() {
        let info = BufferInfo::new(1, "test").with_type(BufferType::Quickfix);
        assert_eq!(info.buftype, BufferType::Quickfix);
    }

    #[test]
    fn test_buffer_scratch() {
        let info = BufferInfo::new(1, "scratch").as_scratch();
        assert_eq!(info.buftype, BufferType::Scratch);
        assert!(!info.flags.listed);
        assert!(!info.flags.swapfile);
    }

    #[test]
    fn test_buffer_local_options() {
        let mut opts = BufferLocalOptions::new();
        opts.tabstop = Some(4);
        opts.expandtab = Some(true);
        opts.set_var("project", "myproj");

        assert_eq!(opts.tabstop, Some(4));
        assert_eq!(opts.get_var("project"), Some(&"myproj".to_string()));
        assert!(opts.has_var("project"));

        opts.remove_var("project");
        assert!(!opts.has_var("project"));
    }

    #[test]
    fn test_mru_list() {
        let mut mru = MruList::new();
        mru.touch(1);
        mru.touch(2);
        mru.touch(3);

        assert_eq!(mru.get(0), Some(3));
        assert_eq!(mru.get(1), Some(2));
        assert_eq!(mru.get(2), Some(1));

        // Touch 1 again moves it to front.
        mru.touch(1);
        assert_eq!(mru.get(0), Some(1));
        assert_eq!(mru.get(1), Some(3));
    }

    #[test]
    fn test_mru_prev() {
        let mut mru = MruList::new();
        mru.touch(1);
        mru.touch(2);
        mru.touch(3);

        // Most recent is 3, prev should skip 3.
        assert_eq!(mru.prev(3), Some(2));
        assert_eq!(mru.prev(2), Some(3));
    }

    #[test]
    fn test_mru_cycle() {
        let mut mru = MruList::new();
        mru.touch(1);
        mru.touch(2);
        mru.touch(3);

        // Cycle forward from 3 -> 2 -> 1 -> 3.
        assert_eq!(mru.cycle(3, false), Some(2));
        assert_eq!(mru.cycle(2, false), Some(1));
        assert_eq!(mru.cycle(1, false), Some(3));

        // Cycle backward.
        assert_eq!(mru.cycle(3, true), Some(1));
        assert_eq!(mru.cycle(1, true), Some(2));
    }

    #[test]
    fn test_bufferline_pick_letter() {
        assert_eq!(Bufferline::pick_letter('a'), Some(0));
        assert_eq!(Bufferline::pick_letter('s'), Some(1));
        assert_eq!(Bufferline::pick_letter('d'), Some(2));
    }

    #[test]
    fn test_bufferline_scroll() {
        let mut bl = Bufferline::new();
        bl.update(vec![1, 2, 3, 4, 5]);

        assert_eq!(bl.visible(), &[1, 2, 3, 4, 5]);

        bl.scroll_right();
        // offset is internal, just check it doesn't panic.
        bl.scroll_left();
    }

    #[test]
    fn test_buffer_manager_mru() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        let id2 = manager.create("buf2");
        let id3 = manager.create("buf3");

        manager.set_current(id1);
        manager.set_current(id2);
        manager.set_current(id3);

        assert_eq!(manager.mru().get(0), Some(id3));
        assert_eq!(manager.mru_prev(id3), Some(id2));
    }

    #[test]
    fn test_buffer_manager_pin() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        let _id2 = manager.create("buf2");

        manager.pin(id1);
        assert!(manager.pinned().contains(&id1));
        assert!(manager.get(id1).unwrap().pinned);

        manager.toggle_pin(id1);
        assert!(!manager.pinned().contains(&id1));
    }

    #[test]
    fn test_buffer_manager_update_bufferline() {
        let mut manager = BufferManager::new();
        let _id1 = manager.create("buf1");
        let id2 = manager.create("buf2");
        let _id3 = manager.create("buf3");

        manager.pin(id2);
        manager.update_bufferline();

        let visible = manager.bufferline().visible();
        // Pinned buffer should be first.
        assert_eq!(visible[0], id2);
    }

    #[test]
    fn test_buffer_group_collapsed() {
        let mut group = BufferGroup::new("test");
        assert!(!group.collapsed);
        group.toggle_collapsed();
        assert!(group.collapsed);
    }

    #[test]
    fn test_buffer_remove_cleans_up() {
        let mut manager = BufferManager::new();
        let id1 = manager.create("buf1");
        manager.set_current(id1);
        manager.pin(id1);

        manager.remove(id1);
        assert!(!manager.pinned().contains(&id1));
        assert!(manager.get(id1).is_none());
    }
}
