//! kjxlkj-service-fs: File system operations, directory listing, and watch events.

pub mod fs_directory;
pub mod fs_ops;
pub mod fs_watch;

pub use fs_directory::{
    filter_hidden, is_hidden, list_directory, sort_entries, DirEntry, DirListing, SortOrder,
};
pub use fs_ops::{detect_encoding, detect_line_ending, file_exists, read_file, write_file};
pub use fs_watch::{FsEvent, FsWatcher};
