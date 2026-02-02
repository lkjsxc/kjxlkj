//! Indexing service for kjxlkj editor.
//!
//! This crate provides file indexing and search.

mod finder;
mod index;
mod service;

pub use finder::Finder;
pub use index::Index;
pub use service::IndexService;
