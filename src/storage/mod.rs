//! Storage layer for records

mod filesystem;
mod traits;

pub use filesystem::FilesystemStorage;
pub use traits::{Storage, StorageError};
