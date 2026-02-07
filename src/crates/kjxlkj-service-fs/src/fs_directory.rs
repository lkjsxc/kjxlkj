//! Directory listing and sorting.

use serde::{Deserialize, Serialize};

/// A directory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub hidden: bool,
}

/// Sort order for directory entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Name,
    NameDesc,
    Size,
    SizeDesc,
    Type,
    TypeDesc,
}

/// A directory listing result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirListing {
    pub path: String,
    pub entries: Vec<DirEntry>,
    pub truncated: bool,
}

/// Create a directory listing from in-memory data.
pub fn list_directory(path: &str, entries: Vec<DirEntry>) -> DirListing {
    DirListing {
        path: path.to_string(),
        entries,
        truncated: false,
    }
}

/// Sort entries with directories first, then by the given order.
pub fn sort_entries(entries: &mut Vec<DirEntry>, order: SortOrder) {
    entries.sort_by(|a, b| {
        // Directories always come first.
        let dir_cmp = b.is_dir.cmp(&a.is_dir);
        if dir_cmp != std::cmp::Ordering::Equal {
            return dir_cmp;
        }
        match order {
            SortOrder::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortOrder::NameDesc => b.name.to_lowercase().cmp(&a.name.to_lowercase()),
            SortOrder::Size => a.size.cmp(&b.size),
            SortOrder::SizeDesc => b.size.cmp(&a.size),
            SortOrder::Type => {
                let ext_a = extension(&a.name);
                let ext_b = extension(&b.name);
                ext_a.cmp(&ext_b)
            }
            SortOrder::TypeDesc => {
                let ext_a = extension(&a.name);
                let ext_b = extension(&b.name);
                ext_b.cmp(&ext_a)
            }
        }
    });
}

fn extension(name: &str) -> String {
    name.rsplit_once('.')
        .map(|(_, e)| e.to_lowercase())
        .unwrap_or_default()
}

/// Filter out hidden entries.
pub fn filter_hidden(entries: &[DirEntry]) -> Vec<DirEntry> {
    entries.iter().filter(|e| !e.hidden).cloned().collect()
}

/// Check if a filename is hidden (starts with '.').
pub fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_dirs_first() {
        let mut entries = vec![
            DirEntry { name: "file.txt".into(), is_dir: false, size: 100, hidden: false },
            DirEntry { name: "src".into(), is_dir: true, size: 0, hidden: false },
        ];
        sort_entries(&mut entries, SortOrder::Name);
        assert!(entries[0].is_dir);
    }

    #[test]
    fn filter_hidden_works() {
        let entries = vec![
            DirEntry { name: ".git".into(), is_dir: true, size: 0, hidden: true },
            DirEntry { name: "main.rs".into(), is_dir: false, size: 50, hidden: false },
        ];
        let visible = filter_hidden(&entries);
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].name, "main.rs");
    }

    #[test]
    fn is_hidden_check() {
        assert!(is_hidden(".gitignore"));
        assert!(!is_hidden("Cargo.toml"));
    }
}
