//! Filesystem IO/watch service.

mod encoding;
mod read_write;
mod watcher;

pub use encoding::detect_encoding;
pub use read_write::FsService;
pub use watcher::FileWatcher;
