//! File explorer.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FileNode {
    File {
        name: String,
        path: PathBuf,
    },
    Directory {
        name: String,
        path: PathBuf,
        children: Vec<FileNode>,
        expanded: bool,
    },
}

impl FileNode {
    pub fn name(&self) -> &str {
        match self {
            Self::File { name, .. } => name,
            Self::Directory { name, .. } => name,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            Self::File { path, .. } => path,
            Self::Directory { path, .. } => path,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Directory { .. })
    }

    pub fn toggle_expand(&mut self) {
        if let Self::Directory { expanded, .. } = self {
            *expanded = !*expanded;
        }
    }
}

#[derive(Debug)]
pub struct FileExplorer {
    pub root: PathBuf,
    pub tree: Vec<FileNode>,
    pub selected: usize,
    pub visible: bool,
    pub width: u16,
}

impl FileExplorer {
    pub fn new(root: PathBuf) -> Self {
        let tree = Self::scan_dir(&root, 0);
        Self {
            root,
            tree,
            selected: 0,
            visible: false,
            width: 30,
        }
    }

    pub fn scan_dir(path: &Path, depth: u32) -> Vec<FileNode> {
        if depth > 3 {
            return Vec::new();
        }
        let entries = match std::fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };
        let mut nodes: Vec<FileNode> = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let file_path = entry.path();
            if file_path.is_dir() {
                let children = if depth < 1 {
                    Self::scan_dir(&file_path, depth + 1)
                } else {
                    Vec::new()
                };
                nodes.push(FileNode::Directory {
                    name,
                    path: file_path,
                    children,
                    expanded: depth == 0,
                });
            } else {
                nodes.push(FileNode::File {
                    name,
                    path: file_path,
                });
            }
        }
        nodes.sort_by(|a, b| {
            let da = a.is_dir() as u8;
            let db = b.is_dir() as u8;
            db.cmp(&da).then(a.name().cmp(b.name()))
        });
        nodes
    }

    pub fn visible_count(&self) -> usize {
        fn count(nodes: &[FileNode]) -> usize {
            let mut c = 0;
            for n in nodes {
                c += 1;
                if let FileNode::Directory {
                    children,
                    expanded: true,
                    ..
                } = n
                {
                    c += count(children);
                }
            }
            c
        }
        count(&self.tree)
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let max = self.visible_count();
        if self.selected + 1 < max {
            self.selected += 1;
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn refresh(&mut self) {
        self.tree = Self::scan_dir(&self.root, 0);
        self.selected = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explorer_creation() {
        let exp = FileExplorer::new(PathBuf::from("/tmp"));
        assert!(!exp.visible);
        assert_eq!(exp.width, 30);
    }

    #[test]
    fn move_selection() {
        let mut exp = FileExplorer::new(PathBuf::from("/tmp"));
        exp.move_down();
        exp.move_up();
        assert_eq!(exp.selected, 0);
    }
}
