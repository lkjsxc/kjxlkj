//! kjxlkj-service-fs - File system service.
//!
//! This crate provides file watching and async file operations.

#![allow(dead_code)]

mod events;
mod watcher;

pub use events::{FsEvent, FsEventKind};
pub use watcher::FileWatcher;
