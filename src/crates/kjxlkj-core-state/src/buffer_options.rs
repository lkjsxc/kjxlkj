//! Buffer-local options and buffer groups.
//!
//! Per /docs/spec/features/buffer/buffer-local-options.md
//! and /docs/spec/features/buffer/buffer-groups.md.

use std::collections::HashMap;

/// Buffer-local options that override global settings.
#[derive(Debug, Clone, Default)]
pub struct BufferLocalOptions {
    /// Tab size override.
    pub tabstop: Option<u16>,
    /// Shift width override.
    pub shiftwidth: Option<u16>,
    /// Expand tab override.
    pub expandtab: Option<bool>,
    /// Text width for auto-wrap.
    pub textwidth: Option<u16>,
    /// File format override.
    pub fileformat: Option<String>,
    /// File encoding override.
    pub fileencoding: Option<String>,
    /// Spell checking override.
    pub spell: Option<bool>,
    /// Wrap override.
    pub wrap: Option<bool>,
    /// Filetype.
    pub filetype: Option<String>,
    /// Custom options map.
    pub custom: HashMap<String, String>,
}

impl BufferLocalOptions {
    /// Create empty options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get effective tabstop (with global default).
    pub fn effective_tabstop(
        &self,
        global: u16,
    ) -> u16 {
        self.tabstop.unwrap_or(global)
    }

    /// Get effective shiftwidth.
    pub fn effective_shiftwidth(
        &self,
        global: u16,
    ) -> u16 {
        self.shiftwidth.unwrap_or(global)
    }

    /// Get effective expandtab.
    pub fn effective_expandtab(
        &self,
        global: bool,
    ) -> bool {
        self.expandtab.unwrap_or(global)
    }
}

/// A buffer group for organizing buffers.
#[derive(Debug, Clone)]
pub struct BufferGroup {
    /// Group name.
    pub name: String,
    /// Buffer IDs in this group.
    pub buffers: Vec<kjxlkj_core_types::BufferId>,
}

/// Buffer group registry.
#[derive(Debug, Clone, Default)]
pub struct BufferGroupRegistry {
    /// Named groups.
    pub groups: Vec<BufferGroup>,
}

impl BufferGroupRegistry {
    /// Create empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create or get a group.
    pub fn get_or_create(
        &mut self,
        name: &str,
    ) -> &mut BufferGroup {
        if let Some(idx) = self
            .groups
            .iter()
            .position(|g| g.name == name)
        {
            &mut self.groups[idx]
        } else {
            self.groups.push(BufferGroup {
                name: name.to_string(),
                buffers: Vec::new(),
            });
            self.groups.last_mut().unwrap()
        }
    }

    /// Add buffer to group.
    pub fn add_to_group(
        &mut self,
        name: &str,
        buf: kjxlkj_core_types::BufferId,
    ) {
        let group = self.get_or_create(name);
        if !group.buffers.contains(&buf) {
            group.buffers.push(buf);
        }
    }

    /// Remove buffer from group.
    pub fn remove_from_group(
        &mut self,
        name: &str,
        buf: kjxlkj_core_types::BufferId,
    ) {
        if let Some(group) = self
            .groups
            .iter_mut()
            .find(|g| g.name == name)
        {
            group.buffers.retain(|&b| b != buf);
        }
    }
}

/// Argument list for `:args`.
#[derive(Debug, Clone, Default)]
pub struct ArgList {
    /// Files in the argument list.
    pub files: Vec<String>,
    /// Current position.
    pub current: usize,
}

impl ArgList {
    /// Create new empty arglist.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set files from command line args.
    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = files;
        self.current = 0;
    }

    /// Get current file.
    pub fn current_file(&self) -> Option<&str> {
        self.files.get(self.current).map(|s| s.as_str())
    }

    /// Next file.
    pub fn next(&mut self) -> Option<&str> {
        if self.current + 1 < self.files.len() {
            self.current += 1;
            self.current_file()
        } else {
            None
        }
    }

    /// Previous file.
    pub fn prev(&mut self) -> Option<&str> {
        if self.current > 0 {
            self.current -= 1;
            self.current_file()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_local_effective() {
        let opts = BufferLocalOptions {
            tabstop: Some(2),
            ..Default::default()
        };
        assert_eq!(opts.effective_tabstop(4), 2);
        assert_eq!(opts.effective_shiftwidth(4), 4);
    }

    #[test]
    fn arglist_navigation() {
        let mut al = ArgList::new();
        al.set_files(vec![
            "a.rs".into(),
            "b.rs".into(),
            "c.rs".into(),
        ]);
        assert_eq!(al.current_file(), Some("a.rs"));
        al.next();
        assert_eq!(al.current_file(), Some("b.rs"));
        al.prev();
        assert_eq!(al.current_file(), Some("a.rs"));
    }

    #[test]
    fn buffer_groups() {
        let mut reg = BufferGroupRegistry::new();
        let bid = kjxlkj_core_types::BufferId(1);
        reg.add_to_group("test", bid);
        let group = reg.get_or_create("test");
        assert_eq!(group.buffers.len(), 1);
    }
}
