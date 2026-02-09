//! Buffer-local options and buffer groups.

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct BufferLocalOptions {
    pub tabstop: Option<u16>,
    pub shiftwidth: Option<u16>,
    pub expandtab: Option<bool>,
    pub textwidth: Option<u16>,
    pub fileformat: Option<String>,
    pub fileencoding: Option<String>,
    pub spell: Option<bool>,
    pub wrap: Option<bool>,
    pub filetype: Option<String>,
    pub custom: HashMap<String, String>,
}

impl BufferLocalOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn effective_tabstop(&self, global: u16) -> u16 {
        self.tabstop.unwrap_or(global)
    }

    pub fn effective_shiftwidth(&self, global: u16) -> u16 {
        self.shiftwidth.unwrap_or(global)
    }

    pub fn effective_expandtab(&self, global: bool) -> bool {
        self.expandtab.unwrap_or(global)
    }
}

#[derive(Debug, Clone)]
pub struct BufferGroup {
    pub name: String,
    pub buffers: Vec<kjxlkj_core_types::BufferId>,
}

#[derive(Debug, Clone, Default)]
pub struct BufferGroupRegistry {
    pub groups: Vec<BufferGroup>,
}

impl BufferGroupRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_create(&mut self, name: &str) -> &mut BufferGroup {
        if let Some(idx) = self.groups.iter().position(|g| g.name == name) {
            &mut self.groups[idx]
        } else {
            self.groups.push(BufferGroup {
                name: name.to_string(),
                buffers: Vec::new(),
            });
            self.groups.last_mut().unwrap()
        }
    }

    pub fn add_to_group(&mut self, name: &str, buf: kjxlkj_core_types::BufferId) {
        let group = self.get_or_create(name);
        if !group.buffers.contains(&buf) {
            group.buffers.push(buf);
        }
    }

    pub fn remove_from_group(&mut self, name: &str, buf: kjxlkj_core_types::BufferId) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.name == name) {
            group.buffers.retain(|&b| b != buf);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArgList {
    pub files: Vec<String>,
    pub current: usize,
}

impl ArgList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = files;
        self.current = 0;
    }

    pub fn current_file(&self) -> Option<&str> {
        self.files.get(self.current).map(|s| s.as_str())
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.current + 1 < self.files.len() {
            self.current += 1;
            self.current_file()
        } else {
            None
        }
    }

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
        al.set_files(vec!["a.rs".into(), "b.rs".into(), "c.rs".into()]);
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
