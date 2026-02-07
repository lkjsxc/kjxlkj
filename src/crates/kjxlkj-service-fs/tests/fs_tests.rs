use kjxlkj_service_fs::{
    filter_hidden, is_hidden, list_directory, sort_entries, DirEntry, DirListing, SortOrder,
};
use kjxlkj_service_fs::{FsEvent, FsWatcher};

fn sample_entries() -> Vec<DirEntry> {
    vec![
        DirEntry { name: "src".into(), is_dir: true, size: 0, hidden: false },
        DirEntry { name: "main.rs".into(), is_dir: false, size: 200, hidden: false },
        DirEntry { name: "lib.rs".into(), is_dir: false, size: 100, hidden: false },
        DirEntry { name: ".gitignore".into(), is_dir: false, size: 50, hidden: true },
        DirEntry { name: "tests".into(), is_dir: true, size: 0, hidden: false },
    ]
}

// --- Directory listing ---

#[test]
fn list_directory_creation() {
    let listing = list_directory("/tmp/project", sample_entries());
    assert_eq!(listing.path, "/tmp/project");
    assert_eq!(listing.entries.len(), 5);
    assert!(!listing.truncated);
}

#[test]
fn sort_by_name_dirs_first() {
    let mut entries = sample_entries();
    sort_entries(&mut entries, SortOrder::Name);
    assert!(entries[0].is_dir);
    assert!(entries[1].is_dir);
}

#[test]
fn sort_by_size() {
    let mut entries = vec![
        DirEntry { name: "big.txt".into(), is_dir: false, size: 500, hidden: false },
        DirEntry { name: "small.txt".into(), is_dir: false, size: 10, hidden: false },
    ];
    sort_entries(&mut entries, SortOrder::Size);
    assert_eq!(entries[0].name, "small.txt");
}

#[test]
fn sort_by_size_desc() {
    let mut entries = vec![
        DirEntry { name: "small.txt".into(), is_dir: false, size: 10, hidden: false },
        DirEntry { name: "big.txt".into(), is_dir: false, size: 500, hidden: false },
    ];
    sort_entries(&mut entries, SortOrder::SizeDesc);
    assert_eq!(entries[0].name, "big.txt");
}

#[test]
fn sort_by_type() {
    let mut entries = vec![
        DirEntry { name: "main.rs".into(), is_dir: false, size: 0, hidden: false },
        DirEntry { name: "lib.py".into(), is_dir: false, size: 0, hidden: false },
    ];
    sort_entries(&mut entries, SortOrder::Type);
    // "py" < "rs"
    assert_eq!(entries[0].name, "lib.py");
}

// --- Hidden file filtering ---

#[test]
fn filter_hidden_removes_hidden() {
    let entries = sample_entries();
    let visible = filter_hidden(&entries);
    assert_eq!(visible.len(), 4);
    assert!(visible.iter().all(|e| !e.hidden));
}

#[test]
fn is_hidden_dotfile() {
    assert!(is_hidden(".gitignore"));
    assert!(is_hidden(".env"));
}

#[test]
fn is_hidden_normal_file() {
    assert!(!is_hidden("Cargo.toml"));
    assert!(!is_hidden("README.md"));
}

// --- File watch events ---

#[test]
fn fs_event_created() {
    let e = FsEvent::Created("new.rs".into());
    assert_eq!(e, FsEvent::Created("new.rs".into()));
}

#[test]
fn fs_event_modified() {
    let e = FsEvent::Modified("main.rs".into());
    assert_eq!(e, FsEvent::Modified("main.rs".into()));
}

#[test]
fn fs_event_deleted() {
    let e = FsEvent::Deleted("old.rs".into());
    assert_ne!(e, FsEvent::Created("old.rs".into()));
}

#[test]
fn fs_event_renamed() {
    let e = FsEvent::Renamed("a.rs".into(), "b.rs".into());
    assert_eq!(e, FsEvent::Renamed("a.rs".into(), "b.rs".into()));
}

// --- FsWatcher ---

#[test]
fn watcher_add_remove() {
    let mut w = FsWatcher::new();
    w.add_watch("/tmp/a");
    w.add_watch("/tmp/b");
    assert_eq!(w.watched().len(), 2);
    w.remove_watch("/tmp/a");
    assert_eq!(w.watched().len(), 1);
}

#[test]
fn watcher_no_duplicates() {
    let mut w = FsWatcher::new();
    w.add_watch("/x");
    w.add_watch("/x");
    assert_eq!(w.watched().len(), 1);
}
