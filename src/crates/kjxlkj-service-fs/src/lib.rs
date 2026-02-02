//! kjxlkj-service-fs - File system service.
//!
//! This crate provides file watching and async file operations.

#![allow(dead_code)]

mod watcher;
mod events;

pub use watcher::FileWatcher;
pub use events::{FsEvent, FsEventKind};
