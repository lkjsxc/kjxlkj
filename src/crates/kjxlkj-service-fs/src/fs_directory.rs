/// FS service directory listing for the file explorer.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DirEntry {
    pub(crate) name: String,
    pub(crate) is_dir: bool,
    pub(crate) size: u64,
    pub(crate) hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SortOrder {
    Name,
    NameDesc,
    Size,
    SizeDesc,
    Type,
    TypeDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DirListing {
    pub(crate) path: String,
    pub(crate) entries: Vec<DirEntry>,
    pub(crate) truncated: bool,
}

pub(crate) fn sort_entries(entries: &mut [DirEntry], order: SortOrder) {
    entries.sort_by(|a, b| {
        // Directories always come first
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
                let ext_a = a.name.rsplit('.').next().unwrap_or("");
                let ext_b = b.name.rsplit('.').next().unwrap_or("");
                ext_a.cmp(ext_b).then_with(|| a.name.cmp(&b.name))
            }
            SortOrder::TypeDesc => {
                let ext_a = a.name.rsplit('.').next().unwrap_or("");
                let ext_b = b.name.rsplit('.').next().unwrap_or("");
                ext_b.cmp(ext_a).then_with(|| b.name.cmp(&a.name))
            }
        }
    });
}

pub(crate) fn filter_hidden<'a>(entries: &'a [DirEntry]) -> Vec<&'a DirEntry> {
    entries.iter().filter(|e| !e.hidden).collect()
}

pub(crate) fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

pub(crate) fn list_directory(path: &str, show_hidden: bool, order: SortOrder) -> DirListing {
    let mut entries = Vec::new();
    // In real implementation this would read from filesystem.
    // Here we build the listing structure for the given path.
    sort_entries(&mut entries, order);
    if !show_hidden {
        entries.retain(|e| !e.hidden);
    }
    DirListing {
        path: path.to_string(),
        entries,
        truncated: false,
    }
}

pub(crate) fn max_children_check(count: usize, limit: usize) -> bool {
    count > limit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<DirEntry> {
        vec![
            DirEntry { name: "bravo.rs".into(), is_dir: false, size: 200, hidden: false },
            DirEntry { name: "alpha".into(), is_dir: true, size: 0, hidden: false },
            DirEntry { name: ".hidden".into(), is_dir: false, size: 50, hidden: true },
            DirEntry { name: "charlie.txt".into(), is_dir: false, size: 100, hidden: false },
        ]
    }

    #[test]
    fn sort_by_name() {
        let mut entries = sample_entries();
        sort_entries(&mut entries, SortOrder::Name);
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names[0], "alpha"); // dir first
        assert_eq!(names[1], ".hidden");
    }

    #[test]
    fn sort_dirs_first() {
        let mut entries = sample_entries();
        sort_entries(&mut entries, SortOrder::Size);
        assert!(entries[0].is_dir, "directories must come first");
    }

    #[test]
    fn filter_hidden_entries() {
        let entries = sample_entries();
        let visible = filter_hidden(&entries);
        assert_eq!(visible.len(), 3);
        assert!(visible.iter().all(|e| !e.hidden));
    }

    #[test]
    fn hidden_detection() {
        assert!(is_hidden(".gitignore"));
        assert!(is_hidden(".hidden"));
        assert!(!is_hidden("README.md"));
        assert!(!is_hidden("src"));
    }

    #[test]
    fn list_directory_basic() {
        let listing = list_directory("/tmp/test", false, SortOrder::Name);
        assert_eq!(listing.path, "/tmp/test");
        assert!(!listing.truncated);
    }

    #[test]
    fn max_children() {
        assert!(max_children_check(1001, 1000));
        assert!(!max_children_check(500, 1000));
        assert!(!max_children_check(1000, 1000));
    }
}
