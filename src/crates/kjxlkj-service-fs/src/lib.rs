//! Filesystem service for kjxlkj editor.
//!
//! This crate handles file IO and watching.

mod ops;
mod service;
mod watcher;

pub use ops::{read_file, write_file};
pub use service::FsService;
pub use watcher::FsWatcher;
